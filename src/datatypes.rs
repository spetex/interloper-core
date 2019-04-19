use std::fmt;

/// Standard datatype for saving position of object in space
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{},{}]", self.x, self.y, self.z)
    }
}

impl Coordinates {
    /// Creates new set of coordinates in the origin
    pub fn new_default() -> Coordinates {
        Coordinates { x: 0, y: 0, z: 0 }
    }
    /// Creates new set of coordinates on specific place in universe
    pub fn new(x: i32, y: i32, z: i32) -> Coordinates {
        Coordinates { x: x, y: y, z: z }
    }
}
