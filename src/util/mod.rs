#![allow(dead_code)]

pub mod bitmap;
pub mod char;
pub mod grid;
pub mod output;
pub mod style;
pub mod vecset;
pub mod write;

pub fn min_max<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if b < a { (b, a) } else { (a, b) }
}

pub fn time<F, O>(f: F) -> impl Fn(&str) -> anyhow::Result<O> + Send + Sync + 'static
where
    F: Fn(&str) -> anyhow::Result<O> + Send + Sync + 'static,
    O: self::output::Output,
{
    use std::time::Instant;

    move |input| {
        let start = Instant::now();
        let result = f(input)?;
        let time = start.elapsed();
        tracing::info!(?time);
        Ok(result)
    }
}
