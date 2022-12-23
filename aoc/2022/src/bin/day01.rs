#![feature(type_alias_impl_trait)]

use aoc_helper::AOCSolution;
use std::error::Error;

struct Day1;

impl<'a> AOCSolution<'a> for Day1 {
    type ParsedInput = impl Iterator<Item = usize> + Clone + 'a;
    type Part1Out = usize;
    type Part2Out = usize;

    fn parse(input: &'a str) -> Self::ParsedInput {
        input
            .split("\n\n")
            .map(|s| s.lines().filter_map(|n| n.parse::<usize>().ok()).sum())
    }

    fn part1(parsed: Self::ParsedInput) -> Self::Part1Out {
        parsed.max().unwrap()
    }

    fn part2(parsed: Self::ParsedInput) -> Self::Part2Out {
        let mut aux_arr = [0; 3 + 1];

        parsed.for_each(|e| {
            aux_arr[0] = e;
            aux_arr.sort_unstable();
        });

        aux_arr[1..].iter().sum()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = include_str!("../../inputs/day01.txt");
    let parsed = Day1::parse(inp);

    let (p1, p2) = (Day1::part1(parsed.clone()), Day1::part2(parsed));

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");

    Ok(())
}
