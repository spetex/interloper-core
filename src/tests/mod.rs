#[cfg(test)]
#[test]
fn it_measures_distance() {
    use celestial::Celestial;

    let earth = Celestial::new_full(
        String::from("Earth"),
        100.0,
        vec![100.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0],
    );
    let moon = Celestial::new_full(
        String::from("Moon"),
        100.0,
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0],
    );

    assert!(earth.get_distance_from(&moon) == 100.0);
}

#[test]
fn it_moves_given_speed() {
    use celestial::Celestial;
    assert!(
        Celestial::new_full(
            String::from("Earth"),
            100.0,
            vec![0.0, 0.0, 0.0],
            vec![10.0, 0.0, 0.0]
        )
        .move_self()
        .coordinates
            == vec![10.0, 0.0, 0.0]
    );
}

#[test]
fn it_moves_given_acceleration() {
    use celestial::Celestial;
    assert!(
        Celestial::new_full(
            String::from("Earth"),
            100.0,
            vec![0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0]
        )
        .accelerate(vec![20.0, 0.0, 0.0])
        .move_self()
        .move_self()
        .move_self()
        .coordinates
            == vec![60.0, 0.0, 0.0]
    );
}
