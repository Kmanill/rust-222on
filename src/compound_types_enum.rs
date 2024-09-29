#[cfg(test)]
mod tests {
    enum Number {
        Zero,
        One,
        Two,
    }

    enum Number1 {
        Zero = 0,
        One = 1,
        Two = 2,
    }

    // C-like enum
    enum Number2 {
        Zero = 0,
        One = 1,
        Two = 2,
    }
   #[test]
    fn test1() {
        assert_eq!(Number1::One as i32, 1);
        assert_eq!(Number2::One as i32, 1);

        println!("Success!");
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    #[test]
    fn test2() {
        let msg1 = Message::Move { x: 1, y: 2 };
        let msg2 = Message::Write(String::from("hello, world!"));

        println!("Success!");
    }

    enum Message1 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
   #[test]
    fn test3() {
        let msg = Message1::Move { x: 1, y: 2 };

        if let Message1::Move { x: a, y: b } = msg {
            assert_eq!(a, 1);
            assert_eq!(b, 2);
        } else {
            panic!("NEVER LET THIS RUN！");
        }

        println!("Success!");
    }
   #[test]
    fn test4() {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        if let Some(n) = six {
            println!("{}", n);

            println!("Success!");
            return;
        }

        panic!("NEVER LET THIS RUN！");
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    fn main() {
        test4();
    }


    enum List {

        Cons(u32, Box<List>),

        Nil,
    }


    impl List {
        // Create an empty list
        fn new() -> List {
            // `Nil` has type `List`
            List::Nil
        }


        fn prepend(self, elem: u32) -> List {
            // `Cons` also has type List
            List::Cons(elem, Box::new(self))
        }


        fn len(&self) -> u32 {
            match *self {
                List::Cons(_, ref tail) => 1 + tail.len(),
                List::Nil => 0,
            }
        }


        fn stringify(&self) -> String {
            match *self {
                List::Cons(head, ref tail) => {
                    format!("{}, {}", head, tail.stringify())
                },
                List::Nil => {
                    format!("Nil")
                },
            }
        }
    }


    fn test_linked_list() {
        // Create an empty linked list
        let mut list = List::new();


        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);


        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());
    }

    // Unit test
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_list() {
            let mut list = List::new();
            list = list.prepend(1);
            list = list.prepend(2);
            list = list.prepend(3);

            assert_eq!(list.len(), 3);
            assert_eq!(list.stringify(), "3, 2, 1, Nil");
        }
    }

    fn main1() {
        test_linked_list();
    }





}