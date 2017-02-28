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

    impl fmt::Display for Orbit {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} {} {} {} {} {}", self.i, self.o, self.e, self.w, self.v, self.a)
        }
    }
    pub struct Orbit {
        // Standard datatype for specification of orbit - Keplerian Elements

        // Shape and Size
        pub e: f32, // Eccentricity
        pub a: f32, // Semimajor axis
        // Orientation of orbital plane
        pub i: f32, // Inclination
        pub o: f32, // Longitude of the ascending node
        // Other
        pub w: f32, // Argument of periapsis
        pub v: f32, // True anomaly
    }
}

pub mod calculations {
    use datatypes::Coordinates;

    pub fn get_squared_delta(a: i32, b: i32) -> i32 {
        // Math helper
        return (b - a) * (b - a);
    }

    pub fn get_distance(point1: Coordinates, point2: Coordinates) -> f32 {
        // Calculates distance between 2 points in the universe based on coordinates
        let squared_x: i32 = get_squared_delta(point1.x, point2.x);
        let squared_y: i32 = get_squared_delta(point1.y, point2.y);
        let squared_z: i32 = get_squared_delta(point1.z, point2.z);
        let sum_int: i32 = squared_x + squared_y + squared_z;
        let sum_float: f32 = sum_int as f32;
        return  sum_float.sqrt();
    }
}
