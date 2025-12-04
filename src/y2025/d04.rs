use foldhash::HashSet;

use crate::solution::Solution;

pub fn solution() -> Solution {
    Solution::new().with_a(a).with_b(b)
}

fn parse(input: &str) -> HashSet<(i32, i32)> {
    let mut rolls = HashSet::default();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '@' {
                rolls.insert((x as i32, y as i32));
            }
        }
    }

    rolls
}

fn a(input: &str) -> anyhow::Result<u64> {
    let rolls = parse(input);

    let accessible = count_accessible_rolls(&rolls);
    Ok(accessible)
}

fn b(input: &str) -> anyhow::Result<u64> {
    let mut rolls = parse(input);

    let mut removed = 0;
    loop {
        let mut next = HashSet::default();
        remove_accessible_rolls(&rolls, &mut next);
        let removed_this_round = rolls.len() - next.len();
        if removed_this_round == 0 {
            break;
        }

        removed += removed_this_round as u64;
        rolls = next;
    }

    Ok(removed)
}

fn count_accessible_rolls(rolls: &HashSet<(i32, i32)>) -> u64 {
    let mut accessible = 0;
    for &(x, y) in rolls.iter() {
        let neighborhood = moore_neighborhood(x, y);
        let rolls = neighborhood.filter(|n| rolls.contains(n));
        let roll_count = rolls.count();

        if roll_count < 4 {
            accessible += 1;
        }
    }

    accessible
}

fn remove_accessible_rolls(read: &HashSet<(i32, i32)>, write: &mut HashSet<(i32, i32)>) {
    for &(x, y) in read.iter() {
        let neighborhood = moore_neighborhood(x, y);
        let rolls = neighborhood.filter(|n| read.contains(n));
        let roll_count = rolls.count();

        if roll_count >= 4 {
            write.insert((x, y));
        }
    }
}

const MOORE_NEIGHBORHOOD: &[(i32, i32); 8] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn moore_neighborhood(x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
    MOORE_NEIGHBORHOOD
        .iter()
        .map(move |&(xoff, yoff)| (x + xoff, y + yoff))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[rstest]
    #[case(TEST_INPUT)]
    fn test_a(#[case] input: &str) {
        let result = super::a(input).unwrap();
        assert_eq!(result, 13);
    }

    #[rstest]
    #[case(TEST_INPUT)]
    fn test_b(#[case] input: &str) {
        let result = super::b(input).unwrap();
        assert_eq!(result, 43);
    }
}
