pub struct System {
    mass: f32,
    subsystems: Vec<System>,
    description: String,
}

impl System {
    pub fn total_mass(&self) -> f32 {
        unimplemented!();
        //self.subsystems.iter().fold(self.mass, |total, s| total + s.total_mass())
    }
}
