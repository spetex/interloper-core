#![doc(html_root_url = "https://spetex.github.io/spacesim/")]
//! # Spacesim Library
//! Hobby library for sumulating celestial mechanics

pub mod datatypes;
pub mod calculations;

pub mod objects {
    use datatypes;
    /// Celestial is a generic object for everything that can reside in the space.
    pub struct Celestial {
        pub id: i32,
        pub location: datatypes::Coordinates,
    }

    /// The Universe - Every interaction takes place in the relation to the universe.
    pub struct Universe {
        pub id: i8,
        pub num_objects: i32,
        pub celestials: Vec<Celestial>,
    }

    impl Universe {
        /// Universe constructor - creates single universe containing 1 celestial.
        pub fn new(id: i8) -> Universe {
            Universe {
                id: id,
                num_objects: 0,
                celestials: vec![Celestial {
                                     id: 0,
                                     location: datatypes::Coordinates::new_default(),
                                 }],
            }
        }
        /// Returns id of the universe.
        pub fn get_id(&self) -> i8 {
            self.id
        }
        /// Spawns a celestial in the origin.
        pub fn spawn(&mut self) {
            self.num_objects = self.num_objects + 1;
            self.celestials.push(Celestial {
                id: self.num_objects,
                location: datatypes::Coordinates::new_default(),
            });
        }
        /// Returns a count of all celestials in the given universe.
        pub fn get_celestial_count(&self) -> i32 {
            self.celestials.len() as i32
        }
        /// Returns reference to the celestial with given id.
        pub fn get_celestial(&self, id: i32) -> &Celestial {
            let i: usize = id as usize;
            &self.celestials[i]
        }
        /// Moves celestial by id to a given position.
        pub fn set_position(&mut self, id: i32, point: datatypes::Coordinates) {
            let i: usize = id as usize;
            self.celestials[i].location = point;
        }
    }
}

mod tests;
