use foldhash::HashSet;
use nalgebra::SimdPartialOrd;

use crate::{solution::Solution, util::vector::IVec2};

pub fn solution() -> Solution {
    Solution::new().with_a(a)
}

fn a(input: &str) -> anyhow::Result<u64> {
    let (shapes, regions) = parser::parse(input)?;
    let upper_shape_bound = shapes
        .iter()
        .map(|s| s.size)
        .fold(IVec2::zeros(), |a, s| a.simd_max(s));

    let mut count = 0;
    for region in regions.iter() {
        if can_fit_upper_shape_bound(upper_shape_bound, region) {
            // region can fit the upper bound of every shape without complex packing
            count += 1;
            continue;
        }

        let (can_fit_upper_piece_count, can_fit_exact_piece_count) =
            can_fit_piece_count(&shapes, region);

        if can_fit_upper_piece_count != can_fit_exact_piece_count {
            return Err(anyhow::anyhow!(
                "unexpected state: upper={}, exact={}",
                can_fit_upper_piece_count,
                can_fit_exact_piece_count
            ));
        }

        if can_fit_upper_piece_count {
            count += 1;
        }
    }

    Ok(count)
}
fn can_fit_upper_shape_bound(size: IVec2, region: &Region) -> bool {
    let shape_space = region.size.component_div(&size);
    let shape_count = region.shape_counts.iter().copied().sum::<i64>();
    let capacity = shape_space.x * shape_space.y;
    debug_assert!(capacity >= 0);

    capacity > shape_count
}

fn can_fit_piece_count(shapes: &[Shape], region: &Region) -> (bool, bool) {
    let capacity = region.size.x * region.size.y;
    debug_assert!(capacity >= 0);

    let mut total_upper = 0;
    let mut total_exact = 0;
    for (shape, &count) in shapes.iter().zip(region.shape_counts.iter()) {
        let upper = shape.size.x * shape.size.y;
        let exact = shape.pieces.len() as i64;
        debug_assert!(upper >= 0);

        total_upper += upper * count;
        total_exact += exact * count;
    }

    (capacity >= total_upper, capacity >= total_exact)
}

#[derive(Debug, Clone)]
struct Shape {
    pieces: HashSet<IVec2>,
    size: IVec2,
}

#[derive(Debug, Clone)]
struct Region {
    size: IVec2,
    shape_counts: Vec<i64>,
}

mod parser {
    use std::str::FromStr;

    use foldhash::HashSet;
    use nalgebra::SimdPartialOrd;

    use crate::util::{
        invalid_input,
        vector::{IVec2, vec2},
    };

    use super::{Region, Shape};

    pub fn parse(input: &str) -> anyhow::Result<(Vec<Shape>, Vec<Region>)> {
        let (shapes, regions) = input.rsplit_once("\n\n").ok_or_else(invalid_input!())?;
        let shapes = parse_shapes(shapes)?;
        let regions = parse_regions(regions)?;
        Ok((shapes, regions))
    }

    pub fn parse_shapes(input: &str) -> anyhow::Result<Vec<Shape>> {
        let mut shapes = Vec::new();
        for part in input.split("\n\n") {
            let (_index, shape) = part.split_once("\n").ok_or_else(invalid_input!())?;
            let shape = parse_shape(shape)?;
            shapes.push(shape);
        }

        Ok(shapes)
    }

    fn parse_shape(input: &str) -> anyhow::Result<Shape> {
        let mut pieces = HashSet::default();
        let mut size = IVec2::zeros();
        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    let piece = vec2(x as i64, y as i64);
                    pieces.insert(piece);
                    size = size.simd_max(piece + vec2(1, 1));
                }
            }
        }

        Ok(Shape { pieces, size })
    }

    fn parse_regions(input: &str) -> anyhow::Result<Vec<Region>> {
        let regions = input.lines().map(parse_region).collect::<Result<_, _>>()?;
        Ok(regions)
    }

    pub fn parse_region(input: &str) -> anyhow::Result<Region> {
        let (size, shape_counts) = input.split_once(": ").ok_or_else(invalid_input!())?;
        let (width, height) = size.split_once('x').ok_or_else(invalid_input!())?;
        let width = width.parse()?;
        let height = height.parse()?;
        let size = vec2(width, height);
        let shape_counts = shape_counts
            .split_ascii_whitespace()
            .map(FromStr::from_str)
            .collect::<Result<_, _>>()?;

        Ok(Region { size, shape_counts })
    }
}
