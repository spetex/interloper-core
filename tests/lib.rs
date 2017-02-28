#[cfg(test)]

extern crate spacesim;

#[test]
fn it_measures_distance() {
    let point1 = spacesim::datatypes::Coordinates{x: 32, y: 11, z: 78};
    let point2 = spacesim::datatypes::Coordinates{x: -78, y: 45, z: 12};

    assert!(spacesim::calculations::get_distance(point1,point2) == 132.7102);
}
