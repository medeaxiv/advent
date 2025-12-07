use crate::{
    solution::Solution,
    util::{char::FromChar, grid::Grid},
};

pub fn solution() -> Solution {
    Solution::new().with_a(a).with_b(b)
}

fn parse(input: &str) -> anyhow::Result<Grid<Tile>> {
    let mut width = 0;
    let mut height = 0;
    let mut tiles = Vec::new();

    for (y, line) in input.lines().step_by(2).enumerate() {
        height += 1;
        for ch in line.chars() {
            if y == 0 {
                width += 1;
            }

            let tile = Tile::from_char(ch)?;
            tiles.push(tile);
        }
    }

    let grid = Grid::from_vec(width, height, tiles)?;
    Ok(grid)
}

fn a(input: &str) -> anyhow::Result<u64> {
    let grid = parse(input)?;
    let mut beams = vec![false; grid.width() as usize];
    let mut next = beams.clone();

    let mut splits = 0;
    for tiles in grid.rows() {
        for (x, tile) in tiles.iter().enumerate() {
            match tile {
                Tile::Empty => {}
                Tile::Start => {
                    next[x] = true;
                }
                Tile::Splitter => {
                    let had_beam = next[x];
                    next[x] = false;
                    next[x - 1] = true;
                    next[x + 1] = true;
                    splits += had_beam as u64;
                }
            }
        }

        beams.copy_from_slice(&next);
    }

    Ok(splits)
}

fn b(input: &str) -> anyhow::Result<u64> {
    let grid = parse(input)?;
    let mut beams = vec![0; grid.width() as usize];
    let mut next = beams.clone();

    for tiles in grid.rows() {
        for (x, tile) in tiles.iter().enumerate() {
            match tile {
                Tile::Empty => {}
                Tile::Start => {
                    next[x] += 1;
                }
                Tile::Splitter => {
                    let count = next[x];
                    next[x] = 0;
                    next[x - 1] += count;
                    next[x + 1] += count;
                }
            }
        }

        beams.copy_from_slice(&next);
    }

    let timelines = beams.into_iter().sum();
    Ok(timelines)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Start,
    Splitter,
}

impl FromChar for Tile {
    type Err = std::convert::Infallible;

    fn from_char(ch: char) -> Result<Self, Self::Err> {
        match ch {
            'S' => Ok(Self::Start),
            '^' => Ok(Self::Splitter),
            _ => Ok(Self::Empty),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[rstest]
    #[case(TEST_INPUT, 21)]
    fn test_a(#[case] input: &str, #[case] expected: u64) {
        let result = super::a(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(TEST_INPUT, 40)]
    fn test_b(#[case] input: &str, #[case] expected: u64) {
        let result = super::b(input).unwrap();
        assert_eq!(result, expected);
    }
}
