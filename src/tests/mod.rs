#[cfg(test)]
#[test]
fn it_measures_distance() {
    use datatypes;
    use calculations;
    let point1 = datatypes::Coordinates {
        x: 32,
        y: 11,
        z: 78,
    };
    let point2 = datatypes::Coordinates {
        x: -78,
        y: 45,
        z: 12,
    };

    assert!(calculations::get_distance(point1, point2) == 132.7102);
}

#[test]
fn coordinates_creates_itself_with_default() {
    use datatypes;
    let point_in_space = datatypes::Coordinates::new_default();
    assert!(point_in_space.x == 0);
    assert!(point_in_space.y == 0);
    assert!(point_in_space.z == 0);
}

#[test]
fn coordinates_creates_itself_on_specific_place() {
    use datatypes;
    let point_in_space = datatypes::Coordinates::new(5, 12, -65);
    assert!(point_in_space.x == 5);
    assert!(point_in_space.y == 12);
    assert!(point_in_space.z == -65);
}

#[test]
fn universe_exists() {
    use objects;
    let first_universe = objects::Universe::new(0);
    assert!(first_universe.id == 0);
}

#[test]
fn universe_spawns_celestial() {
    use objects;
    let mut first_universe = objects::Universe::new(0);
    let celestial_one = first_universe.spawn();
    assert!(celestial_one.id == 1);
    assert!(celestial_one.location.x == 0);
    assert!(celestial_one.location.y == 0);
    assert!(celestial_one.location.z == 0);
}
