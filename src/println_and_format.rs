fn main() {
    let s1 = "hello";
    let s = format!("{}, world!", s1);
    assert_eq!(s, "hello, world!");
    println!("Завдання 1: {}", s);

    print!("Hello world, ");
    println!("I am ");
    println!("Sunface!");
}