use aoc_helper::AOCSolution;
use std::{collections::HashSet, error::Error};

struct Day12;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate((usize, usize));

type Grid = Vec<Vec<char>>;

trait GridMethods {
    fn get_row(&self, row: usize) -> Option<(usize, &Vec<char>)>;
    fn get_char(&self, coords: Coordinate) -> Option<char>;
}

impl GridMethods for Grid {
    fn get_row(&self, row: usize) -> Option<(usize, &Vec<char>)> {
        for (idx, r) in self.iter().enumerate() {
            if idx == row {
                return Some((row, r));
            } else {
                continue;
            }
        }

        None
    }

    fn get_char(&self, coords: Coordinate) -> Option<char> {
        let (row, col) = coords.0;

        let Some((_, row)) = self.get_row(row) else {
            panic!("Row with that number does not exist");
        };

        for (idx, ch) in row.iter().enumerate() {
            if idx == col {
                return Some(*ch);
            } else {
                continue;
            }
        }

        None
    }
}

fn dfs(
    grid: Grid,
    start: Coordinate,
    target: Coordinate,
    mut visited: HashSet<Coordinate>,
) -> usize {
    visited.insert(start);
    let (row_num, row) = grid.get_row(start.0 .0).unwrap();

    todo!();
}

impl<'a> AOCSolution<'a> for Day12 {
    type ParsedInput = (Grid, (Coordinate, Coordinate));
    type Part1Out = usize;
    type Part2Out = usize;

    fn parse(input: &'a str) -> Self::ParsedInput {
        let char_matrix = input.lines().map(|s| s.chars().collect()).collect();

        let start_end_coords = input
            .lines()
            .enumerate()
            .filter(|(_, line)| line.chars().any(|c| c == 'E' || c == 'S'))
            .map(|(idx, line)| {
                let start_pos = line.chars().position(|c| c == 'S').unwrap();
                let end_pos = line.chars().position(|c| c == 'E').unwrap();

                (Coordinate((idx, start_pos)), Coordinate((idx, end_pos)))
            })
            .next()
            .unwrap();

        (char_matrix, start_end_coords)
    }

    fn part1(parsed: Self::ParsedInput) -> Self::Part1Out {
        todo!();
    }

    fn part2(parsed: Self::ParsedInput) -> Self::Part2Out {
        todo!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = include_str!("../../../inputs/day12.txt");
    let parsed = Day12::parse(inp);

    let (p1, p2) = (Day12::part1(parsed.clone()), Day12::part2(parsed));

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");

    Ok(())
}
