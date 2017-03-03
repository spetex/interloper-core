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

}
