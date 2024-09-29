#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let s: &str = "hello, world";
        println!("Success!");
    }
    #[test]
    fn test2() {
        let s: Box<str> = "hello, world".into();
        greetings_box(s);
    }

    fn greetings_box(s: Box<str>) {
        println!("{}", s);
    }

    fn greetings(s: &str) {
        println!("{}", s);
    }
    #[test]
    fn test3() {
        let mut s = String::new();
        s.push_str("hello, world");
        s.push('!');

        assert_eq!(s, "hello, world!");

        println!("Success!");
    }
    #[test]
    fn test4() {
        let mut s = String::from("hello");
        s.push(',');
        s.push_str(" world");
        s += "!";

        println!("{}", s);
    }
    #[test]
    fn test5() {
        let s = String::from("I like dogs");
        let s1 = s.replace("dogs", "cats");

        assert_eq!(s1, "I like cats");

        println!("Success!");
    }
    #[test]
    fn test6() {
        let s1 = String::from("hello,");
        let s2 = String::from("world!");
        let s3 = s1.clone() + &s2;
        assert_eq!(s3, "hello,world!");
        println!("{}", s1);
    }
    #[test]
    fn test7() {
        let s1 = "hello, world";
        greetings(s1);

        let s2 = String::from("hello!");
        greetings(s2.as_str());
    }
    #[test]
    fn test8() {
        let s = "hello, world".to_string();
        let s1: &str = &s;

        println!("Success!");
    }
    #[test]
    fn test9() {
        let byte_escape = "I'm writing Ru\x73\x74!";
        println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

        let unicode_codepoint = "\u{211D}";
        let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

        println!("Unicode character {} (U+211D) is called {}",
                 unicode_codepoint, character_name );

        let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
        println!("{}", long_string);
    }
    #[test]
    fn test10() {
        let s1 = String::from("hi,中国");

        let h = s1.chars().nth(0).unwrap();
        assert_eq!(h, 'h');

        let h1: String = s1.chars().skip(3).take(1).collect();
        assert_eq!(h1, "中");

        println!("Success!");
    }
    #[test]
    fn test12() {
        for c in "你好，世界".chars() {
            println!("{}", c);
        }
    }


























}