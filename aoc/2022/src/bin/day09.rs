#![feature(type_alias_impl_trait)]
#![allow(unused_variables, unused_mut)]

use aoc_helper::AOCSolution;
use itertools::Itertools;
use std::{collections::HashSet, error::Error};

struct Day9;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        use Direction::*;

        match c {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => unreachable!(),
        }
    }
}

struct Instruction {
    direction: Direction,
    amount: u8,
}

impl Instruction {
    fn new(direction: Direction, amount: u8) -> Self {
        Self { direction, amount }
    }

    fn offset(&self) -> (i32, i32) {
        use Direction::*;

        match self.direction {
            Up => (0, 1),
            Down => (0, -1),
            Left => (-1, 0),
            Right => (1, 0),
        }
    }
}

impl<'a> AOCSolution<'a> for Day9 {
    type ParsedInput = impl Iterator<Item = Instruction> + Clone + 'a;
    type Part1Out = usize;
    type Part2Out = usize;

    fn parse(input: &'a str) -> Self::ParsedInput {
        input.lines().filter_map(|line| {
            let (direction, amount) = line.split_once(' ')?;
            let direction = Direction::from_char(direction.chars().next()?);
            let amount = amount.parse::<u8>().ok()?;

            let instruction = Instruction::new(direction, amount);

            Some(instruction)
        })
    }

    fn part1(parsed: Self::ParsedInput) -> Self::Part1Out {
        let mut head_location = (0, 0);
        let mut tail_location = (0, 0);
        let mut tail_visited = HashSet::<(i32, i32)>::from([(0, 0)]);

        for instr in parsed {
            let offset = instr.offset();

            for _ in 0..instr.amount {
                head_location = (head_location.0 + offset.0, head_location.1 + offset.1);

                let delta = (
                    head_location.0 - tail_location.0,
                    head_location.1 - tail_location.1,
                );

                match (delta.0.abs(), delta.1.abs()) {
                    (1, 0) | (0, 1) | (1, 1) | (0, 0) => continue,
                    (_, 0) => tail_location.0 += delta.0.signum(),
                    (0, _) => tail_location.1 += delta.1.signum(),
                    (_, _) => {
                        tail_location.0 += delta.0.signum();
                        tail_location.1 += delta.1.signum();
                    }
                }

                tail_visited.insert(tail_location);
            }
        }

        tail_visited.len()
    }

    fn part2(parsed: Self::ParsedInput) -> Self::Part2Out {
        const KNOT_COUNT: usize = 10;
        let mut knots = [(0i32, 0i32); KNOT_COUNT];
        let mut tail_visited = HashSet::<(i32, i32)>::from([(0, 0)]);

        for instr in parsed {
            let offset = instr.offset();

            for _ in 0..instr.amount {
                knots[0] = (knots[0].0 + offset.0, knots[0].1 + offset.1);
                for (last_idx, current_idx) in (0..knots.len()).tuple_windows() {
                    let old_knot = knots[last_idx];
                    let mut knot = knots.get_mut(current_idx).unwrap();
                    let delta: (i32, i32) = (old_knot.0 - knot.0, old_knot.1 - knot.1);

                    match (delta.0.abs(), delta.1.abs()) {
                        (1, 0) | (0, 1) | (1, 1) | (0, 0) => continue,
                        (_, 0) => knot.0 += delta.0.signum(),
                        (0, _) => knot.1 += delta.1.signum(),
                        (_, _) => {
                            knot.0 += delta.0.signum();
                            knot.1 += delta.1.signum();
                        }
                    }
                }

                tail_visited.insert(knots[KNOT_COUNT - 1]);
            }
        }

        tail_visited.len()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = include_str!("../../inputs/day09.txt");
    let parsed = Day9::parse(inp);

    let (p1, p2) = (Day9::part1(parsed.clone()), Day9::part2(parsed));

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");

    Ok(())
}
