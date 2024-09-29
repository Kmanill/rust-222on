#[cfg(test)]
mod tests {
    struct A;
    struct S(A);
    struct SGen<T>(T);

    fn reg_fn(_s: S) {}
    fn gen_spec_t(_s: SGen<A>) {}
    fn gen_spec_i32(_s: SGen<i32>) {}
    fn generic<T>(_s: SGen<T>) {}

    #[test]
    fn test1() {
        reg_fn(S(A));
        gen_spec_t(SGen(A));
        gen_spec_i32(SGen(32));
        generic::<char>(SGen('a'));
        generic(SGen('b'));
    }

    fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }

    #[test]
    fn test2() {
        assert_eq!(5, sum(2i8, 3i8));
        assert_eq!(50, sum(20, 30));
        assert_eq!(2.46, sum(1.23, 1.23));
    }

    struct Point<T, U> {
        x: T,
        y: U,
    }

    #[test]
    fn test3() {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
        println!("Success!");
    }

    struct PointAnyType<T, U> {
        x: T,
        y: U,
    }

    #[test]
    fn test4() {
        let p = PointAnyType { x: 5, y: "hello".to_string() };
        println!("Success!");
    }

    struct Val<T> {
        val: T,
    }

    impl<T> Val<T> {
        fn value(&self) -> &T {
            &self.val
        }
    }

    #[test]
    fn test5() {
        let x = Val { val: 3.0 };
        let y = Val { val: "hello".to_string() };
        assert_eq!(*x.value(), 3.0);
        assert_eq!(y.value(), "hello");
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    #[test]
    fn test6() {
        let p1 = Point { x: 5, y: 10 };
        let p2 = Point { x: "Hello", y: '中' };
        let p3 = p1.mixup(p2);
        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, '中');
    }

    struct PointF32 {
        x: f32,
        y: f32,
    }

    impl PointF32 {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    #[test]
    fn test7() {
        let p = PointF32 { x: 5.0, y: 10.0 };
        assert_eq!(p.distance_from_origin(), (5.0f32.powi(2) + 10.0f32.powi(2)).sqrt());
    }
}
