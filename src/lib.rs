pub struct System {
    mass: f32,
    moment_of_inertia: [[f32; 3];3],
    subsystems: Vec<System>,
    description: String,
}

impl System {
    pub fn total_mass(&self) -> f32 {
        self.subsystems.iter().fold(self.mass, |total, s| total + s.total_mass())
    }

    pub fn new(mass: f32, moment_of_inertia: [[f32; 3];3], subsystems: Vec<System>, description: String) -> System {
        System{mass: mass,
               moment_of_inertia: moment_of_inertia,
               subsystems: subsystems,
               description: description}
    }
}
