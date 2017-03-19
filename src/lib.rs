#![doc(html_root_url = "https://spetex.github.io/spacesim/")]
//! # Spacesim Library
//! Hobby library for simulating celestial mechanics

pub mod datatypes;
pub mod calculations;

pub mod objects {
    use datatypes::Coordinates;
    /// Celestial is a generic object for everything that can reside in the space.
    pub struct Celestial {
        pub id: i32,
        pub location: Coordinates,
    }

    impl Celestial {
        /// Celestial constructor - creates new celestial object
        pub fn new(id: i32) -> Celestial {
            Celestial {
                id: id,
                location: Coordinates::new_default(),
            }
        }
    }

    /// The Universe - Every interaction takes place in the relation to the universe.
    pub struct Universe {
        id: i8,
        num_objects: i32,
        epoch: i32,
        celestials: Vec<Celestial>,
    }

    impl Universe {
        /// Universe constructor - creates single universe containing 1 celestial.
        pub fn new(id: i8) -> Universe {
            Universe {
                id: id,
                num_objects: 0,
                epoch: 0,
                celestials: Vec::new(),
            }
        }
        /// Returns id of the universe.
        pub fn get_id(&self) -> i8 {
            self.id
        }
        /// Spawns a celestial in the origin.
        pub fn spawn(&mut self) {
            self.celestials.push(Celestial::new(self.num_objects));
            self.num_objects = self.num_objects + 1;
        }
        /// Returns a count of all celestials in the given universe.
        pub fn get_celestial_count(&self) -> i32 {
            self.celestials.len() as i32
        }
        /// Returns reference to the celestial with given id.
        pub fn get_celestial(&self, id: i32) -> &Celestial {
            &self.celestials[id as usize]
        }
        /// Moves celestial by id to a given position.
        pub fn set_position(&mut self, id: i32, point: Coordinates) {
            self.celestials[id as usize].location = point;
        }
    }
}

mod tests;
