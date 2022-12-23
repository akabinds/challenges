#![feature(iter_array_chunks, type_alias_impl_trait)]

use aoc_helper::AOCSolution;
use std::error::Error;

struct Day3;

const ALPHA: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn char_priority(c: char) -> Option<u8> {
    ALPHA.iter().position(|a| *a == c).map(|idx| idx as u8 + 1)
}

impl<'a> AOCSolution<'a> for Day3 {
    type ParsedInput = impl Iterator<Item = &'a str> + Clone + 'a;
    type Part1Out = usize;
    type Part2Out = usize;

    fn parse(input: &'a str) -> Self::ParsedInput {
        input.lines()
    }

    fn part1(parsed: Self::ParsedInput) -> Self::Part1Out {
        parsed.fold(0, |acc, r| {
            let (c1, c2) = r.split_at(r.len() / 2);

            acc + c1
                .chars()
                .find(|c| c2.contains(*c))
                .and_then(char_priority)
                .unwrap() as usize
        })
    }

    fn part2(parsed: Self::ParsedInput) -> Self::Part2Out {
        parsed.array_chunks::<3>().fold(0, |acc, [r1, r2, r3]| {
            acc + r1
                .chars()
                .find(|c| r2.contains(*c) && r3.contains(*c))
                .and_then(char_priority)
                .unwrap() as usize
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = include_str!("../../inputs/day03.txt");
    let parsed = Day3::parse(inp);

    let (p1, p2) = (Day3::part1(parsed.clone()), Day3::part2(parsed));

    println!("Part 1 Sum: {p1}");
    println!("Part 2 Sum: {p2}");

    Ok(())
}
