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
    fn window_max(window: &[u8]) -> (u8, usize) {
        let mut max = 0;
        let mut idx = 0;
        for (i, &value) in window.iter().enumerate() {
            if value > max {
                max = value;
                idx = i;
            }
        }

        (max, idx)
    }

    let n = n - 1;
    let mut window_start = 0;
    let mut window_len = bank.len() - n as usize;
    let mut position = 10u64.strict_pow(n);
    let mut joltage = 0;
    while position != 0 {
        let window_end = window_start + window_len;
        let window = &bank[window_start..window_end];
        let (digit, idx) = window_max(window);

        joltage += digit as u64 * position;
        position /= 10;
        window_start += 1 + idx;
        window_len -= idx;
    }

    joltage
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
