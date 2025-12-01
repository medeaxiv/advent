use std::str::FromStr;

use crate::solution::Solutions;

#[derive(clap::Args)]
/// Run solutions on your input files
pub struct SolveCli {
    #[arg()]
    year: SolveYear,
    #[arg()]
    day: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SolveYear {
    All,
    Year(u32),
}

impl FromStr for SolveYear {
    type Err = ParseSolveYearError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let year = s.parse();
        match year {
            Ok(year) => Ok(Self::Year(year)),
            Err(_) if s == "all" => Ok(Self::All),
            Err(_) => Err(ParseSolveYearError),
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("must be \"all\" or a positive integer")]
struct ParseSolveYearError;

pub fn run_command(cli: SolveCli) -> anyhow::Result<()> {
    let solutions = Solutions::default();

    match (cli.year, cli.day) {
        (SolveYear::All, _) => solutions.run_all(),
        (SolveYear::Year(year), Some(day)) => solutions.run_day(year, day),
        (SolveYear::Year(year), None) => solutions.run_year(year),
    }
}
