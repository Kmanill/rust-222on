#[cfg(test)]
mod tests {
    struct Person {
        name: String,
        age: u8,
        hobby: String,
    }
    #[test]
    fn test1() {
        let age = 30;
        let p = Person {
            name: String::from("sunface"),
            age,
            hobby: String::from("coding"), // Provide a value for the hobby field
        };

        println!("Success!");
    }
    struct Unit;

    trait SomeTrait {
    }

    impl SomeTrait for Unit { }
    #[test]
    fn test2() {
        let u = Unit;
        do_something_with_unit(u);

        println!("Success!");
    }
    fn do_something_with_unit(u: Unit) { }

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
      #[test]
    fn test3() {
        let v = Color(0, 127, 255); // Create a Color instance


        println!("Success!");
    }

    struct Person1 {
        name: String,
        age: u8,
    }
   #[test]
    fn test5() {
        println!("Success!");
    }

    fn build_person(name: String, age: u8) -> Person1 {
        Person1 {
            name, // Fill in the blank with the `name` parameter
            age,
        }
    }

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
   #[test]
    fn test6() {
        let u1 = User {
            email: String::from("someone@example.com"),
            username: String::from("sunface"),
            active: true,
            sign_in_count: 1,
        };

        let u2 = set_email(u1);

        println!("Success!");
    }

    fn set_email(u: User) -> User {
        User {
            email: String::from("contact@im.dev"),
            // Fill the blank using struct update syntax
            active: u.active,
            username: u.username,
            sign_in_count: u.sign_in_count,
        }
    }

    #[derive(Debug)] // Fill the blank with the `Debug` trait
    struct Rectangle {
        width: u32,
        height: u32,
    }
   #[test]
    fn test7() {
        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale), // Print debug info to stderr and assign the value of `30 * scale` to `width`
            height: 50,
        };

        dbg!(&rect1); // Print debug info to stderr

        println!("{:?}", rect1); // Print debug info to stdout
    }

    #[derive(Debug)]
    struct File {
        name: String,
        data: String,
    }
   #[test]
    fn test8() {
        let f = File {
            name: String::from("readme.md"),
            data: "Rust By Practice".to_string(),
        };

        let _name = f.name.clone(); // Clone the name to avoid ownership issues

        // ONLY modify this line
        println!("{}, {}, {:?}", _name, f.data, f); // Use _name instead of f.name
    }








}