use aoc_helper::AOCSolution;
use std::error::Error;

struct Day6;

fn chunk_has_n_unique<const N: usize>(chunk: &[char]) -> bool {
    for (idx, c) in chunk.iter().take(N.min(chunk.len())).enumerate() {
        if chunk[..idx].contains(c) {
            return false;
        }
    }

    true
}

impl<'a> AOCSolution<'a> for Day6 {
    type ParsedInput = Vec<char>;
    type Part1Out = usize;
    type Part2Out = usize;

    fn parse(input: &'a str) -> Self::ParsedInput {
        input.chars().collect()
    }

    fn part1(parsed: Self::ParsedInput) -> Self::Part1Out {
        parsed
            .windows(4)
            .position(chunk_has_n_unique::<4>)
            .map(|idx| idx + 4)
            .unwrap()
    }

    fn part2(parsed: Self::ParsedInput) -> Self::Part2Out {
        parsed
            .windows(14)
            .position(chunk_has_n_unique::<14>)
            .map(|idx| idx + 14)
            .unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = include_str!("../../inputs/day06.txt");
    let parsed = Day6::parse(inp);

    let (p1, p2) = (Day6::part1(parsed.clone()), Day6::part2(parsed));

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");

    Ok(())
}
