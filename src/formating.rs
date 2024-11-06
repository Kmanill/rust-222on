fn main() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // => Alice, this is Bob. Bob, this is Alice
    assert_eq!(format!("{1}{0}", 1, 2), "21");
    assert_eq!(format!("{1}{0}{0}{1}", 1, 2), "2112");
    println!("Success!");

    println!("{argument}", argument = "test"); // => "test"
    assert_eq!(format!("{name}{}", 1, name = 2), "21");
    assert_eq!(format!("{a} {c} {b}", a = "a", b = 'b', c = 3), "a 3 b");
    // Corrected position of named argument
    println!("{abc} {1}", 2, abc = "def");
    println!("Success!");

    println!("Hello {:5}!", "x"); // => "Hello x    !"
    println!("Hello {:1$}!", "x", 5); // => "Hello x    !"
    assert_eq!(format!("Hello {:5}!", "x"), "Hello x    !");
    assert_eq!(format!("Hello {width$}!", "x", width = 5), "Hello x    !");
    println!("Success!");

    // Left align
    println!("Hello {:<5}!", "x"); // => Hello x    !
    // Right align
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
    // Center align
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");
    // Left align, pad with '&'
    assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&&!");
    println!("Success!");

    println!("Hello {:5}!", 5); // => Hello     5!
    println!("Hello {:+}!", 5); // =>  Hello +5!
    println!("Hello {:05}!", 5); // => Hello 00005!
    println!("Hello {:05}!", -5); // => Hello -0005!
    assert!(format!("{number:0>width$}", number = 1, width = 6) == "000001");
    println!("Success!");

    let v = 3.1415926;
    println!("{:.1$}", v, 4); // => 3.1416
    assert_eq!(format!("{:.2}", v), "3.14");
    assert_eq!(format!("{:+.2}", v), "+3.14");
    assert_eq!(format!("{:.0}", v), "3");
    println!("Success!");

    let s = "Hello, world!";
    println!("{0:.5}", s); // => Hello
    assert_eq!(format!("Hello {:.3}!", "abcdefg"), "Hello abc!");
    println!("Success!");

    // Завдання 8: Binary, octal, hex
    assert_eq!(format!("{:#b}", 27), "0b11011");
    assert_eq!(format!("{:#o}", 27), "0o33");
    assert_eq!(format!("{:#x}", 27), "0x1b");
    assert_eq!(format!("{:#X}", 27), "0x1B");
    println!("{:x}!", 27); // => 1b
    println!("{:#010b}", 27); // => 0b00011011
    println!("Success!");

    // Завдання 9: Capture the environment
    fn get_person() -> String {
        String::from("sunface")
    }

    fn get_format() -> (usize, usize) {
        (4, 1)
    }

    let person = get_person();
    println!("Hello, {person}!");

    let (width, precision) = get_format();
    let scores = [("sunface", 99.12), ("jack", 60.34)];
    for (name, score) in scores {
        println!("{name}: {score:.1$}", score, precision);
    }
}