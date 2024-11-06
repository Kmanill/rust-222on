//1
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }

        pub fn seat_at_table() -> String {
            String::from("sit down please")
        }
    }

    pub mod serving {
        pub fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}
        fn complain() {} // private
    }
}
//2
pub fn eat_at_restaurant() {
    // Вызов с использованием абсолютного пути
    crate::front_of_house::hosting::add_to_waitlist();

    // Вызов с использованием относительного пути
    front_of_house::hosting::add_to_waitlist();
}
//3
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order(); // Вызов через `super`
    }

    fn cook_order() {}
}
//4
.
├── Cargo.toml
├── src
│   ├── back_of_house.rs
│   ├── front_of_house
│   │   ├── hosting.rs
│   │   ├── mod.rs
│   │   └── serving.rs
│   ├── lib.rs
│   └── main.rs
//
pub mod hosting;
pub mod serving;
//
pub fn add_to_waitlist() {}
pub fn seat_at_table() -> String {
    String::from("sit down please")
}
//
pub fn take_order() {}
pub fn serve_order() {}
pub fn take_payment() {}
fn complain() {}
//
pub fn fix_incorrect_order() {
    cook_order();
    crate::front_of_house::serving::serve_order();
}

fn cook_order() {}
//5
fn main() {
    assert_eq!(crate::front_of_house::hosting::seat_at_table(), "sit down please");
    assert_eq!(crate::eat_at_restaurant(), "yummy yummy!");
    println!("Success!");
}

