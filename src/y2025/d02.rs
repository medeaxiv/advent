use crate::{solution::Solution, util::invalid_input};

pub fn solution() -> Solution {
    Solution::new().with_a(a).with_b(b)
}

fn parse(fragment: &str) -> anyhow::Result<(u64, u64)> {
    let (low, high) = fragment
        .trim()
        .split_once('-')
        .ok_or_else(invalid_input!())?;
    let low = low.parse()?;
    let high = high.parse()?;
    Ok((low, high))
}

fn a(input: &str) -> anyhow::Result<u64> {
    let sum = input
        .split(',')
        .map(parse)
        .try_fold::<_, _, anyhow::Result<_>>(0, |sum, range| {
            let (low, high) = range?;
            let next = sum + iter_invalid_ids_a(low, high).sum::<u64>();
            Ok(next)
        })?;

    Ok(sum)
}

fn b(input: &str) -> anyhow::Result<u64> {
    let sum = input
        .split(',')
        .map(parse)
        .try_fold::<_, _, anyhow::Result<_>>(0, |sum, range| {
            let (low, high) = range?;
            let next = sum + iter_invalid_ids_b(low, high).sum::<u64>();
            Ok(next)
        })?;

    Ok(sum)
}

fn iter_invalid_ids_a(low: u64, high: u64) -> impl Iterator<Item = u64> {
    let (start, start_len) = find_even_len(low);
    let mut half = downshift(start, start_len / 2);

    std::iter::from_fn(move || {
        loop {
            let len = len(half);
            let id = upshift(half, len) + half;
            half += 1;

            if id < low {
                continue;
            } else if id <= high {
                return Some(id);
            } else {
                return None;
            }
        }
    })
}

fn iter_invalid_ids_b(low: u64, high: u64) -> impl Iterator<Item = u64> {
    (low..=high).filter(|id| is_invalid_input_b(*id))
}

fn is_invalid_input_b(id: u64) -> bool {
    let len = len(id);
    for len in 1..=(len / 2) {
        if is_repeating_pattern(id, len) {
            return true;
        }
    }

    false
}

fn is_repeating_pattern(id: u64, pattern_len: u32) -> bool {
    let mask = 10u64.strict_pow(pattern_len);
    let pattern = id % mask;
    let mut remaining = id / mask;
    let mut len = len(remaining);
    while remaining != 0 {
        let part = remaining % mask;
        if len < pattern_len || part != pattern {
            return false;
        }

        remaining /= mask;
        len -= pattern_len;
    }

    true
}

const fn len(n: u64) -> u32 {
    if n > 0 { 1 + n.ilog10() } else { 1 }
}

const fn find_even_len(n: u64) -> (u64, u32) {
    let len = len(n);
    if len.is_multiple_of(2) {
        (n, len)
    } else {
        (10u64.strict_pow(len), len + 1)
    }
}

const fn upshift(n: u64, amt: u32) -> u64 {
    let mut iter = amt;
    let mut value = n;
    while iter != 0 {
        value *= 10;
        iter -= 1;
    }

    value
}

const fn downshift(n: u64, amt: u32) -> u64 {
    let mut iter = amt;
    let mut value = n;
    while iter != 0 {
        value /= 10;
        iter -= 1;
    }

    value
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(11, 22, &[11, 22])]
    #[case(95, 115, &[99])]
    #[case(998, 1012, &[1010])]
    #[case(1188511880, 1188511890, &[1188511885])]
    #[case(222220, 222224, &[222222])]
    #[case(1698522, 1698528, &[])]
    #[case(446443, 446449, &[446446])]
    #[case(38593856, 38593862, &[38593859])]
    fn test_iter_invalid_ids_a(#[case] low: u64, #[case] high: u64, #[case] expected: &[u64]) {
        let result: Vec<_> = super::iter_invalid_ids_a(low, high).collect();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(11, 22, &[11, 22])]
    #[case(95, 115, &[99, 111])]
    #[case(998, 1012, &[999, 1010])]
    #[case(1188511880, 1188511890, &[1188511885])]
    #[case(222220, 222224, &[222222])]
    #[case(1698522, 1698528, &[])]
    #[case(446443, 446449, &[446446])]
    #[case(38593856, 38593862, &[38593859])]
    #[case(565653, 565659, &[565656])]
    #[case(2121212118, 2121212124, &[2121212121])]
    fn test_iter_invalid_ids_b(#[case] low: u64, #[case] high: u64, #[case] expected: &[u64]) {
        let result: Vec<_> = super::iter_invalid_ids_b(low, high).collect();
        assert_eq!(result, expected);
    }
}
