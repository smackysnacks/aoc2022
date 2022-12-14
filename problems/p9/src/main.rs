use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    times: usize,
}

fn parse_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let times = line[2..].parse().unwrap();
            match &line[0..1] {
                "U" => Move {
                    direction: Direction::U,
                    times,
                },
                "D" => Move {
                    direction: Direction::D,
                    times,
                },
                "L" => Move {
                    direction: Direction::L,
                    times,
                },
                "R" => Move {
                    direction: Direction::R,
                    times,
                },
                _ => panic!("unexpected direction"),
            }
        })
        .collect()
}

fn touching(a: (isize, isize), b: (isize, isize)) -> bool {
    let xd = a.0 - b.0;
    let yd = a.1 - b.1;
    xd.abs() <= 1 && yd.abs() <= 1
}

fn adjust(head: (isize, isize), mut tail: (isize, isize)) -> (isize, isize) {
    if touching(head, tail) {
        return tail;
    }

    if head.0 != tail.0 {
        let xoffset = head.0 - tail.0;

        if xoffset > 0 {
            tail.0 += 1;
        } else {
            tail.0 -= 1;
        }
    }

    if head.1 != tail.1 {
        let yoffset = head.1 - tail.1;
        if yoffset > 0 {
            tail.1 += 1;
        } else {
            tail.1 -= 1;
        }
    }

    tail
}

fn part1(moves: &[Move]) -> usize {
    let mut head: (isize, isize) = (0, 0);
    let mut tail: (isize, isize) = (0, 0);
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    for m in moves {
        for _ in 0..m.times {
            match m.direction {
                Direction::U => head.1 += 1,
                Direction::D => head.1 -= 1,
                Direction::L => head.0 -= 1,
                Direction::R => head.0 += 1,
            }

            tail = adjust(head, tail);
            visited.insert(tail);
        }
    }

    visited.len()
}

fn part2(moves: &[Move]) -> usize {
    let mut knots = vec![(0, 0); 10];
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    for m in moves {
        for _ in 0..m.times {
            match m.direction {
                Direction::U => knots[0].1 += 1,
                Direction::D => knots[0].1 -= 1,
                Direction::L => knots[0].0 -= 1,
                Direction::R => knots[0].0 += 1,
            }

            for i in 0..9 {
                knots[i + 1] = adjust(knots[i], knots[i + 1]);
            }

            visited.insert(knots[9]);
        }
    }

    visited.len()
}

fn main() {
    let input = include_str!("../input.txt");
    let moves = parse_input(input);
    dbg!(part1(&moves));
    dbg!(part2(&moves));
}
