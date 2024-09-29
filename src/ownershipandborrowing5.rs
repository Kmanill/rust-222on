#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let x = String::from("Hello world");
        let y = x.clone();
        println!("{}, {}", x, y);
    }
    #[test]
    fn test2() {
        let s1 = String::from("Hello world");
        let s2 = take_ownership(s1);

        println!("{}", s2);
    }


    fn take_ownership(s: String) -> String {
        println!("{}", s);
        s
    }
    #[test]
    fn test3() {
        let s = give_ownership();
        println!("{}", s);
    }

    fn give_ownership() -> String {
        let s = String::from("Hello world");
        let _s_bytes = s.clone().into_bytes();
        s
    }
    #[test]
    fn test4() {
        let s = String::from("Hello World");

        print_str(&s);

        println!("{}", s);
    }

    fn print_str(s: &String)  {
        println!("{}", s);
    }
    #[test]
    fn test5() {
        let x = (1, 2, ());
        let y = x;
        println!("{:?}, {:?}", x, y);
    }
    #[test]
    fn test6() {
        let mut s = String::from("Hello ");

        let mut s1 = s;

        s1.push_str("World!");

        println!("Success!");
    }
    #[test]
    fn test7() {
        let mut x = Box::new(5); // Declare x as mutable

        let y = &mut *x;

        *y = 4;

        assert_eq!(*x, 4);

        println!("Success!");
    }
    #[test]
    fn test8() {
        let t = (String::from("hello"), String::from("world"));

        let s = &t.0;

        println!("{:?}", t);
    }
    #[test]
    fn test9() {
        let t = (String::from("hello"), String::from("world"));

        let (s1, s2) = (&t.0, &t.1);

        println!("{:?}, {:?}, {:?}", s1, s2, t);
    }
    #[test]
    fn test10() {
        let x = 5;
        let p = &x;

        println!("the memory address of x is {:p}", p);
    }
    #[test]
    fn test12() {
        let x = 5;
        let y = &x;

        assert_eq!(5, *y);

        println!("Success!");
    }
    #[test]
    fn test13() {
        let s = String::from("hello, ");

        borrow_object(&s);

        println!("Success!");
    }

    fn borrow_object(s: &String) {}
    #[test]
    fn test14() {
        let mut s = String::from("hello, ");

        push_str(&mut s);

        println!("Success! {}", s);
    }

    fn push_str(s: &mut String) {
        s.push_str("world");
    }
    #[test]
    fn test15() {
        let mut s = String::from("hello, ");


        let p = &mut s;

        p.push_str("world");

        println!("Success! {}", s);
    }
    #[test]
    fn test16() {
        let c = '中';

        let r1 = &c;

        let r2 = &c;

        assert_eq!(*r1, *r2);

        assert_eq!(get_addr(r1), get_addr(r2));

        println!("Success!");
    }

    fn get_addr(r: &char) -> String {
        format!("{:p}", r)
    }
    #[test]
    fn test17() {
        let mut s = String::from("hello");

        let r1 = &mut s;
        println!("{}", r1);


        {
            let _r1 = r1;
        }

        let r2 = &s;

        println!("{}, {}", r2, s);

        println!("Success!");
    }
    #[test]
    fn test18() {
        let mut s = String::from("hello, ");

        {
            let r1 = &mut s;
            r1.push_str("world");
        }

        let r2 = &mut s;
        r2.push_str("!");

        println!("{}", r2);
    }























}