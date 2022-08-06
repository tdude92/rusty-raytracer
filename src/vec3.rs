use std::ops;

pub type Point3 = Vec3;
pub type Color  = Vec3;

#[derive(Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {
            e: [x, y, z]
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.e[0] /= other;
        self.e[1] /= other;
        self.e[2] /= other;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

#[cfg(test)]
mod tests {
    use approx::ulps_eq;
    use super::*;

    #[test]
    fn test_getters() {
        let v = Vec3 {e: [1.0, 2.0, 3.0]};

        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_constructor() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_length_functions() {
        let v = Vec3 {e: [3.0, 4.0, 12.0]};

        assert!(ulps_eq!(v.length_squared(), 169.0));
        assert!(ulps_eq!(v.length(), 13.0));
    }

    mod ops {
        use super::*;

        #[test]
        fn test_neg() {
            let v = Vec3 {e: [1.0, 2.0, 3.0]};
            let v = -v;

            assert!(ulps_eq!(v.x(), -1.0));
            assert!(ulps_eq!(v.y(), -2.0));
            assert!(ulps_eq!(v.z(), -3.0));
        }

        #[test]
        fn test_add_assign() {
            let mut v1 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v2 = Vec3 {e: [1.0, 2.0, 3.0]};
            v1 += v2;

            assert!(ulps_eq!(v1.x(), 2.0));
            assert!(ulps_eq!(v1.y(), 4.0));
            assert!(ulps_eq!(v1.z(), 6.0));
        }

        #[test]
        fn test_sub_assign() {
            let mut v1 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v2 = Vec3 {e: [1.0, 2.0, 3.0]};
            v1 -= v2;

            assert!(ulps_eq!(v1.x(), 0.0));
            assert!(ulps_eq!(v1.y(), 0.0));
            assert!(ulps_eq!(v1.z(), 0.0));
        }

        #[test]
        fn test_mul_assign() {
            let mut v = Vec3 {e: [1.0, 2.0, 3.0]};
            v *= 2.0;

            assert!(ulps_eq!(v.x(), 2.0));
            assert!(ulps_eq!(v.y(), 4.0));
            assert!(ulps_eq!(v.z(), 6.0));
        }

        #[test]
        fn test_div_assign() {
            let mut v = Vec3 {e: [1.0, 2.0, 3.0]};
            v /= 2.0;

            assert!(ulps_eq!(v.x(), 0.5));
            assert!(ulps_eq!(v.y(), 1.0));
            assert!(ulps_eq!(v.z(), 1.5));
        }

        #[test]
        fn test_index() {
            let v = Vec3 {e: [1.0, 2.0, 3.0]};

            assert!(ulps_eq!(v[0], 1.0));
            assert!(ulps_eq!(v[1], 2.0));
            assert!(ulps_eq!(v[2], 3.0));
        }
    }
}
