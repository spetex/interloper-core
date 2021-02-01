use celestial::Celestial;

/// The Universe - Every interaction takes place in the relation to the universe.
pub struct Universe {
    id: String,
    celestials: Vec<Celestial>,
}

impl Universe {
    /// Universe constructor - creates single universe containing 1 celestial.
    pub fn new(id: String, celestials: Vec<Celestial>) -> Universe {
        Universe {
            id: id,
            celestials: celestials,
        }
    }

    pub fn spawn(self, celestial: Celestial) -> Universe {
        let celestials = vec![celestial];
        let celestials = celestials.into_iter().chain(self.celestials).collect();
        return Universe::new(self.id, celestials);
    }

    pub fn celestial_count(self) -> usize {
        self.celestials.len()
    }

    pub fn get_celestial(self, id: String) -> Option<Celestial> {
        return self.celestials.into_iter().find(|item| item.id == id);
    }

    pub fn get_influential_celestials(&self, celestial: &Celestial) -> Vec<Celestial> {
        self.celestials
            .into_iter()
            .filter_map(|it| {
                if it.id != celestial.id {
                    Some(it)
                } else {
                    None
                }
            })
            .map(|it| it)
            .collect()
    }

    pub fn next_state(&self) -> Universe {
        return Universe::new(
            self.id.clone(),
            self.celestials
                .iter()
                .map(|it| {
                    it.next_state(&self)
                })
                .collect()
            )
    }
}
