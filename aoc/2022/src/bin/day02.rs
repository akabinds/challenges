#![feature(type_alias_impl_trait)]

use aoc_helper::AOCSolution;
use std::error::Error;

struct Day2;

fn outcome_of(moves: (char, char)) -> usize {
    match moves {
        ('A', 'X') => 4,
        ('A', 'Y') => 8,
        ('A', 'Z') => 3,
        ('B', 'X') => 1,
        ('B', 'Y') => 5,
        ('B', 'Z') => 9,
        ('C', 'X') => 7,
        ('C', 'Y') => 2,
        ('C', 'Z') => 6,
        _ => unreachable!(),
    }
}

fn from_outcome((op, outcome): (char, char)) -> usize {
    match (op, outcome) {
        ('A', 'X') => 3,
        ('A', 'Y') => 4,
        ('A', 'Z') => 8,
        ('B', 'X') => 1,
        ('B', 'Y') => 5,
        ('B', 'Z') => 9,
        ('C', 'X') => 2,
        ('C', 'Y') => 6,
        ('C', 'Z') => 7,
        _ => unreachable!(),
    }
}

impl<'a> AOCSolution<'a> for Day2 {
    type ParsedInput = impl Iterator<Item = (char, char)> + Clone + 'a;
    type Part1Out = usize;
    type Part2Out = usize;

    fn parse(input: &'a str) -> Self::ParsedInput {
        input.lines().filter_map(|line| {
            let (op, other) = line.split_once(' ')?;
            let op = op.chars().next()?;
            let other = other.chars().next()?;
            Some((op, other))
        })
    }

    fn part1(parsed: Self::ParsedInput) -> Self::Part1Out {
        parsed.fold(0, |acc, moves| acc + outcome_of(moves))
    }

    fn part2(parsed: Self::ParsedInput) -> Self::Part2Out {
        parsed.fold(0, |acc, (op, outcome)| acc + from_outcome((op, outcome)))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = include_str!("../../inputs/day02.txt");
    let parsed = Day2::parse(inp);

    let (p1, p2) = (Day2::part1(parsed.clone()), Day2::part2(parsed));

    println!("Part 1 Score: {p1}");
    println!("Part 2 Score: {p2}");

    Ok(())
}
