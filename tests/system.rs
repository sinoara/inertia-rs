use inertia_rs::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mass_simple_system() {
        let system = System::new(1.5, [0.0;3], [[0.0;3];3], vec![], String::from("sys"));

        assert_eq!(system.total_mass(), 1.5);
    }

    #[test]
    fn mass_zero_mass_system() {
        let system = System::new(0.0, [0.0;3], [[0.0;3];3], vec![], String::from("sys"));

        assert_eq!(system.total_mass(), 0.0);
    }

    #[test]
    fn mass_composed_system() {
        let sub_system = System::new(2.4, [0.0;3], [[0.0;3];3], vec![], String::from("sys"));
        let system = System::new(1.7, [0.0;3], [[0.0;3];3], vec![sub_system], String::from("sys"));

        assert_eq!(system.total_mass(), 2.4+1.7);
    }

    #[test]
    fn inertia_simple_system() {
        let system = System::new(
            1.5, [0.0;3],
            [[1.0, 0.0, 0.0],[0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            vec![], String::from("sys"));

        assert_eq!(system.total_inertia(), [[1.0, 0.0, 0.0],[0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    }

    #[test]
    fn inertia_same_center_system() {
        let sub_system = System::new(
            1.5, [0.0;3],
            [[1.0, 0.0, 0.0],[0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            vec![], String::from("sys"));
        let system = System::new(
            1.5, [0.0;3],
            [[1.5, 0.0, 0.0],[0.0, 0.9, 0.0], [0.0, 0.0, 1.1]],
            vec![sub_system], String::from("sys"));

        assert_eq!(system.total_inertia(), [[1.0+1.5, 0.0, 0.0],[0.0, 1.0+0.9, 0.0], [0.0, 0.0, 1.0+1.1]]);
    }

    /*
    #[test]
    fn inertia_same_center_system() {
        let sub_system = System::new(
            1.2, [0.0;3],
            [[1.0, 0.0, 0.0],[0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            vec![], String::from("sys"));
        let system = System::new(
            1.5, [],
            [[1.5, 0.0, 0.0],[0.0, 0.9, 0.0], [0.0, 0.0, 1.1]],
            vec![sub_system], String::from("sys"));

        assert_eq!(system.total_inertia(), [[1.0+1.5, 0.0, 0.0],[0.0, 1.0+0.9, 0.0], [0.0, 0.0, 1.0+1.1]]);
    }
    */
}
