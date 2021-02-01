use universe::Universe;
use datatypes::Vector;
use utils;

/// Celestial is a generic object for everything that can reside in the space.
pub struct Celestial {
    pub id: String,
    pub mass: f64,
    pub coordinates: Vector,
    pub speed: Vector,
}

impl Celestial {
    /// Celestial constructor - creates new celestial object
    pub fn new(id: String, mass: f64) -> Celestial {
        Celestial {
            id: id,
            mass: mass,
            coordinates: vec![0.0, 0.0, 0.0],
            speed: vec![0.0, 0.0, 0.0],
        }
    }
    pub fn new_full(id: String, mass: f64, coordinates: Vector, speed: Vector) -> Celestial {
        Celestial {
            id: id,
            mass: mass,
            coordinates: coordinates,
            speed: speed,
        }
    }
    pub fn copy(self) -> Celestial {
        Celestial::new_full(self.id, self.mass, self.coordinates, self.speed)
    }
    pub fn move_by(self, vector: Vector) -> Celestial {
        Celestial::new_full(
            self.id,
            self.mass,
            vec![
                self.coordinates[0] + vector[0],
                self.coordinates[1] + vector[1],
                self.coordinates[2] + vector[2],
            ],
            self.speed,
        )
    }

    pub fn accelerate(&self, vector: Vector) -> Celestial {
        Celestial::new_full(
            self.id.clone(),
            self.mass,
            self.coordinates.clone(),
            vec![
                self.speed[0] + vector[0],
                self.speed[1] + vector[1],
                self.speed[2] + vector[2],
            ],
        )
    }

    pub fn move_to(self, position: Vector) -> Celestial {
        Celestial::new_full(self.id, self.mass, position, self.speed)
    }

    pub fn move_self(self) -> Celestial {
        Celestial::new_full(
            self.id,
            self.mass,
            vec![
                self.coordinates[0] + self.speed[0],
                self.coordinates[1] + self.speed[1],
                self.coordinates[2] + self.speed[2],
            ],
            self.speed,
        )
    }

    pub fn next_state(&self, universe: &Universe) -> Celestial {
        let acceleration = self.get_current_acceleration(universe);
        self.accelerate(acceleration).move_self()
    }

    fn get_current_acceleration(&self, universe: &Universe) -> Vector {
        self.get_force_vector(universe).iter().map(|it| { it / self.mass }).collect()
    }

    fn get_force_vector(&self, universe: &Universe) -> Vector {
        return universe
            .get_influential_celestials(&self)
            .iter()
            .map(|val| {
                self.get_pull_force_of(val)
             })
            .fold(vec![], |acc, list| {
                   vec![acc[0] + list[0], acc[1] + list[1], acc[2] + list[2]]
             });
    }

    pub fn get_distance_from(&self, target: &Celestial) -> f64 {
        utils::get_distance(self, target)
    }

    pub fn get_pull_force_of(&self, celestial: &Celestial) -> Vector {
        return vec![
            utils::get_pull_force(
                self.mass,
                celestial.mass,
                self.coordinates[0],
                celestial.coordinates[0],
                self.get_distance_from(&celestial),
            ),
            utils::get_pull_force(
                self.mass,
                celestial.mass,
                self.coordinates[1],
                celestial.coordinates[1],
                self.get_distance_from(&celestial),
            ),
            utils::get_pull_force(
                self.mass,
                celestial.mass,
                self.coordinates[2],
                celestial.coordinates[2],
                self.get_distance_from(&celestial),
            ),
        ];
    }
}
