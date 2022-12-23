#![feature(type_alias_impl_trait)]

use aoc_helper::AOCSolution;
use std::error::Error;

struct Day13;

impl<'a> AOCSolution<'a> for Day13 {
    type ParsedInput = impl Iterator<Item = &'a str> + Clone + 'a;
    type Part1Out = usize;
    type Part2Out = usize;

    fn parse(input: &'a str) -> Self::ParsedInput {
        input.split("\n\n")
    }

    fn part1(parsed: Self::ParsedInput) -> Self::Part1Out {
        todo!();
    }

    fn part2(parsed: Self::ParsedInput) -> Self::Part2Out {
        todo!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = include_str!("../../../inputs/day13.txt");
    let parsed = Day13::parse(inp);

    let (p1, p2) = (Day13::part1(parsed.clone()), Day13::part2(parsed));

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");

    Ok(())
}
