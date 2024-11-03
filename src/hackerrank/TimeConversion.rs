use std::io::{self, BufRead};

fn timeConversion(s: &str) -> String {
    let period = &s[8..];
    let mut hours: i32 = s[0..2].parse().unwrap();
    let minutes = &s[3..5];
    let seconds = &s[6..8];

    if period == "AM" {
        if hours == 12 {
            hours = 0;
        }
    } else {
        if hours != 12 {
            hours += 12;
        }
    }

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let s = stdin_iterator.next().unwrap().unwrap();
    let result = timeConversion(&s);

    println!("{}", result);
}
