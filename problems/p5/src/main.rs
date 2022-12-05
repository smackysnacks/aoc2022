#[derive(Debug)]
struct Command {
    quantity: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Command>) {
    let parts = input.split("\n\n").collect::<Vec<_>>();

    let mut crates: Vec<Vec<char>> = Vec::new();
    for line in parts[0].lines().rev().skip(1) {
        let mut p = 0;

        for chunk in line.as_bytes().chunks(4) {
            if crates.len() <= p {
                crates.push(Vec::new());
            }
            if chunk[1] != b' ' {
                crates[p].push(chunk[1].into());
            }
            p += 1;
        }
    }

    let commands = parts[1]
        .lines()
        .map(|line| {
            let (quantity, from, to) =
                sscanf::sscanf!(line, "move {usize} from {usize} to {usize}").unwrap();
            Command {
                quantity,
                from: from - 1,
                to: to - 1,
            }
        })
        .collect();

    (crates, commands)
}

fn part1(mut crates: Vec<Vec<char>>, commands: &[Command]) -> String {
    for command in commands {
        for _ in 0..command.quantity {
            let c = crates[command.from].pop().unwrap();
            crates[command.to].push(c);
        }
    }

    crates.iter().map(|c| c.last().unwrap()).collect()
}

fn part2(mut crates: Vec<Vec<char>>, commands: &[Command]) -> String {
    for command in commands {
        let mut stack = Vec::new();
        for _ in 0..command.quantity {
            stack.push(crates[command.from].pop().unwrap());
        }
        for _ in 0..command.quantity {
            crates[command.to].push(stack.pop().unwrap());
        }
    }

    crates.iter().map(|c| c.last().unwrap()).collect()
}

fn main() {
    let input = include_str!("../input.txt");
    let (crates, commands) = parse_input(input);
    dbg!(part1(crates.clone(), &commands));
    dbg!(part2(crates, &commands));
}
