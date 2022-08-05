use std::ops;
use approx::ulps_eq;

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

/*impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {} // TODO
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {} // TODO
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {} // TODO
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {} // TODO
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {} // TODO
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> Self::Output {} // TODO
}*/

#[cfg(test)]
mod tests {
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
}
