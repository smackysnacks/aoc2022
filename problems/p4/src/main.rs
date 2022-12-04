use std::{error::Error, str::FromStr};

struct SectionRange {
    start: usize,
    end: usize,
}

impl FromStr for SectionRange {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let section_parts: Vec<&str> = s.split(|c| c == '-').collect();
        assert_eq!(section_parts.len(), 2);
        let start = section_parts[0].parse()?;
        let end = section_parts[1].parse()?;
        Ok(SectionRange { start, end })
    }
}

fn section_ranges_from_line(line: &str) -> (SectionRange, SectionRange) {
    let parts: Vec<&str> = line.split(|c| c == ',').collect();
    let r1: SectionRange = parts[0].parse().unwrap();
    let r2: SectionRange = parts[1].parse().unwrap();

    (r1, r2)
}

fn fully_overlap(r1: SectionRange, r2: SectionRange) -> bool {
    (r1.start <= r2.start && r2.end <= r1.end) || (r2.start <= r1.start && r1.end <= r2.end)
}

fn partially_overlap(r1: SectionRange, r2: SectionRange) -> bool {
    r1.start <= r2.end && r2.start <= r1.end
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(section_ranges_from_line)
        .map(|(r1, r2)| fully_overlap(r1, r2))
        .map(usize::from)
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(section_ranges_from_line)
        .map(|(r1, r2)| partially_overlap(r1, r2))
        .map(usize::from)
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input));
    dbg!(part2(input));
}
