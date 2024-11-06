#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let decimal = 97.123_f32;

        let integer: u8 = decimal as u8;

        let c2 = integer as char;

        assert_eq!(integer, 'a' as u8);

        println!("Success!");
    }
    #[test]
    fn test2() {
        assert_eq!(u8::MAX, 255);

        let v = 1000;

        println!("Success! v = {}", v);
    }
    #[test]
    fn test3() {
        let mut values: [i32; 2] = [1, 2];
        let p1: *mut i32 = values.as_mut_ptr();
        let first_address: usize = p1 as usize;
        let second_address = first_address + 4;
        let p2: *mut i32 = second_address as *mut i32;

        unsafe {
            *p2 += 1;
        }

        assert_eq!(values[1], 3);

        println!("Success!");
    }
    fn main() {
        let arr: [u64; 13] = [0; 13];
        assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
        let a: *const [u64] = &arr;
        let b = a as *const [u8];
        unsafe {
            assert_eq!(std::mem::size_of_val(&*b), 104);
        }

        println!("Success!");
    }





}