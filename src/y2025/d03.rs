use std::num::NonZeroU64;

use crate::solution::Solution;

pub fn solution() -> Solution {
    Solution::new().with_a(a).with_b(b)
}

fn parse(line: &str) -> anyhow::Result<Vec<u8>> {
    line.chars()
        .map(|c| char::to_digit(c, 10).map(|c| c as u8))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| anyhow::anyhow!("invalid input"))
}

fn a(input: &str) -> anyhow::Result<u64> {
    let joltage = input
        .lines()
        .map(parse)
        .try_fold::<_, _, anyhow::Result<_>>(0, |joltage, bank| {
            let joltage = joltage + bank_joltage(&bank?, 2);
            Ok(joltage)
        })?;

    Ok(joltage)
}

fn b(input: &str) -> anyhow::Result<u64> {
    let joltage = input
        .lines()
        .map(parse)
        .try_fold::<_, _, anyhow::Result<_>>(0, |joltage, bank| {
            let joltage = joltage + bank_joltage(&bank?, 12);
            Ok(joltage)
        })?;

    Ok(joltage)
}

fn bank_joltage(bank: &[u8], n: u32) -> u64 {
    struct Ctx<'a> {
        bank: &'a [u8],
        cache: Vec<Option<NonZeroU64>>,
        limit: u32,
    }

    impl<'a> Ctx<'a> {
        fn new(bank: &'a [u8], limit: u32) -> Self {
            let cache_len = bank.len() * limit as usize;
            let cache = vec![None; cache_len];
            Self { bank, cache, limit }
        }

        fn idx(&self, index: usize, exp: u32) -> usize {
            (self.bank.len() * exp as usize) + index
        }

        fn get(&self, index: usize, exp: u32) -> Option<NonZeroU64> {
            let cache_index = self.idx(index, exp);
            self.cache[cache_index]
        }

        fn insert(&mut self, index: usize, exp: u32, value: u64) {
            let cache_index = self.idx(index, exp);
            self.cache[cache_index] = NonZeroU64::new(value);
        }
    }

    fn inner(ctx: &mut Ctx, index: usize, exp: u32) -> u64 {
        if exp >= ctx.limit {
            return 0;
        }

        if let Some(cached) = ctx.get(index, exp) {
            return cached.get();
        }

        let value = ctx.bank[index] as u64 * 10u64.strict_pow(exp);
        let mut max = 0;
        for next_index in (0..index).rev() {
            let msd = inner(ctx, next_index, exp + 1);
            max = max.max(msd);
        }

        let result = value + max;
        ctx.insert(index, exp, result);
        result
    }

    let mut ctx = Ctx::new(bank, n);
    let mut max = 0;
    for i in (0..bank.len()).rev() {
        let value = inner(&mut ctx, i, 0);
        max = max.max(value);
    }

    max
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case("987654321111111", 2, 98)]
    #[case("811111111111119", 2, 89)]
    #[case("234234234234278", 2, 78)]
    #[case("818181911112111", 2, 92)]
    #[case("987654321111111", 12, 987654321111)]
    #[case("811111111111119", 12, 811111111119)]
    #[case("234234234234278", 12, 434234234278)]
    #[case("818181911112111", 12, 888911112111)]
    fn test_bank_joltage(#[case] bank: &str, #[case] n: u32, #[case] expected: u64) {
        let bank = super::parse(bank).unwrap();
        let result = super::bank_joltage(&bank, n);
        assert_eq!(result, expected);
    }
}
