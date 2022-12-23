pub trait AOCSolution<'a> {
    type ParsedInput: Clone;
    type Part1Out;
    type Part2Out;

    fn parse(input: &'a str) -> Self::ParsedInput;
    fn part1(parsed: Self::ParsedInput) -> Self::Part1Out;
    fn part2(parsed: Self::ParsedInput) -> Self::Part2Out;
}
