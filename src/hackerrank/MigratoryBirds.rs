use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut bird_counts = HashMap::new();

    // Count the occurrences of each bird type
    for &bird in arr {
        *bird_counts.entry(bird).or_insert(0) += 1;
    }

    // Find the bird type with the maximum count
    let mut max_count = 0;
    let mut bird_id = -1;

    for (&bird, &count) in bird_counts.iter() {
        if count > max_count || (count == max_count && bird < bird_id) {
            max_count = count;
            bird_id = bird;
        }
    }

    bird_id
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
