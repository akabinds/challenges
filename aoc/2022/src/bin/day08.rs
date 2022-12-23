#![allow(unused_mut, unused_variables)]

use aoc_helper::AOCSolution;
use std::error::Error;

struct Day8;

trait GridChecks {
    fn on_edge(&self, coords: (usize, usize)) -> bool;
}

type Grid = Vec<Vec<u32>>;

impl GridChecks for Grid {
    fn on_edge(&self, coords: (usize, usize)) -> bool {
        let (row, col) = coords;

        row == 0 || row == (self.len() - 1) || col == 0 || col == (self[row].len() - 1)
    }
}

impl<'a> AOCSolution<'a> for Day8 {
    type ParsedInput = Grid;
    type Part1Out = u64;
    type Part2Out = u64;

    fn parse(input: &'a str) -> Self::ParsedInput {
        let mut grid = Vec::new();

        input
            .lines()
            .for_each(|line| grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect()));

        grid
    }

    fn part1(parsed: Self::ParsedInput) -> Self::Part1Out {
        let mut visible = 0;

        for (row_idx, row) in parsed.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if parsed.on_edge((row_idx, col_idx)) {
                    visible += 1;
                    continue;
                }
            }
        }

        visible
    }

    fn part2(parsed: Self::ParsedInput) -> Self::Part2Out {
        todo!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = include_str!("../../inputs/day08.txt");
    let parsed = Day8::parse(inp);

    let (p1, p2) = (Day8::part1(parsed.clone()), Day8::part2(parsed));

    println!("Part 1 Score: {p1}");
    println!("Part 2 Score: {p2}");

    Ok(())
}
