use nalgebra::{Matrix3, Vector3};

pub struct System {
    mass: f32,
    position: Vector3<f32>,
    moment_of_inertia: Matrix3<f32>,
    subsystems: Vec<System>,
    description: String,
}

impl System {
    pub fn total_mass(&self) -> f32 {
        self.subsystems
            .iter()
            .fold(self.mass, |total, s| total + s.total_mass())
    }

    pub fn total_inertia(&self) -> Matrix3<f32> {
        unimplemented!()
    }

    pub fn new(
        mass: f32,
        position: Vector3<f32>,
        moment_of_inertia: Matrix3<f32>,
        subsystems: Vec<System>,
        description: String,
    ) -> System {
        System {
            mass: mass,
            position: position,
            moment_of_inertia: moment_of_inertia,
            subsystems: subsystems,
            description: description,
        }
    }
}
