#[cfg(test)]
mod tests {

    trait Draw {
        fn draw(&self) -> String;
    }

    struct Circle;
    struct Square;

    impl Draw for Circle {
        fn draw(&self) -> String {
            String::from("Circle")
        }
    }

    impl Draw for Square {
        fn draw(&self) -> String {
            String::from("Square")
        }
    }

    fn create_shape(shape_type: &str) -> Box<dyn Draw> {
        match shape_type {
            "circle" => Box::new(Circle),
            "square" => Box::new(Square),
            _ => panic!("Unknown shape type"),
        }
    }

    #[test]
    fn test1() {
        let shape = create_shape("circle");
        assert_eq!(shape.draw(), "Circle");

        let shape = create_shape("square");
        assert_eq!(shape.draw(), "Square");
    }


    trait Animal {
        fn sound(&self) -> &'static str;
    }

    struct Dog;
    struct Cat;

    impl Animal for Dog {
        fn sound(&self) -> &'static str {
            "Bark"
        }
    }

    impl Animal for Cat {
        fn sound(&self) -> &'static str {
            "Meow"
        }
    }

    #[test]
    fn test2() {
        let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
        let sounds: Vec<&str> = animals.iter().map(|animal| animal.sound()).collect();
        assert_eq!(sounds, vec!["Bark", "Meow"]);
    }


    trait Shape {
        fn area(&self) -> f64;
    }

    struct Rectangle {
        width: f64,
        height: f64,
    }

    struct Triangle {
        base: f64,
        height: f64,
    }

    impl Shape for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    impl Shape for Triangle {
        fn area(&self) -> f64 {
            0.5 * self.base * self.height
        }
    }

    fn print_area(shape: &dyn Shape) {
        println!("The area is {}", shape.area());
    }

    #[test]
    fn test3() {
        let rect = Rectangle { width: 5.0, height: 10.0 };
        let tri = Triangle { base: 6.0, height: 4.0 };

        print_area(&rect);
        print_area(&tri);

        assert_eq!(rect.area(), 50.0);
        assert_eq!(tri.area(), 12.0);
    }


    trait Greet {
        fn greet(&self) -> &str;
    }

    struct Human;
    struct Robot;

    impl Greet for Human {
        fn greet(&self) -> &str {
            "Hello, Human!"
        }
    }

    impl Greet for Robot {
        fn greet(&self) -> &str {
            "Hello, Robot!"
        }
    }

    fn static_greet<T: Greet>(entity: &T) {
        println!("{}", entity.greet());
    }

    fn dynamic_greet(entity: &dyn Greet) {
        println!("{}", entity.greet());
    }

    #[test]
    fn test4() {
        let human = Human;
        let robot = Robot;

        static_greet(&human);
        static_greet(&robot);

        dynamic_greet(&human);
        dynamic_greet(&robot);

        assert_eq!(human.greet(), "Hello, Human!");
        assert_eq!(robot.greet(), "Hello, Robot!");
    }


    trait Person {
        fn name(&self) -> &str;
        fn age(&self) -> u32;
        fn info(&self) -> String {
            format!("Name: {}, Age: {}", self.name(), self.age())
        }
    }

    struct Student {
        student_name: String,
        student_age: u32,
    }

    impl Person for Student {
        fn name(&self) -> &str {
            &self.student_name
        }
        fn age(&self) -> u32 {
            self.student_age
        }
    }

    #[test]
    fn test5() {
        let student = Student {
            student_name: "Alice".to_string(),
            student_age: 20,
        };

        assert_eq!(student.info(), "Name: Alice, Age: 20");
    }
}
