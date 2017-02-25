pub mod datatypes {
    use std::fmt;

    impl fmt::Display for Coordinates {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{},{},{}]", self.x, self.y, self.z)
        }
    }

    pub struct Coordinates {
        // Standard datatype for saving position of object in space
        pub x: i32,
        pub y: i32,
        pub z: i32,
    }
}

pub mod calculations {
    use datatypes::Coordinates;

    pub fn add(a: i32, b: i32) -> i32 {
        // Example function...
        println!("Calculating...");
        return a + b;
    }

    pub fn get_distance(point1: Coordinates, point2: Coordinates) -> f32 {
        // Calculates distance between 2 points in the universe based on coordinates
        let diff_x: i32 = point2.x - point1.x;
        let diff_y: i32 = point2.y - point1.y;
        let diff_z: i32 = point2.z - point1.z;
        let squared_x: i32 = diff_x * diff_x;
        let squared_y: i32 = diff_y * diff_y;
        let squared_z: i32 = diff_z * diff_z;
        let sum_int: i32 = squared_x + squared_y + squared_z;
        let sum_float: f32 = sum_int as f32;
        let res: f32 = sum_float.sqrt();
        return res;
    }
}

mod tests;
