extern crate nalgebra as na;


pub struct System {
    mass: f32,
    position: [f32; 3],
    moment_of_inertia: [[f32; 3];3],
    subsystems: Vec<System>,
    description: String,
}

impl System {
    pub fn total_mass(&self) -> f32 {
        self.subsystems.iter().fold(self.mass, |total, s| total + s.total_mass())
    }

    pub fn total_inertia(&self) -> [[f32; 3]; 3] {
        unimplemented!()
    }

    pub fn new(mass: f32, position: [f32; 3], moment_of_inertia: [[f32; 3];3], subsystems: Vec<System>, description: String) -> System {
        System{mass: mass,
               position: position,
               moment_of_inertia: moment_of_inertia,
               subsystems: subsystems,
               description: description}
    }
}
