struct Inst {
    adjustment: isize,
    clocks: usize,
}

fn parse_input(input: &str) -> Vec<Inst> {
    input
        .lines()
        .map(|line| {
            if line == "noop" {
                return Inst {
                    adjustment: 0,
                    clocks: 1,
                };
            } else {
                return Inst {
                    adjustment: line[5..].parse().unwrap(),
                    clocks: 2,
                };
            }
        })
        .collect()
}

fn part1(instructions: &[Inst]) -> isize {
    let mut x = 1;
    let mut cycle = 0;
    let mut sum = 0;

    for inst in instructions {
        for _ in 0..inst.clocks {
            cycle += 1;
            if cycle == 20
                || cycle == 60
                || cycle == 100
                || cycle == 140
                || cycle == 180
                || cycle == 220
            {
                sum += cycle * x;
            }
        }
        x += inst.adjustment;
    }

    sum
}

fn part2(instructions: &[Inst]) {
    let mut x = 1;
    let mut cycle = 0;

    for inst in instructions {
        for _ in 0..inst.clocks {
            if x - 1 <= cycle && cycle <= x + 1 {
                print!("#");
            } else {
                print!(".");
            }
            cycle += 1;
            if cycle == 40 {
                cycle = 0;
                println!();
            }
        }
        x += inst.adjustment;
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let instructions = parse_input(input);
    dbg!(part1(&instructions));
    part2(&instructions);
}
