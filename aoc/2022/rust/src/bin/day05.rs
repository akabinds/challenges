#![feature(type_alias_impl_trait, get_many_mut)]

use aoc_helper::AOCSolution;
use itertools::Itertools;
use std::{error::Error, num::NonZeroUsize};

struct Day5;

#[derive(Debug, Copy, Clone)]
struct Crate(char);

impl Crate {
    fn from_char(c: char) -> Self {
        Self(c)
    }
}

#[derive(Debug, Clone)]
struct Stack {
    id: usize,
    crates: Vec<Crate>,
}

impl Stack {
    fn new(id: usize, crates: Vec<Crate>) -> Self {
        Self { id, crates }
    }

    fn set_id(&mut self, new: usize) {
        self.id = new;
    }

    fn push_crate(&mut self, c: Crate) {
        self.crates.push(c)
    }

    fn pop_crate(&mut self) -> Option<Crate> {
        self.crates.pop()
    }

    fn split_off_crates(&mut self, at: usize) -> Vec<Crate> {
        self.crates.split_off(at)
    }

    fn extend_crates(&mut self, v: Vec<Crate>) {
        self.crates.extend(v)
    }

    fn get_top_crate(&self) -> Crate {
        *self.crates.last().unwrap()
    }
}

struct MoveInstruction {
    amount: NonZeroUsize,
    from: usize,
    to: usize,
}

impl MoveInstruction {
    fn new(amount: NonZeroUsize, from: usize, to: usize) -> Self {
        Self { amount, from, to }
    }

    fn exec(self, stacks: &mut [Stack], order: bool) {
        let amount = self.amount.get();
        let (to_idx, from_idx) = ((self.to - 1) as usize, (self.from - 1) as usize);
        let Ok([to, from]) = stacks
            .get_many_mut([to_idx, from_idx]) else {
                panic!("Stack not found at either or both given indices")
            };

        match order {
            true => {
                let split_idx = from.crates.len() - amount as usize;
                let removed = from.split_off_crates(split_idx);
                to.extend_crates(removed);
            }
            false => (0..amount).for_each(|_| to.push_crate(from.pop_crate().unwrap())),
        }
    }
}

impl<'a> AOCSolution<'a> for Day5 {
    type ParsedInput = (
        Vec<Stack>,
        impl Iterator<Item = MoveInstruction> + Clone + 'a,
    );
    type Part1Out = String;
    type Part2Out = String;

    fn parse(input: &'a str) -> Self::ParsedInput {
        let mut stacks = vec![Stack::new(0, Vec::<Crate>::new()); 9];
        let (initial_stacks, instructions) = input.split_once("\n\n").unwrap();

        let parsed_stacks = initial_stacks.lines().map(|line| {
            line.char_indices()
                .filter(|(idx, _)| (idx + 1) % 4 != 0)
                .map(|(_, c)| c)
                .tuples::<(_, _, _)>()
        });

        for it in parsed_stacks.rev() {
            for (idx, (_, c, _)) in it.into_iter().enumerate() {
                if c.is_whitespace() || c.is_numeric() {
                    continue;
                }

                let stack = stacks.get_mut(idx).unwrap();
                let stack_num = idx + 1;
                stack.set_id(stack_num);

                stack.push_crate(Crate::from_char(c));
            }
        }

        let instructions = instructions.lines().map(|instruction| {
            let (amount, from, to) = instruction
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .next_tuple()
                .unwrap();

            unsafe { MoveInstruction::new(NonZeroUsize::new_unchecked(amount), from, to) }
        });

        (stacks, instructions)
    }

    fn part1(parsed: Self::ParsedInput) -> Self::Part1Out {
        let mut stacks = parsed.0;

        parsed
            .1
            .for_each(|instruction| instruction.exec(&mut stacks, false));

        stacks
            .into_iter()
            .map(|stack| stack.get_top_crate().0)
            .collect::<String>()
    }

    fn part2(parsed: Self::ParsedInput) -> Self::Part2Out {
        let mut stacks = parsed.0;

        parsed
            .1
            .for_each(|instruction| instruction.exec(&mut stacks, true));

        stacks
            .into_iter()
            .map(|stack| stack.get_top_crate().0)
            .collect::<String>()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../../inputs/day05.txt");
    let parsed = Day5::parse(input);

    let (p1, p2) = (Day5::part1(parsed.clone()), Day5::part2(parsed));

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");

    Ok(())
}
