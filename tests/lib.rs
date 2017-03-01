#[cfg(test)]

extern crate spacecalcs;

#[test]
fn it_measures_distance() {
    let point1 = spacecalcs::datatypes::Coordinates{x: 32, y: 11, z: 78};
    let point2 = spacecalcs::datatypes::Coordinates{x: -78, y: 45, z: 12};

    assert!(spacecalcs::calculations::get_distance(point1,point2) == 132.7102);
}
