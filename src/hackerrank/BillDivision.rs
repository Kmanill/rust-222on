use std::io::{self, BufRead};

/*
 * Complete the 'bonAppetit' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY bill
 *  2. INTEGER k
 *  3. INTEGER b
 */

fn bonAppetit(bill: &[i32], k: usize, b: i32) {
    // Calculate the total bill without the item k
    let total_bill: i32 = bill.iter().enumerate()
        .filter(|&(i, _)| i != k) // Exclude the k-th item
        .map(|(_, &cost)| cost) // Get the cost of the remaining items
        .sum();

    // Calculate Anna's share
    let anna_share = total_bill / 2;

    // Check if Anna was overcharged
    if b > anna_share {
        println!("{}", b - anna_share); // Output the difference
    } else {
        println!("Bon Appetit"); // Correct amount paid
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let k = first_multiple_input[1].trim().parse::<usize>().unwrap(); // Change to usize for indexing

    let bill: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let b = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    bonAppetit(&bill, k, b);
}
