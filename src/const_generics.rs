#[cfg(test)]
mod tests {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    struct Array<T, const N: usize> {
        data: [T; N],
    }

    struct Point<T, U> {
        x: T,
        y: U,
    }

    struct PointAnyType<T, U> {
        x: T,
        y: U,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl<T> Array<T, 3> {
        fn new(data: [T; 3]) -> Self {
            Array { data }
        }
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
    fn test1() {
        let rect1 = Rectangle { width: 30, height: 50 };
        assert_eq!(rect1.area(), 1500);
    }

    #[test]
    fn test2() {
        let _integer_array = Array::new([1, 2, 3]);
        let _float_array = Array::new([1.0, 2.0, 3.0]);
        println!("Success!");
    }

    #[test]
    fn test3() {
        let _p = PointAnyType { x: 5, y: "hello".to_string() };
        println!("Success!");
    }

    #[test]
    fn test4() {
        let c = TrafficLightColor::Yellow;
        assert_eq!(c.color(), "yellow");
    }
}
