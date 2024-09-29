#[cfg(test)]
mod tests {

    trait Hello {
        fn say_hi(&self) -> String {
            String::from("hi")
        }
        fn say_something(&self) -> String;
    }

    struct Student {}
    impl Hello for Student {
        fn say_something(&self) -> String {
            String::from("I'm a good student")
        }
    }

    struct Teacher {}
    impl Hello for Teacher {
        fn say_hi(&self) -> String {
            String::from("Hi, I'm your new teacher")
        }
        fn say_something(&self) -> String {
            String::from("I'm not a bad teacher")
        }
    }

    #[test]
    fn test_hello_trait() {
        let s = Student {};
        assert_eq!(s.say_hi(), "hi");
        assert_eq!(s.say_something(), "I'm a good student");

        let t = Teacher {};
        assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
        assert_eq!(t.say_something(), "I'm not a bad teacher");
    }


    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    #[derive(Debug)]
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            Centimeters(self.0 as f64 * 2.54)
        }
    }

    #[derive(Debug, PartialEq, PartialOrd)]
    struct Seconds(i32);

    #[test]
    fn test_derive_traits() {
        let one_second = Seconds(1);
        println!("One second looks like: {:?}", one_second);
        assert!(one_second == one_second);

        let foot = Inches(12);
        println!("One foot equals {:?}", foot);

        let meter = Centimeters(100.0);
        assert!(foot.to_centimeters() < meter);
    }


    use std::ops;

    #[derive(Debug, PartialEq)]
    struct FooBar;

    #[derive(Debug, PartialEq)]
    struct BarFoo;

    struct Foo;
    struct Bar;

    impl ops::Add<Bar> for Foo {
        type Output = FooBar;
        fn add(self, _rhs: Bar) -> FooBar {
            FooBar
        }
    }

    impl ops::Sub<Foo> for Bar {
        type Output = BarFoo;
        fn sub(self, _rhs: Foo) -> BarFoo {
            BarFoo
        }
    }

    #[test]
    fn test_overloaded_operators() {

        assert_eq!(Foo + Bar, FooBar);
        assert_eq!(Bar - Foo, BarFoo);
    }


    trait Summary {
        fn summarize(&self) -> String;
    }

    #[derive(Debug)]
    struct Post {
        title: String,
        author: String,
        content: String,
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("The author of post '{}' is {}", self.title, self.author)
        }
    }

    #[derive(Debug)]
    struct Weibo {
        username: String,
        content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{} published a weibo '{}'", self.username, self.content)
        }
    }

    fn summary(item: impl Summary) {
        println!("{}", item.summarize());
    }

    #[test]
    fn test_summary() {
        let post = Post {
            title: "Popular Rust".to_string(),
            author: "Sunface".to_string(),
            content: "Rust is awesome!".to_string(),
        };
        let weibo = Weibo {
            username: "sunface".to_string(),
            content: "Weibo seems to be worse than Tweet".to_string(),
        };

        summary(post);
        summary(weibo);
    }


    struct Sheep {}
    struct Cow {}

    trait Animal {
        fn noise(&self) -> String;
    }

    impl Animal for Sheep {
        fn noise(&self) -> String {
            "baaaaah!".to_string()
        }
    }

    impl Animal for Cow {
        fn noise(&self) -> String {
            "moooooo!".to_string()
        }
    }

    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }

    #[test]
    fn test_random_animal() {
        let animal = random_animal(0.3);
        assert_eq!(animal.noise(), "baaaaah!");

        let animal = random_animal(0.8);
        assert_eq!(animal.noise(), "moooooo!");
    }
}
