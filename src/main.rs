extern crate spacesim;
use spacesim::datatypes;
use spacesim::calculations;


fn main() {
    let here = datatypes::Coordinates{ x:44, y:32, z:678 };

    println!("Starting SpaceSim...");
    println!("Add {}", calculations::add(5,5));
    println!("Spot: {}", here);
}
