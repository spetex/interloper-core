extern crate spacecalcs;
use spacecalcs::datatypes;
use spacecalcs::calculations;


fn main() {
    let here = datatypes::Coordinates{ x:32, y:11, z:78 };
    let there = datatypes::Coordinates{ x:-78, y:45, z:12 };

    println!("Starting SpaceSim...");
    println!("Spot 1: {}", here);
    println!("Spot 2: {}", there);
    println!("Distance: {}", calculations::get_distance(here, there));
}
