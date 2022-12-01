fn part1(input: &str) {
    let mut max = 0;
    let mut local_sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            max = std::cmp::max(max, local_sum);
            local_sum = 0;
        } else {
            let n = line.parse::<i32>().unwrap();
            local_sum += n;
        }
    }

    println!("part1: {max}");
}

fn part2(input: &str) {
    let mut calories = Vec::new();

    let mut local_sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            calories.push(local_sum);
            local_sum = 0;
        } else {
            let n = line.parse::<i32>().unwrap();
            local_sum += n;
        }
    }

    calories.sort();
    let sum: i32 = calories.iter().rev().take(3).sum();

    println!("part2: {sum}");
}

fn main() {
    let input = include_str!("../input.txt");
    part1(input);
    part2(input);
}
