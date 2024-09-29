#[cfg(test)]
mod tests {
    use std::fmt;

    // Задача 1: Associated Types
    struct Container(i32, i32);

    trait Contains {
        type A;
        type B;

        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        fn first(&self) -> Self::A;
        fn last(&self) -> Self::B;
    }

    impl Contains for Container {
        type A = i32;
        type B = i32;

        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }
        fn first(&self) -> i32 {
            self.0
        }
        fn last(&self) -> i32 {
            self.1
        }
    }

    use std::ops::Sub;

    fn difference<C: Contains>(container: &C) -> i32
    where
        <C as Contains>::B: Sub<<C as Contains>::A, Output = i32>,
    {
        container.last() - container.first()
    }

    #[test]
    fn test_contains() {
        let number_1 = 3;
        let number_2 = 10;
        let container = Container(number_1, number_2);

        assert!(container.contains(&number_1, &number_2));
        assert_eq!(container.first(), 3);
        assert_eq!(container.last(), 10);
        assert_eq!(difference(&container), 7);
    }


    #[derive(Debug, PartialEq)]
    struct Point<T = i32> {
        x: T,
        y: T,
    }

    impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Point<T> {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            Point {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    #[test]
    fn test_point_sub() {
        let p1 = Point { x: 2, y: 3 };
        let p2 = Point { x: 1, y: 0 };
        assert_eq!(p1 - p2, Point { x: 1, y: 3 });
    }


    trait Pilot {
        fn fly(&self) -> String;
    }

    trait Wizard {
        fn fly(&self) -> String;
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) -> String {
            String::from("This is your captain speaking.")
        }
    }

    impl Wizard for Human {
        fn fly(&self) -> String {
            String::from("Up!")
        }
    }

    impl Human {
        fn fly(&self) -> String {
            String::from("*waving arms furiously*")
        }
    }

    #[test]
    fn test_fully_qualified_syntax() {
        let person = Human;
        assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
        assert_eq!(Wizard::fly(&person), "Up!");
        assert_eq!(person.fly(), "*waving arms furiously*");
    }


    trait Person {
        fn name(&self) -> String;
    }

    trait Student: Person {
        fn university(&self) -> String;
    }

    trait Programmer {
        fn fav_language(&self) -> String;
    }

    trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }

    struct CSStudent {
        name: String,
        university: String,
        fav_language: String,
        git_username: String,
    }

    impl Person for CSStudent {
        fn name(&self) -> String {
            self.name.clone()
        }
    }

    impl Student for CSStudent {
        fn university(&self) -> String {
            self.university.clone()
        }
    }

    impl Programmer for CSStudent {
        fn fav_language(&self) -> String {
            self.fav_language.clone()
        }
    }

    impl CompSciStudent for CSStudent {
        fn git_username(&self) -> String {
            self.git_username.clone()
        }
    }

    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
        format!(
            "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username()
        )
    }

    #[test]
    fn test_supertraits() {
        let student = CSStudent {
            name: "Sunfei".to_string(),
            university: "XXX".to_string(),
            fav_language: "Rust".to_string(),
            git_username: "sunface".to_string(),
        };
        assert_eq!(
            comp_sci_student_greeting(&student),
            "My name is Sunfei and I attend XXX. My favorite language is Rust. My Git username is sunface"
        );
    }


    struct Pretty(String);

    impl fmt::Display for Pretty {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\"{}\"", self.0.clone() + ", world")
        }
    }

    #[test]
    fn test_pretty() {
        let w = Pretty("hello".to_string());
        assert_eq!(format!("{}", w), "\"hello, world\"");
    }
}
