//1
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("Success!");
}
//2
use std::str::FromStr;

fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let from_str = i32::from_str("20").unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!");
}
//3
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .map(|x| x.trim())
            .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}

fn main() {
    let p = "(3, 4)".parse::<Point>();
    assert_eq!(p.unwrap(), Point { x: 3, y: 4 });

    println!("Success!");
}
//4
fn foo() -> i32 {
    0
}

fn main() {
    let pointer = foo as *const ();
    let function = unsafe {
        std::mem::transmute::<*const (), fn() -> i32>(pointer)
    };
    assert_eq!(function(), 0);

    println!("Success!");
}
//5
fn main() {
    let raw_bytes = [0x78, 0x56, 0x34, 0x12];

    // Использование transmute
    let num = unsafe { std::mem::transmute::<[u8; 4], u32>(raw_bytes) };
    assert_eq!(num, 0x12345678);

    // Безопасная альтернатива
    let num = u32::from_ne_bytes(raw_bytes);
    assert_eq!(num, 0x12345678);

    let num = u32::from_le_bytes(raw_bytes);
    assert_eq!(num, 0x12345678);

    let num = u32::from_be_bytes(raw_bytes);
    assert_eq!(num, 0x78563412);

    println!("Success!");
}
