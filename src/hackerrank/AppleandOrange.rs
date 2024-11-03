use std::io::{self, BufRead};

fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = apples.iter()
        .filter(|&&distance| {
            let landing_position = a + distance;
            landing_position >= s && landing_position <= t
        })
        .count();

    let orange_count = oranges.iter()
        .filter(|&&distance| {
            let landing_position = b + distance;
            landing_position >= s && landing_position <= t
        })
        .count();

    println!("{}", apple_count);
    println!("{}", orange_count);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (s, t) = (first_multiple_input[0], first_multiple_input[1]);

    let second_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (a, b) = (second_multiple_input[0], second_multiple_input[1]);

    let third_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (m, n) = (third_multiple_input[0], third_multiple_input[1]);

    let apples: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let oranges: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    countApplesAndOranges(s, t, a, b, &apples, &oranges);
}
