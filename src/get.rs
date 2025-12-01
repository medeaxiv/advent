use std::{
    env, io,
    path::{Path, PathBuf},
    sync::Mutex,
};

const ADVENT_OF_CODE_URI: &str = "https://adventofcode.com";
const SESSION_VAR: &str = "ADVENT_SESSION";
const INPUT_DIR_VAR: &str = "ADVENT_INPUT_DIR";

#[derive(clap::Args)]
/// Fetch your input files
pub struct GetCli {
    #[arg(long)]
    /// Print the input to standard output
    print: bool,
    #[arg()]
    year: u32,
    #[arg()]
    day: u32,
}

pub fn run_command(cli: GetCli) -> anyhow::Result<()> {
    let input = fetch_input(cli.year, cli.day)?;

    if cli.print {
        println!("{input}");
    }

    Ok(())
}

pub fn get_input(year: u32, day: u32) -> Result<String, GetError> {
    if let Ok(contents) = read_input(year, day) {
        return Ok(contents);
    }

    fetch_input(year, day)
}

pub fn read_input(year: u32, day: u32) -> Result<String, GetError> {
    let path = input_file_path(year, day)?;
    let contents = std::fs::read_to_string(path)?;
    Ok(contents)
}

pub fn fetch_input(year: u32, day: u32) -> Result<String, GetError> {
    let path = input_file_path(year, day)?;
    let url = input_file_url(year, day);

    // Ensure input directory exists
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }

    // Fetch input file
    let client = client()?;
    let mut response = client.get(url).call()?;
    let contents = response.body_mut().read_to_string()?;

    // Save input file locally
    std::fs::write(&path, contents.as_bytes())?;

    Ok(contents)
}

fn input_file_path(year: u32, day: u32) -> Result<PathBuf, env::VarError> {
    let dir = env::var(INPUT_DIR_VAR)?;
    let file = format!("{year}-{day}.txt");
    let path = Path::new(&dir).join(&file);
    Ok(path)
}

fn input_file_url(year: u32, day: u32) -> String {
    format!("{ADVENT_OF_CODE_URI}/{year}/day/{day}/input")
}

fn client() -> Result<ureq::Agent, GetError> {
    static AGENT: Mutex<Option<ureq::Agent>> = Mutex::new(None);

    let mut lock = AGENT.lock().unwrap();
    if let Some(agent) = lock.as_ref() {
        return Ok(agent.clone());
    }

    let agent: ureq::Agent = ureq::Agent::config_builder()
        .https_only(true)
        .accept("text/plain")
        .build()
        .into();

    {
        let uri = ureq::http::Uri::from_static(ADVENT_OF_CODE_URI);
        let session = env::var(SESSION_VAR)?;
        let cookie = format!("session={session}");
        let cookie = ureq::Cookie::parse(cookie, &uri)?;
        let mut cookie_jar = agent.cookie_jar_lock();
        cookie_jar.insert(cookie, &uri)?;
    }

    *lock = Some(agent.clone());
    Ok(agent)
}

#[derive(thiserror::Error, Debug)]
pub enum GetError {
    #[error(transparent)]
    Env(#[from] env::VarError),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Http(#[from] ureq::Error),
}
