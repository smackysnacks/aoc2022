use std::collections::HashSet;

fn solve(input: &[u8], n: usize) -> usize {
    let mut set = HashSet::new();
    for i in 0..input.len() {
        for j in 0..n {
            set.insert(input[i + j]);
        }
        if set.len() == n {
            return i + n;
        }
        set.clear();
    }
    panic!("not found!");
}

fn part1(input: &[u8]) -> usize {
    solve(input, 4)
}

fn part2(input: &[u8]) -> usize {
    solve(input, 14)
}

fn main() {
    let input = include_bytes!("../input.txt");
    dbg!(part1(input));
    dbg!(part2(input));
}
