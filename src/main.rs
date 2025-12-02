mod get;
mod solution;
mod solve;
mod util;
mod y2025;

fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    trace();
    let cli: Cli = clap::Parser::parse();

    match cli.command {
        Command::Get(get) => crate::get::run_command(get),
        Command::Solve(solve) => crate::solve::run_command(solve),
    }
}

fn trace() {
    use tracing_subscriber::{EnvFilter, prelude::*};

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Get(crate::get::GetCli),
    Solve(crate::solve::SolveCli),
}
