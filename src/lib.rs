use nalgebra::{Matrix3, Vector3};

pub struct System {
    mass: f32,
    position: Vector3<f32>,
    moment_of_inertia: Matrix3<f32>,
    subsystems: Vec<Box<dyn Inertia>>,
    description: String,
}

pub trait Inertia {
    fn total_mass(&self) -> f32;
    fn total_inertia(&self) -> Matrix3<f32>;
    fn center_of_mass(&self) -> Vector3<f32>;

    fn get_mass(&self) -> f32;
    fn get_position(&self) -> Vector3<f32>;
}

impl Inertia for System {
    fn total_mass(&self) -> f32 {
        self.subsystems
            .iter()
            .fold(self.mass, |total, s| total + s.total_mass())
    }

    fn total_inertia(&self) -> Matrix3<f32> {
        self.subsystems
            .iter()
            .fold(self.moment_of_inertia, |total, s| {
                total
                    + s.total_inertia()
                    + s.get_mass()
                        * (s.get_position().dot(&s.get_position()) * Matrix3::identity()
                            - s.get_position() * s.get_position().transpose())
            })
    }

    fn center_of_mass(&self) -> Vector3<f32> {
        self.subsystems
            .iter()
            .fold(Vector3::zeros(), |total, s| total + s.total_mass()*s.get_position())
            /self.total_mass()
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }

    fn get_position(&self) -> Vector3<f32> {
        self.position
    }
}

impl System {
    pub fn new(
        mass: f32,
        position: Vector3<f32>,
        moment_of_inertia: Matrix3<f32>,
        subsystems: Vec<Box<dyn Inertia>>,
        description: String,
    ) -> System {
        System {
            mass,
            position,
            moment_of_inertia,
            subsystems,
            description,
        }
    }
}
