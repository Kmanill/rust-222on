#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let arr = [1, 2, 3];
        let s1: &[i32] = &arr[0..2];

        let s2: &str = "hello, world";

        println!("Success!");
    }
    #[test]
    fn test2() {
        let arr: [char; 3] = ['中', '国', '人'];

        let slice = &arr[..2];

        assert!(std::mem::size_of_val(&slice) == std::mem::size_of::<&[char]>());

        println!("Success!");
    }
    #[test]
    fn test3() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let slice: &[i32] = &arr[1..4];
        assert_eq!(slice, &[2, 3, 4]);

        println!("Success!");
    }
    #[test]
    fn test4() {
        let s = String::from("hello");

        let slice1 = &s[0..2];
        let slice2 = &s[..2];

        assert_eq!(slice1, slice2);

        println!("Success!");
    }
    #[test]
    fn test5() {
        let s = "你好，世界";
        let slice = &s[0..3];

        assert!(slice == "你");

        println!("Success!");
    }
    #[test]
    fn test6() {
        let s = String::from("hello world");

        let letter = first_letter(&s);
        let mut s_mut = s.clone();
        s_mut.clear();

        println!("the first letter is: {}", letter);
    }

    fn first_letter(s: &str) -> &str {
        &s[..1]
    }






}
