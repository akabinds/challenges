#![feature(type_alias_impl_trait)]
#![allow(unused_variables, unused_mut)]

use aoc_helper::AOCSolution;
use std::{
    error::Error,
    io::{self, BufWriter, Write},
};

struct Day10;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Noop,
    Addx(i16),
}

fn signal_strength(cycles: i16, reg_val: i16) -> i16 {
    if (cycles - 20) % 40 == 0 {
        return cycles * reg_val;
    }

    0
}

impl<'a> AOCSolution<'a> for Day10 {
    type ParsedInput = impl Iterator<Item = Instruction> + Clone + 'a;
    type Part1Out = i16;
    type Part2Out = ();

    fn parse(input: &'a str) -> Self::ParsedInput {
        input.lines().map(|line| {
            let (_, val) = line.split_at(4);

            if val.is_empty() {
                Instruction::Noop
            } else {
                Instruction::Addx(val.trim().parse().unwrap())
            }
        })
    }

    fn part1(parsed: Self::ParsedInput) -> Self::Part1Out {
        let mut cycles = 1;
        let mut reg_val = 1;
        let mut res = 0;

        parsed.for_each(|instruction| {
            res += signal_strength(cycles, reg_val);

            if let Instruction::Addx(v) = instruction {
                cycles += 1;
                res += signal_strength(cycles, reg_val);
                reg_val += v;
            }

            cycles += 1;
        });

        res
    }

    fn part2(parsed: Self::ParsedInput) -> Self::Part2Out {
        const CRT_PIXELS: usize = 40 * 6 + 6;
        let mut cycles = 1;
        let mut reg_val = 1;
        let mut crt = [32u8; CRT_PIXELS];

        parsed.for_each(|instruction| {
            todo!();
        });

        let stdout = io::stdout().lock();
        let mut writer = BufWriter::with_capacity(CRT_PIXELS, stdout);
        writer.write_all(&crt).unwrap();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = include_str!("../../inputs/day10.txt");
    let parsed = Day10::parse(inp);

    let (p1, p2) = (Day10::part1(parsed.clone()), Day10::part2(parsed));

    println!("Part 1: {p1}");

    Ok(())
}
