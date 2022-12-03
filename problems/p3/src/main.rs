const fn priority(x: u8) -> u8 {
    if b'a' <= x && x <= b'z' {
        return 26 - (b'z' - x);
    }
    (26 - (b'Z' - x)) + 26
}

fn find_misplaced(sack1: &[u8], sack2: &[u8]) -> u8 {
    let mut table = [0u8; 256];
    for &item in sack1 {
        table[item as usize] = 1;
    }
    for &item in sack2 {
        if table[item as usize] == 1 {
            return item;
        }
    }

    panic!("no duplicate item found!");
}

fn find_badge(sack1: &[u8], sack2: &[u8], sack3: &[u8]) -> u8 {
    let mut table = [0u8; 256];
    for &item in sack1 {
        table[item as usize] = 1;
    }
    for &item in sack2 {
        if table[item as usize] == 1 {
            table[item as usize] = 2;
        }
    }
    for &item in sack3 {
        if table[item as usize] == 2 {
            return item;
        }
    }

    panic!("badge not found!");
}

fn part1(input: &[u8]) -> usize {
    input
        .split(|&b| b == b'\n')
        .map(|line| {
            let (sack1, sack2) = (&line[..line.len() / 2], &line[line.len() / 2..]);
            priority(find_misplaced(sack1, sack2)) as usize
        })
        .sum()
}

fn part2(input: &[u8]) -> usize {
    let lines: Vec<_> = input.split(|&b| b == b'\n').collect();
    lines
        .chunks_exact(3)
        .map(|sacks| priority(find_badge(sacks[0], sacks[1], sacks[2])) as usize)
        .sum()
}

fn main() {
    let input = include_bytes!("../input.txt");
    dbg!(part1(input));
    dbg!(part2(input));
}
