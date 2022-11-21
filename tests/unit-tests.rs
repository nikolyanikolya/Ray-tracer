#[cfg(test)]
mod tests {
    use raytracer::util::vec3::{PixelOperations, Vec3};

    #[test]
    fn test_vec3_add() {
        let vec1 = Vec3 {
            x: 1.0,
            y: 5.0,
            z: 7.0,
        };
        let vec2 = Vec3 {
            x: -5.0,
            y: 10.0,
            z: 0.0,
        };
        assert_eq!(
            Vec3 {
                x: -4.0,
                y: 15.0,
                z: 7.0,
            },
            vec1 + vec2
        );
    }

    #[test]
    fn test_vec3_sub() {
        let vec1 = Vec3 {
            x: 1.0,
            y: 5.0,
            z: 7.0,
        };
        let vec2 = Vec3 {
            x: -5.0,
            y: 10.0,
            z: 0.0,
        };
        assert_eq!(
            Vec3 {
                x: 6.0,
                y: -5.0,
                z: 7.0,
            },
            vec1 - vec2
        );
    }

    #[test]
    fn test_vec3_mul_vec3() {
        let vec1 = Vec3 {
            x: 1.0,
            y: 5.0,
            z: 7.0,
        };
        let vec2 = Vec3 {
            x: -5.0,
            y: 10.0,
            z: 0.0,
        };
        assert_eq!(45.0, vec1 * vec2);
    }

    #[test]
    fn test_vec3_mul_float() {
        let vec = Vec3 {
            x: 1.0,
            y: 5.0,
            z: 7.0,
        };
        let f = 2.5;
        assert_eq!(
            Vec3 {
                x: 2.5,
                y: 12.5,
                z: 17.5,
            },
            vec * f
        );
    }

    #[test]
    fn test_vec3_neg() {
        let vec = Vec3 {
            x: 1.0,
            y: 5.0,
            z: 7.0,
        };
        assert_eq!(
            Vec3 {
                x: -1.0,
                y: -5.0,
                z: -7.0,
            },
            -vec
        );
    }

    #[test]
    fn test_vec3_norm() {
        let vec = Vec3 {
            x: 4.0,
            y: 4.0,
            z: 7.0,
        };
        assert_eq!(9.0, vec.norm());
    }

    #[test]
    fn test_vec3_normalized() {
        let vec = Vec3 {
            x: 4.0,
            y: 4.0,
            z: 7.0,
        };
        assert_eq!(
            Vec3 {
                x: 4.0 / 9.0,
                y: 4.0 / 9.0,
                z: 7.0 / 9.0,
            },
            vec.normalized()
        );
    }

    #[test]
    fn test_vec3_square() {
        let vec = Vec3 {
            x: 1.0,
            y: 5.0,
            z: 7.0,
        };
        assert_eq!(75.0, vec * vec);
    }
}
