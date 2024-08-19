mod shapes {
    pub struct Circle {
        radius: f64,
    }

    impl Circle {
        pub fn new(radius: f64) -> Circle {
            Circle { radius }
        }

        pub fn new_1(radius: f64) -> Result<Circle, String> {
            if radius >= 0.0 {
                Ok(Circle { radius })
            } else {
                Err(String::from("Radius should be positive"))
            }
        }

        pub fn new_2(radius: f64) -> Circle {
            match radius {
                ..=0.0 => panic!("radius should be positive"),
                _ => Circle { radius },
            }
        }

        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}

#[cfg(test)]
mod tests {
    use shapes::Circle;

    use super::*;

    #[test]
    fn larger_circle_contains_smaller() {
        let larger_circle = Circle::new(7.0);
        let smaller_circle = Circle::new(5.0);
        assert_eq!(larger_circle.contains(&smaller_circle), true);
        assert_ne!(larger_circle.contains(&smaller_circle), false);
        assert!(larger_circle.contains(&smaller_circle));
    }

    #[test]
    fn smaller_circle_not_contain_larger() {
        let larger_circle = Circle::new(10.0);
        let smaller_circle = Circle::new(5.0);
        assert_eq!(smaller_circle.contains(&larger_circle), false);
    }

    #[test]
    fn should_not_create_circle() -> Result<(), String> {
        let _circle = Circle::new_1(-23.0)?;
        Ok(())
    }

    #[test]
    #[should_panic(expected = "radius should be positive")]
    fn should_not_create_and_panic() {
        let _circle = Circle::new_2(-5.0);
    }

    #[test]
    #[ignore]
    fn huge_test() {}
}
