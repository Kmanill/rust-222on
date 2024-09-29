#[cfg(test)]
mod tests {
    enum Direction {
        East,
        West,
        North,
        South,
    }
    #[test]
    fn test1() {
        let dire = Direction::South;
        match dire {
            Direction::East => println!("East"),
            Direction::South | Direction::North => {
                println!("South or North");
            }
            _ => println!("West"),
        };
    }
    #[test]
    fn test2() {
        let boolean = true;


        let binary = match boolean {
            true => 1,
            false => 0,
        };

        assert_eq!(binary, 1);

        println!("Success!");
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    #[test]
    fn test3() {
        let msgs = [
            Message::Quit,
            Message::Move { x: 1, y: 3 },
            Message::ChangeColor(255, 255, 0),
        ];

        for msg in msgs {
            show_message(msg)
        }

        println!("Success!");
    }

    fn show_message(msg: Message) {
        match msg {
            Message::Move { x: a, y: b } => {
                assert_eq!(a, 1);
                assert_eq!(b, 3);
            }
            Message::ChangeColor(_, g, b) => {
                assert_eq!(g, 255);
                assert_eq!(b, 0);
            }
            Message::Quit | Message::Write(_) => println!("no data in these variants"),
        }
    }
    #[test]
    fn test4() {
        let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];


        for ab in alphabets {
            assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'));
        }
        println!("Success!");
    }
    #[test]
    fn test5() {
        let o = Some(7);


        if let Some(i) = o {
            println!("This is a really long string and `{:?}`", i);
            println!("Success!");
        }
    }
    enum Foo1 {
        Bar(u8),
    }
    #[test]
    fn test6() {
        let a = Foo1::Bar(1);

        if let Foo1::Bar(i) = a {
            println!("foobar holds the value: {}", i);
            println!("Success!");
        }
    }

    enum Foo2 {
        Bar,
        Baz,
        Qux(u32),
    }
    #[test]
    fn test7() {
        let a = Foo2::Qux(10);


        match a {
            Foo2::Bar => {
                println!("match foo::bar");
            }
            Foo2::Baz => {
                println!("match foo::baz");
            }
            _ => {
                println!("match others");
            }
        }
    }
    #[test]
    fn test8() {
        let age = Some(30);
        if let Some(age_value) = age {
            assert_eq!(age_value, 30);
        }

        match age {

            Some(age) => println!("age is a new variable, its value is {}", age),
            _ => (),
        }
    }
    #[test]
    fn test9() {
        match_number(1);
        match_number(2);
        match_number(7);
        match_number(11);
    }

    fn match_number(n: i32) {
        match n {

            1 => println!("One!"),

            2 | 3 | 4 | 5 => println!("match 2 -> 5"),

            6..=10 => {
                println!("match 6 -> 10")
            },
            _ => {
                println!("match -infinite -> 0 or 11 -> +infinite")
            }
        }
    }
    struct Point {
        x: i32,
        y: i32,
    }
    #[test]
    fn test10() {

        let p = Point { x: 3, y: 20 };

        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),

            Point { x: 0..=5, y: y @ (10 | 20 | 30) } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }
    enum Message2 {
        Hello { id: i32 },
    }
    #[test]
    fn test11() {
        let msg = Message2::Hello { id: 5 };

        match msg {
            Message2::Hello { id: id @ 3..=7 } => println!("Found an id in range [3, 7]: {}", id),
            Message2::Hello { id } if id == 10 || id == 11 || id == 12 => {
                println!("Found an id in another range [10, 12]: {}", id)
            }
            Message2::Hello { id } => println!("Found some other id: {}", id),
        }
    }
    #[test]
    fn test12() {
        let num = Some(4);
        let split = 5;
        match num {
            Some(x) if x < split => assert!(x < split),
            Some(x) => assert!(x >= split),
            None => (),
        }

        println!("Success!");
    }

    #[test]
    fn test13() {
        let numbers = Some((2, 3));

        match numbers {
            Some((first, _)) => {
                assert_eq!(first, 2);
            }
            None => (),
        }

        println!("Success!");
    }

    #[test]
    fn test14() {
        let mut v = String::from("hello,");
        let r = &mut v;

        match r {
            value => value.push_str(" world!"),
        }
    }






}