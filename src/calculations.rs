use datatypes::Coordinates;

/// Math helper
pub fn get_squared_delta(a: i32, b: i32) -> i32 {
    return (b - a) * (b - a);
}

/// Calculates distance between 2 points in the universe based on coordinates
pub fn get_distance(point1: Coordinates, point2: Coordinates) -> f32 {
    let squared_x: i32 = get_squared_delta(point1.x, point2.x);
    let squared_y: i32 = get_squared_delta(point1.y, point2.y);
    let squared_z: i32 = get_squared_delta(point1.z, point2.z);
    let sum_int: i32 = squared_x + squared_y + squared_z;
    let sum_float: f32 = sum_int as f32;
    sum_float.sqrt()
}
