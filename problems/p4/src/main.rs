use sscanf::sscanf;

struct SectionRange {
    start: usize,
    end: usize,
}

fn section_ranges_from_line(line: &str) -> (SectionRange, SectionRange) {
    let (s1, e1, s2, e2) = sscanf!(line, "{usize}-{usize},{usize}-{usize}").unwrap();
    (
        SectionRange { start: s1, end: e1 },
        SectionRange { start: s2, end: e2 },
    )
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
