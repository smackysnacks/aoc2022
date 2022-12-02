fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| match line {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => panic!("impossible!!"),
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| match line {
            "A X" => part1("A Z"),
            "A Y" => part1("A X"),
            "A Z" => part1("A Y"),
            "B X" => part1("B X"),
            "B Y" => part1("B Y"),
            "B Z" => part1("B Z"),
            "C X" => part1("C Y"),
            "C Y" => part1("C Z"),
            "C Z" => part1("C X"),
            _ => panic!("impossible!!1"),
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input));
    dbg!(part2(input));
}
