//1
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>()?;
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}

fn main() {
    let result = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result = multiply("t", "2");
    assert!(result.is_err());

    println!("Success!");
}
//2
use std::fs::File;
use std::io::{self, Read};

fn read_file2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
//3
use std::num::ParseIntError;

fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().map(|n| n + 2)
}

fn main() {
    assert_eq!(add_two("4").unwrap(), 6);
    println!("Success!");
}
//4
use std::num::ParseIntError;

type Res<T> = Result<T, ParseIntError>;

fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn main() {
    println!("{:?}", multiply("10", "2"));
    println!("{:?}", multiply("t", "2"));
}
//5
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = number_str.parse::<i32>()?;
    println!("{}", number);
    Ok(())
}
