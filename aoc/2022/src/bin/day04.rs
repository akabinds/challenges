#![feature(type_alias_impl_trait, iter_array_chunks)]

use aoc_helper::AOCSolution;
use itertools::Itertools;
use std::{collections::HashSet, error::Error, ops::RangeInclusive};

struct Day4;

impl<'a> AOCSolution<'a> for Day4 {
    type ParsedInput =
        impl Iterator<Item = (RangeInclusive<usize>, RangeInclusive<usize>)> + Clone + 'a;
    type Part1Out = usize;
    type Part2Out = usize;

    fn parse(input: &'a str) -> Self::ParsedInput {
        input.lines().filter_map(|s| {
            s.split(|c| c == ',' || c == '-')
                .filter_map(|s| s.parse::<usize>().ok())
                .array_chunks::<2>()
                .map(|[start, end]| start..=end)
                .collect_tuple()
        })
    }

    fn part1(parsed: Self::ParsedInput) -> Self::Part1Out {
        parsed.fold(0, |mut acc, (r1, r2)| {
            let (mut r1set, mut r2set) = (HashSet::new(), HashSet::new());
            r1set.extend(r1);
            r2set.extend(r2);

            if r1set.is_superset(&r2set) || r2set.is_superset(&r1set) {
                acc += 1;
            }

            acc
        })
    }

    fn part2(parsed: Self::ParsedInput) -> Self::Part2Out {
        parsed.fold(0, |mut acc, (mut r1, mut r2)| {
            if r1.any(|n| r2.contains(&n)) || r2.any(|n| r1.contains(&n)) {
                acc += 1;
            }

            acc
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = include_str!("../../inputs/day04.txt");
    let parsed = Day4::parse(inp);

    let (p1, p2) = (Day4::part1(parsed.clone()), Day4::part2(parsed));

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");

    Ok(())
}
