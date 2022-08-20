use inertia_rs::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_system() {
        let system = System::new(1.5, vec![], String::from("sys"));

        assert_eq!(system.total_mass(), 1.5);
    }

    #[test]
    fn zero_mass_system() {
        let system = System::new(0.0, vec![], String::from("sys"));

        assert_eq!(system.total_mass(), 0.0);
    }

    #[test]
    fn composed_system() {
        let sub_system = System::new(2.4, vec![], String::from("sys"));
        let system = System::new(1.7, vec![sub_system], String::from("sys"));

        assert_eq!(system.total_mass(), 4.1);
    }
}
