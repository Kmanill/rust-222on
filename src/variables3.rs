#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let x: i32 = 5;
        let _y: i32;
        assert_eq!(x, 5);
        println!("Success!");
    }

    #[test]
    fn test2() {
        let x: i32 = 5;
        let y: i32 = 10;
        let real: i32 = x + y;
        let expected: i32 = 15;
        assert_eq!(real, expected);
        println!("Success!");
    }

    #[test]
    fn test3() {
        let x: i32 = 10;
        {
            let y: i32 = 5;
            assert_eq!(x, 10);
            assert_eq!(y, 5);
            println!("The value of x is {} and value of y is {}", x, y);
        }

        assert_eq!(x, 10);
        println!("The value of x is {}", x);
    }
    #[test]
    fn test4() {let x = define_x();
        println!("{}, world", x);}

    fn define_x() -> &'static str {
        let x = "hello";
        x
    }
    #[test]
    fn test5() {
        let x: i32 = 5;
        {
            let x = 12;
            assert_eq!(x, 12);
        }

        assert_eq!(x, 5);

        let x = 42;
        println!("{}", x);
    }
    #[test]
    fn test6() {
        let mut x: i32 = 1;
        x = 7;
        let x = x;

        let y = 4;

        let y = "I can also be bound to text!";

        println!("Success!");
    }
    #[test]
    fn test7() {
        let x = 1;
        println!("{}", x);
    }
    #[test]
    fn test8() {
        let (mut x, y) = (1, 2); // Робимо x змінною
        x += 2;

        assert_eq!(x, 3);
        assert_eq!(y, 2);

        println!("Success!");
    }
    #[test]
    fn test9() {
        let (x, y);
        (x, ..) = (3, 4);
        [.., y] = [1, 2];

        assert_eq!([x, y], [3, 2]);

        println!("Success!");
    }



}
