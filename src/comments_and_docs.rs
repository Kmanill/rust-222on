/// ```
/// let arg = 5;
/// let answer = add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// ```
/// let arg = 5;
/// let answer = add_two(arg);
///
/// assert_eq!(7, answer);
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}

/// ```rust,should_panic
/// div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}

/// ```
/// # fn try_main() -> Result<(), String> {
/// # let res = try_div(10, 0)?;
/// # Ok(()) // повертаємо з try_main
/// # }
/// # fn main() {
/// #    try_main().unwrap();
/// #
/// # }
/// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {

    let result_add_one = add_one(5);
    println!("add_one(5) = {}", result_add_one);

    let result_add_two = add_two(5);
    println!("add_two(5) = {}", result_add_two);

    let result_div = div(10, 2);
    println!("div(10, 2) = {}", result_div);

    match try_div(10, 0) {
        Ok(result) => println!("try_div(10, 0) = {}", result),
        Err(e) => println!("try_div(10, 0) помилка: {}", e),
    }
}