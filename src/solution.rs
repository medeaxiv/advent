use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

use crate::util::{output::Output, vecset::SortedVecSet};

type RunEntry<'a> = ((u32, u32), &'a Solution);

pub struct Solutions {
    entries: BTreeMap<(u32, u32), Solution>,
    years: BTreeMap<u32, SortedVecSet<u32>>,
}

impl Default for Solutions {
    fn default() -> Self {
        Self::new()
    }
}

impl Solutions {
    pub fn new() -> Self {
        let mut solutions = Self::empty();
        crate::y2025::register(&mut solutions);
        solutions
    }

    pub fn empty() -> Self {
        Self {
            entries: BTreeMap::default(),
            years: BTreeMap::default(),
        }
    }

    pub fn register(&mut self, year: u32, day: u32, solution: Solution) {
        self.entries.insert((year, day), solution);
        let days = self
            .years
            .entry(year)
            .or_insert_with(|| SortedVecSet::with_capacity(25));
        days.insert(day);
    }

    pub fn run_all(&self) -> anyhow::Result<()> {
        let entries = self.entries.iter().map(|(k, v)| (*k, v));
        Self::run_entries(entries)
    }

    pub fn run_year(&self, year: u32) -> anyhow::Result<()> {
        let days = self
            .years
            .get(&year)
            .ok_or_else(|| anyhow::anyhow!("No solutions registed for year {}", year))?
            .as_slice();

        let entries = days.iter().flat_map(|day| {
            let k = (year, *day);
            self.entries.get(&k).map(|v| (k, v))
        });

        Self::run_entries(entries)
    }

    pub fn run_day(&self, year: u32, day: u32) -> anyhow::Result<()> {
        let key = (year, day);
        let solution = self.entries.get(&key).ok_or_else(|| {
            anyhow::anyhow!("No solution registered for year {}, day {}", year, day)
        })?;

        Self::run_entries([(key, solution)])
    }

    fn run_entries<'a>(entries: impl IntoIterator<Item = RunEntry<'a>>) -> anyhow::Result<()> {
        let mut error_count = 0;

        for ((year, day), solution) in entries {
            let input = crate::get::get_input(year, day)?;
            if let Err(errors) = solution.run(year, day, &input) {
                error_count += errors.0;
            }
        }

        if error_count == 0 {
            Ok(())
        } else {
            Err(SolutionErrors(error_count).into())
        }
    }
}

#[derive(Default)]
pub struct Solution {
    a: Option<BoxedFn>,
    b: Option<BoxedFn>,
}

impl Solution {
    pub fn new() -> Self {
        Self { a: None, b: None }
    }

    pub fn with_a<A, O>(self, a: A) -> Self
    where
        A: Fn(&str) -> anyhow::Result<O> + 'static,
        O: Output + 'static,
    {
        Self {
            a: Some(wrap(a)),
            ..self
        }
    }

    pub fn with_b<B, O>(self, b: B) -> Self
    where
        B: Fn(&str) -> anyhow::Result<O> + 'static,
        O: Output + 'static,
    {
        Self {
            b: Some(wrap(b)),
            ..self
        }
    }

    fn run(&self, year: u32, day: u32, input: &str) -> Result<(), SolutionErrors> {
        let mut error_count = 0;
        if let Some(a) = self.a.as_deref() {
            let result = (a)(input);
            error_count += result.is_err() as u32;
            print!("{year}-{day} A:");
            print_solution_result(&result);
        }

        if let Some(b) = self.b.as_deref() {
            let result = (b)(input);
            error_count += result.is_err() as u32;
            print!("{year}-{day} B:");
            print_solution_result(&result);
        }

        if error_count == 0 {
            Ok(())
        } else {
            Err(SolutionErrors(error_count))
        }
    }
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
#[error("{} {} failed", .0, if *.0 > 1 {"solutions"} else {"solution"})]
struct SolutionErrors(u32);

fn print_solution_result(result: &anyhow::Result<(BoxedOutput, Duration)>) {
    use crate::util::style::{Color, ToStyled as _};

    match result {
        Ok((output, time)) if output.is_multiline() => {
            println!(" ({time:?})");
            let formatted = format!("{output}");
            for line in formatted.lines() {
                println!("  {line}");
            }
        }
        Ok((output, time)) => {
            println!(" {output} ({time:?})");
        }
        Err(error) => {
            let error = format!("{error}");
            println!(" {}", error.with_fg(Color::Red));
        }
    }
}

type BoxedOutput = Box<dyn Output>;
type InnerFn = dyn Fn(&str) -> anyhow::Result<(BoxedOutput, Duration)>;
type BoxedFn = Box<InnerFn>;

fn wrap<F, O>(f: F) -> BoxedFn
where
    F: Fn(&str) -> anyhow::Result<O> + 'static,
    O: Output + 'static,
{
    let closure = move |input: &str| {
        let start = Instant::now();
        let output = (f)(input)?;
        let time = start.elapsed();
        let boxed: Box<dyn Output> = Box::new(output);
        Ok((boxed, time))
    };

    Box::new(closure)
}
