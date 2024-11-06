//1
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        panic!("Oh no! Not lemonade!");
    }

    println!("Exercise Failed if printing out this line!");
}

fn main() {
    drink("lemonade");
}
//2
fn main() {
    // Исправляем ассерцию
    assert_eq!("abc".as_bytes(), [97, 98, 99]);

    let v = vec![1, 2, 3];
    // Используем безопасный доступ
    let ele = v.get(3);
    if ele.is_none() {
        println!("Index out of bounds");
    }

    // Обработка деления на ноль
    let result = divide(15, 0);
    if let Some(value) = result {
        println!("Result: {}", value);
    } else {
        println!("Cannot divide by zero");
    }

    println!("Success!")
}

fn divide(x: u8, y: u8) -> Option<u8> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

