#[cfg(test)]
mod tests {
    use std::ops::{Range, RangeInclusive};
    use std::mem::size_of_val;

    #[test]
    fn test1() {
        let x: i32 = 5;
        let z = 10;
        println!("Success!");
    }

    #[test]
    fn test2() {
        let v: u16 = 38_u8 as u16;
        println!("Success!");
    }

    #[test]
    fn test3() {
        let x = 5;
        assert_eq!("i32".to_string(), type_of(&x));
        println!("Success!");
    }

    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }

    #[test]
    fn test4() {
        assert_eq!(i8::MAX, 127);
        assert_eq!(u8::MAX, 255);
        println!("Success!");
    }

    #[test]
    fn test5() {
        let v1 = 251_u16 + 8;
        let v2 = u8::checked_add(251, 8);

        match v2 {
            Some(value) => println!("{}, {}", v1, value),
            None => println!("Overflow occurred when adding 251 and 8 as u8!"),
        }
    }

    #[test]
    fn test6() {
        let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
        assert_eq!(v, 1597);
        println!("Success!");
    }

    #[test]
    fn test7() {
        let x = 1_000.000_1;
        let y: f32 = 0.12;
        let z = 0.01_f64;

        assert_eq!(type_of(&x), "f64".to_string());
        println!("Success!");
    }

    #[test]
    fn test8() {
        let epsilon = f64::EPSILON;
        assert!((0.1_f64 + 0.2_f64 - 0.3_f64).abs() < epsilon);
        println!("Success!");
    }

    #[test]
    fn test9() {
        let mut sum = 0;
        for i in -3..2 {
            sum += i;
        }

        assert_eq!(sum, -5);
        for c in 'a'..='z' {
            print!("{} ", c as u32);
        }
    }

    #[test]
    fn test10() {
        assert_eq!((1..5), Range { start: 1, end: 5 });
        assert_eq!((1..=5), RangeInclusive::new(1, 5));
        println!("Success!");
    }

    #[test]
    fn test11() {
        assert_eq!(1u32 + 2, 3);
        assert_eq!(1i32 - 2, -1);
        assert_eq!(3 * 50, 150);
        let result = 9.6_f64 / 3.2_f64;
        assert!((result - 3.0_f64).abs() < 1e-10, "Expected {}, got {}", 3.0_f64, result);
        assert_eq!(24 % 5, 4);
        assert!(true && false == false);
        assert!(true || false == true);
        assert!(!true == false);

        let and_result = 0b0011u32 & 0b0101;
        let or_result = 0b0011u32 | 0b0101;
        let xor_result = 0b0011u32 ^ 0b0101;
        let left_shift_result = 1u32 << 5;
        let right_shift_result = 0x80u32 >> 2;

        println!("0011 AND 0101 is {:04b}", and_result);
        println!("0011 OR 0101 is {:04b}", or_result);
        println!("0011 XOR 0101 is {:04b}", xor_result);
        println!("1 << 5 is {}", left_shift_result);
        println!("0x80 >> 2 is 0x{:x}", right_shift_result);
    }

    #[test]
    fn test12() {
        let c1 = 'a';
        assert_eq!(size_of_val(&c1), 4);
        let c2 = '中';
        assert_eq!(size_of_val(&c2), 4);
        println!("Success!");
    }

    #[test]
    fn test13() {
        let c1 = '中';
        print_char(c1);
    }

    fn print_char(c: char) {
        println!("{}", c);
    }

    #[test]
    fn test14() {
        let t = true;
        if t {
            println!("Success!");
        }
    }

    #[test]
    fn test15() {
        let f = false;
        let t = true && false;
        assert_eq!(t, f);
        println!("Success!");
    }

    #[test]
    fn test16() {
        let v: () = ();
        assert_eq!(v, implicitly_ret_unit());
        println!("Success!");
    }

    fn implicitly_ret_unit() {
        println!("I will return a ()");
    }

    #[test]
    fn test17() {
        let unit: () = ();
        assert!(size_of_val(&unit) == 0);
        println!("Success!");
    }

    #[test]
    fn test18() {
        let x = 5u32;

        let y = {
            let x_squared = x * x;
            let x_cube = x_squared * x;
            x_cube + x_squared + x
        };

        let z = 2 * x;

        println!("x is {:?}", x);
        println!("y is {:?}", y);
        println!("z is {:?}", z);
    }

    #[test]
    fn test19() {
        let v = {
            let mut x = 1;
            x += 2;
            x
        };

        assert_eq!(v, 3);

        println!("Success!");
    }

    #[test]
    fn test20() {
        let v = 3;
        assert!(v == 3);
        println!("Success!");
    }

    #[test]
    fn test21() {
        let s = sum(1, 2);
        assert_eq!(s, 3);
        println!("Success!");
    }

    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }

    #[test]
    fn test22() {
        let (x, y) = (1, 2);
        let s = sum(x, y);
        assert_eq!(s, 3);
        println!("Success!");
    }

    #[test]
    fn test23() {
        print();
    }

    fn print() {
        println!("Success!");
    }

    #[test]
    fn test25() {
        println!("Success!");
    }

    fn get_option(tp: u8) -> Option<i32> {
        match tp {
            1 => Some(42),
            _ => None,
        }
    }

}
