pub mod test_mod {
    use std::fmt;

    #[derive(Debug)]
    pub struct TestObj {
        pub a: u32,
        pub b: u32
    }

    impl TestObj {
        pub fn new(a: u32, b: u32) -> TestObj {
            if a > 100 {
                panic!("a can't over 100");
            }

            if b > 100 {
                panic!("b can't over 100");
            }

            TestObj { a, b }
        }
    }

    impl fmt::Display for TestObj {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "a: {}, b: {}", self.a, self.b)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_new() {
            let obj = TestObj::new(10, 20);
            println!("test obj : {:?}, {}", obj, obj);

            assert_eq!(obj.a, 10);
            assert_eq!(obj.b, 20);
        }

        #[test]
        fn test_display() {
            let obj = TestObj::new(1, 2);
            let fmt: String = format!{"{}", obj};

            assert_eq!("a: 1, b: 2", fmt);
        }
    }
}
