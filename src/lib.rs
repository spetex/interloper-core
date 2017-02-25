pub mod calculations {
    pub fn add(a: i32, b: i32) -> i32 {
        println!("Calculating...");
        return a + b;
    }
}

pub mod datatypes {
    use std::fmt;

    impl fmt::Display for Coordinates {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}:{}:{}", self.x, self.y, self.z)
        }
    }

    pub struct Coordinates {
        pub x: i32,
        pub y: i32,
        pub z: i32,
    }
}

mod tests;
