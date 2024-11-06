#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let arr: [u8; 3] = [1, 2, 3];
        let v = Vec::from(arr);
        is_vec(&v);

        let v = vec![1, 2, 3];
        is_vec(&v);

        let mut v1 = Vec::new();
        for &item in &arr {
            v1.push(item);
        }

        assert_eq!(v, v1);
        println!("Success!");
    }

    fn is_vec(v: &Vec<u8>) {}
    #[test]
    fn test2() {
        let mut v1 = Vec::from([1, 2, 4]);
        v1.pop();
        v1.push(3);

        let mut v2 = Vec::from([1, 2, 3]);

        assert_eq!(v1, v2);
        println!("Success!");
    }
    #[test]
    fn test3() {
        let arr = [1, 2, 3];
        let v1 = Vec::from(arr);
        let v2: Vec<i32> = arr.into();

        assert_eq!(v1, v2);

        let s = "hello".to_string();
        let v1: Vec<u8> = s.clone().into_bytes();

        let v3 = Vec::from("hello".as_bytes());
        assert_eq!(v1, v3);

        println!("Success!");
    }
    #[test]
    fn test4() {
        let mut vec = Vec::with_capacity(10);

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 10);

        for i in 0..10 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 10);
        assert_eq!(vec.capacity(), 10);

        vec.push(11);
        assert_eq!(vec.len(), 11);
        assert!(vec.capacity() >= 11);

        let mut vec = Vec::with_capacity(100);
        for i in 0..100 {
            vec.push(i);
        }

        assert_eq!(vec.len(), 100);
        assert_eq!(vec.capacity(), 100);

        println!("Success!");
    }




}