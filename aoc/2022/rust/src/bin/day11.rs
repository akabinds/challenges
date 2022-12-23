#![feature(
    type_alias_impl_trait,
    iter_array_chunks,
    iter_next_chunk,
    get_many_mut
)]
#![allow(unused_variables, unused_mut)]

use aoc_helper::AOCSolution;
use std::{collections::HashMap, error::Error};

struct Day11;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Monkey {
    items: Vec<usize>,
    op: (String, u8),
    div_by_test: usize,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        op: (&str, u8),
        div_by_test: usize,
        if_true: usize,
        if_false: usize,
    ) -> Self {
        Self {
            items,
            op: (op.0.to_owned(), op.1),
            div_by_test,
            if_true,
            if_false,
        }
    }
}

impl<'a> AOCSolution<'a> for Day11 {
    type ParsedInput = Vec<Monkey>;
    type Part1Out = usize;
    type Part2Out = usize;

    fn parse(input: &'a str) -> Self::ParsedInput {
        input
            .split("\n\n")
            .filter_map(|s| {
                let monkey_data = s
                    .split("\n  ")
                    .map(|s| {
                        s.trim()
                            .chars()
                            .filter(|c| c.is_numeric() || *c == '+' || *c == '*')
                            .collect::<String>()
                    })
                    .skip(1)
                    .next_chunk::<5>()
                    .ok()?;

                let items = monkey_data[0]
                    .chars()
                    .array_chunks::<2>()
                    .filter_map(|[n1, n2]| {
                        let mut s = String::with_capacity(2);
                        s.push_str(&format!("{n1}{n2}"));

                        s.parse::<usize>().ok()
                    })
                    .collect::<Vec<_>>();

                let (op, num) = monkey_data[1].split_at(1);
                let op = if op == "*" && num.is_empty() {
                    "**"
                } else {
                    op
                };
                let num = if op == "**" {
                    2
                } else {
                    num.parse::<u8>().ok()?
                };

                let div_by_test = monkey_data[2].parse::<usize>().ok()?;
                let if_true = monkey_data[3].parse::<usize>().ok()?;
                let if_false = monkey_data[4].parse::<usize>().ok()?;

                Some(Monkey::new(
                    items,
                    (op, num),
                    div_by_test,
                    if_true,
                    if_false,
                ))
            })
            .collect()
    }

    fn part1(mut parsed: Self::ParsedInput) -> Self::Part1Out {
        let mut inspected = HashMap::<usize, usize>::new();

        for _ in 0..20 {
            for i in 0..parsed.len() {
                let monkey = parsed.get_mut(i).unwrap();
                let mut items = std::mem::take(&mut monkey.items);
                let items_len = items.len();

                items.iter_mut().for_each(|i| {
                    let (sign, by) = (monkey.op.0.as_str(), monkey.op.1 as usize);

                    *i = match sign {
                        "+" => *i + by,
                        "*" => *i * by,
                        "**" => (*i).pow(2),
                        _ => 0,
                    } / 3
                });

                inspected.insert(i, inspected.get(&i).unwrap_or(&0usize) + items_len);
                let clone = monkey.clone();

                items.iter().for_each(|i| {
                    if i % clone.div_by_test == 0 {
                        parsed[clone.if_true].items.push(*i);
                    } else {
                        parsed[clone.if_false].items.push(*i);
                    }
                });
            }
        }

        let mut inspected = inspected
            .values()
            .into_iter()
            .copied()
            .collect::<Vec<usize>>();

        inspected.sort_unstable();

        let len = inspected.len();
        inspected[len - 1] * inspected[len - 2]
    }

    fn part2(mut parsed: Self::ParsedInput) -> Self::Part2Out {
        let mut inspected = HashMap::<usize, usize>::new();

        let lcm = parsed
            .iter()
            .map(|m| m.div_by_test)
            .fold(1, num::integer::lcm);

        for _ in 0..10000 {
            for i in 0..parsed.len() {
                let monkey = parsed.get_mut(i).unwrap();
                let mut items = std::mem::take(&mut monkey.items);
                let items_len = items.len();

                items.iter_mut().for_each(|i| {
                    let (sign, by) = (monkey.op.0.as_str(), monkey.op.1 as usize);

                    *i = match sign {
                        "+" => *i + by,
                        "*" => *i * by,
                        "**" => (*i).pow(2),
                        _ => 0,
                    } % lcm
                });

                inspected.insert(i, inspected.get(&i).unwrap_or(&0usize) + items_len);
                let clone = monkey.clone();

                items.iter().for_each(|i| {
                    if i % clone.div_by_test == 0 {
                        parsed[clone.if_true].items.push(*i);
                    } else {
                        parsed[clone.if_false].items.push(*i);
                    }
                })
            }
        }

        let mut inspected = inspected
            .values()
            .into_iter()
            .copied()
            .collect::<Vec<usize>>();

        inspected.sort_unstable();

        let len = inspected.len();
        inspected[len - 1] * inspected[len - 2]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = include_str!("../../../inputs/day11.txt");
    let parsed = Day11::parse(inp);

    let (p1, p2) = (Day11::part1(parsed.clone()), Day11::part2(parsed));

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");

    Ok(())
}
