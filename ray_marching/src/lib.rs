pub mod vectors;

#[cfg(test)]
mod tests {
    
}

#[cfg(test)]
mod vector_tests {
    use super::vectors::Vector;

    
    #[test]
    fn test_commutativity() {
        let a: Vector = Vector::new(
            3, 
            4, 
            -2
        );

        assert_eq!(3*a, a*3);
    }

    #[test]
    fn division_inverse_of_multiplicaiton() {
        let a: Vector = Vector::new(
            3, 
            4, 
            -2
        );

        assert_eq!((3*a)/3, a);
        assert_eq!(a + a, 2*a);
    }

    #[test]
    fn repeated_addition_is_multiplication() {
        let a: Vector = Vector::new(
            3, 
            4, 
            -2
        );

        assert_eq!(a + a, 2*a);
    }

    #[test]
    fn test_subtraction() {
        let a: Vector = Vector::new(
            1,
            0,
            0
        );

        let b: Vector = Vector::new(
            0,
            0,
            0
        );

        assert_eq!(a-a, b);
        assert_eq!(b-a, -a);
    }

    #[test]
    fn equality_test() {
        let a: Vector = Vector::new(
            3, 
            4, 
            -2
        );

        assert_ne!(a, Vector::new(
            3, 
            4, 
            -1
        ));

        assert_eq!(a, a);
    }

    #[test]
    fn test_norm() {
        let a: Vector = Vector::new(
            1,
            0,
            0
        );

        assert_eq!(a.norm(), 1.);
    }

    #[test]
    fn test_dist() {
        let a: Vector = Vector::new(
            1,
            0,
            0
        );

        let b: Vector = Vector::new(
            0,
            0,
            0
        );

        assert_eq!(a.dist(b), 1.);
    }

    #[test]
    fn to_normed_test() {
        let a: Vector = Vector::new(
            1,
            0,
            0
        );

        assert_eq!(a, a.to_normed());
    }

    #[test]
    fn test_string() {
        let a: Vector = Vector::new(
            1,
            0,
            0
        );

        let b: Vector = Vector::new(
            0,
            0,
            0
        );

        assert_eq!(a.to_string(), "(1, 0, 0)");
        assert_eq!((a-a).to_string(), b.to_string());
    }
}
