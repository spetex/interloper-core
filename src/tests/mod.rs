#[cfg(test)]

use calculations;
use datatypes;

#[test]
fn it_works() {
}

#[test]
fn it_adds() {
    assert!(calculations::add(5,5) == 10);
}

#[test]
fn it_measures_distance() {
    let point1 = datatypes::Coordinates{x: 32, y: 11, z: 78};
    let point2 = datatypes::Coordinates{x: -78, y: 45, z: 12};

    assert!(calculations::get_distance(point1,point2) == 132.7102);
}
