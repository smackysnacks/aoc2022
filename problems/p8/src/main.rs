#[derive(Default)]
struct Tree {
    value: i8,
    tallest_left: Option<i8>,
    tallest_right: Option<i8>,
    tallest_up: Option<i8>,
    tallest_down: Option<i8>,
}

fn build_trees(input: &str) -> Vec<Vec<Tree>> {
    let mut trees: Vec<Vec<Tree>> = Vec::new();
    for line in input.lines() {
        let row: Vec<_> = line
            .chars()
            .map(|c| {
                let value = c.to_digit(10).unwrap() as i8;
                Tree {
                    value,
                    ..Default::default()
                }
            })
            .collect();
        trees.push(row);
    }
    trees
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(mut input: Vec<Vec<Tree>>) -> usize {
    fn calculate(input: &mut Vec<Vec<Tree>>, direction: Direction, row: isize, col: isize) -> i8 {
        // out of bounds / no tree here
        if row as usize == input.len() || row < 0 || col as usize == input[0].len() || col < 0 {
            return -1;
        }

        let tallest_seen = match direction {
            Direction::Up => calculate(input, direction, row - 1, col),
            Direction::Down => calculate(input, direction, row + 1, col),
            Direction::Left => calculate(input, direction, row, col - 1),
            Direction::Right => calculate(input, direction, row, col + 1),
        };
        match direction {
            Direction::Up => input[row as usize][col as usize].tallest_up = Some(tallest_seen),
            Direction::Down => input[row as usize][col as usize].tallest_down = Some(tallest_seen),
            Direction::Left => input[row as usize][col as usize].tallest_left = Some(tallest_seen),
            Direction::Right => {
                input[row as usize][col as usize].tallest_right = Some(tallest_seen)
            }
        }

        std::cmp::max(tallest_seen, input[row as usize][col as usize].value)
    }

    // preprocess forest
    let last_row_idx = input.len() - 1;
    let last_col_idx = input[0].len() - 1;
    for col in 0..input[0].len() {
        calculate(&mut input, Direction::Down, 0, col as isize);
        calculate(
            &mut input,
            Direction::Up,
            last_row_idx as isize,
            col as isize,
        );
    }
    for row in 0..input.len() {
        calculate(&mut input, Direction::Right, row as isize, 0);
        calculate(
            &mut input,
            Direction::Left,
            row as isize,
            last_col_idx as isize,
        );
    }

    let mut total_visible = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let t = &input[i][j];
            if t.value > t.tallest_down.unwrap()
                || t.value > t.tallest_up.unwrap()
                || t.value > t.tallest_left.unwrap()
                || t.value > t.tallest_right.unwrap()
            {
                total_visible += 1;
            }
        }
    }

    total_visible
}

fn part2(input: &Vec<Vec<Tree>>) -> usize {
    fn visit(
        input: &Vec<Vec<Tree>>,
        consider: i8,
        direction: Direction,
        row: isize,
        col: isize,
    ) -> usize {
        // out of bounds / no tree here
        if row as usize == input.len() || row < 0 || col as usize == input[0].len() || col < 0 {
            return 0;
        }

        if input[row as usize][col as usize].value >= consider {
            return 1;
        }

        1 + match direction {
            Direction::Up => visit(input, consider, direction, row - 1, col),
            Direction::Down => visit(input, consider, direction, row + 1, col),
            Direction::Left => visit(input, consider, direction, row, col - 1),
            Direction::Right => visit(input, consider, direction, row, col + 1),
        }
    }

    let mut best_scenic = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let scenic_up = visit(
                &input,
                input[i][j].value,
                Direction::Up,
                i as isize - 1,
                j as isize,
            );
            let scenic_down = visit(
                &input,
                input[i][j].value,
                Direction::Down,
                i as isize + 1,
                j as isize,
            );
            let scenic_left = visit(
                &input,
                input[i][j].value,
                Direction::Left,
                i as isize,
                j as isize - 1,
            );
            let scenic_right = visit(
                &input,
                input[i][j].value,
                Direction::Right,
                i as isize,
                j as isize + 1,
            );

            best_scenic = std::cmp::max(
                best_scenic,
                scenic_up * scenic_down * scenic_left * scenic_right,
            );
        }
    }

    best_scenic
}

fn main() {
    let input = include_str!("../input.txt");
    let trees = build_trees(input);
    dbg!(part2(&trees));
    dbg!(part1(trees));
}
