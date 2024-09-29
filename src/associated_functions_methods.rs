#[cfg(test)]
mod tests {

    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    #[test]
    fn test1() {
        let rect1 = Rectangle { width: 30, height: 50 };
        assert_eq!(rect1.area(), 1500);
    }


    #[derive(Debug)]
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        pub fn show_state(&self) {
            println!("the current state is {}", self.color);
        }
    }

    #[test]
    fn test2() {
        let light = TrafficLight { color: "red".to_owned() };
        light.show_state();
    }


    impl TrafficLight {
        pub fn new() -> Self {
            Self { color: "red".to_string() }
        }

        pub fn get_state(&self) -> &str {
            &self.color
        }
    }

    #[test]
    fn test3() {
        let light = TrafficLight::new();
        assert_eq!(light.get_state(), "red");
    }


    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    #[test]
    fn test4() {
        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 10, height: 40 };
        assert!(rect1.can_hold(&rect2));
    }


    #[derive(Debug)]
    enum TrafficLightColor {
        Red,
        Yellow,
        Green,
    }

    impl TrafficLightColor {
        fn color(&self) -> &str {
            match self {
                TrafficLightColor::Red => "red",
                TrafficLightColor::Yellow => "yellow",
                TrafficLightColor::Green => "green",
            }
        }
    }

    #[test]
    fn test5() {
        let c = TrafficLightColor::Yellow;
        assert_eq!(c.color(), "yellow");
    }
}
