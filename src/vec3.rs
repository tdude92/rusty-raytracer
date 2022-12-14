use std::ops;

use crate::{random_f64, random_f64_in};

pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {
            e: [x, y, z]
        }
    }

    pub fn random() -> Self {
        Self::new(random_f64(), random_f64(), random_f64())
    }

    /// Generates random vector with component values in [min, max)
    pub fn random_in(min: f64, max: f64) -> Self {
        Self::new(random_f64_in(min, max), random_f64_in(min, max), random_f64_in(min, max))
    }

    /// Returns random vector within the unit sphere
    /// ie. length of the vector is less than 1.0
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_in(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// Returns a random vector within the unit disc
    /// lying on the xy-plane
    pub fn random_in_unit_disc() -> Self {
        loop {
            let p = Self::new(random_f64_in(-1.0, 1.0), random_f64_in(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    /// Reflects the current vector across a normal unit vector n
    pub fn reflect(&self, n: &Self) -> Self {
        self - 2.0*Self::dot(self, n)*n
    }

    /// Refracts vector across a normal unit vector n
    /// with a ratio of refractive index ratio eta_i / eta_t
    pub fn refract(&self, n: &Vec3, eta_ratio: f64) -> Self {
        let cos_theta = Self::dot(&(-self), n).min(1.0);
        let r_out_perpendicular = eta_ratio * (self + cos_theta*n);
        let r_out_parallel = -(1.0 - r_out_perpendicular.length_squared()).abs().sqrt()*n;
        r_out_perpendicular + r_out_parallel
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn dot(v1: &Self, v2: &Self) -> f64 {
        v1.e[0]*v2.e[0] + v1.e[1]*v2.e[1] + v1.e[2]*v2.e[2]
    }

    pub fn cross(v1: &Self, v2: &Self) -> Vec3 {
        Vec3 {
            e: [
                v1.e[1]*v2.e[2] - v1.e[2]*v2.e[1],
                v1.e[2]*v2.e[0] - v1.e[0]*v2.e[2],
                v1.e[0]*v2.e[1] - v1.e[1]*v2.e[0],
            ]
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

    /// Returns true if this vector is approximately a zero vector
    pub fn is_near_zero(&self) -> bool {
        let epsilon: f64 = 1e-8;

        (self.e[0].abs() < epsilon) &&
        (self.e[1].abs() < epsilon) &&
        (self.e[2].abs() < epsilon)
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

// Implements a binary operation on any combination of
// operand types Vec3, &Vec3, and f64
macro_rules! binary_op {
    ($Operation:ident $op_fn:ident $op_sym:tt) => {
        // Vec3, Vec3
        impl ops::$Operation<Vec3> for Vec3 {
            type Output = Vec3;

            fn $op_fn(self, other: Vec3) -> Vec3 {
                Vec3 {
                    e: [
                        self.e[0] $op_sym other.e[0],
                        self.e[1] $op_sym other.e[1],
                        self.e[2] $op_sym other.e[2],
                    ]
                }
            }
        }

        // &Vec3, &Vec3
        impl<'a, 'b> ops::$Operation<&'a Vec3> for &'b Vec3 {
            type Output = Vec3;

            fn $op_fn(self, other: &'a Vec3) -> Vec3 {
                Vec3 {
                    e: [
                        self.e[0] $op_sym other.e[0],
                        self.e[1] $op_sym other.e[1],
                        self.e[2] $op_sym other.e[2],
                    ]
                }
            }
        }

        // &Vec3, Vec3
        impl<'a> ops::$Operation<Vec3> for &'a Vec3 {
            type Output = Vec3;

            fn $op_fn(self, other: Vec3) -> Vec3 {
                Vec3 {
                    e: [
                        self.e[0] $op_sym other.e[0],
                        self.e[1] $op_sym other.e[1],
                        self.e[2] $op_sym other.e[2],
                    ]
                }
            }
        }

        // Vec3, &Vec3
        impl<'a> ops::$Operation<&'a Vec3> for Vec3 {
            type Output = Vec3;

            fn $op_fn(self, other: &'a Vec3) -> Vec3 {
                Vec3 {
                    e: [
                        self.e[0] $op_sym other.e[0],
                        self.e[1] $op_sym other.e[1],
                        self.e[2] $op_sym other.e[2],
                    ]
                }
            }
        }

        // Vec3, f64
        impl ops::$Operation<f64> for Vec3 {
            type Output = Vec3;

            fn $op_fn(self, other: f64) -> Vec3 {
                Vec3 {
                    e: [
                        self.e[0] $op_sym other,
                        self.e[1] $op_sym other,
                        self.e[2] $op_sym other,
                    ]
                }
            }
        }

        // &Vec3, f64
        impl<'a> ops::$Operation<f64> for &'a Vec3 {
            type Output = Vec3;

            fn $op_fn(self, other: f64) -> Vec3 {
                Vec3 {
                    e: [
                        self.e[0] $op_sym other,
                        self.e[1] $op_sym other,
                        self.e[2] $op_sym other,
                    ]
                }
            }
        }

        // f64, Vec3
        impl ops::$Operation<Vec3> for f64 {
            type Output = Vec3;

            fn $op_fn(self, other: Vec3) -> Vec3 {
                Vec3 {
                    e: [
                        self $op_sym other.e[0],
                        self $op_sym other.e[1],
                        self $op_sym other.e[2],
                    ]
                }
            }
        }

        // f64, &Vec3
        impl<'a> ops::$Operation<&'a Vec3> for f64 {
            type Output = Vec3;

            fn $op_fn(self, other: &'a Vec3) -> Vec3 {
                Vec3 {
                    e: [
                        self $op_sym other.e[0],
                        self $op_sym other.e[1],
                        self $op_sym other.e[2],
                    ]
                }
            }
        }
    }
}

// Implements a unary operation for both Vec3 and &Vec3
macro_rules! unary_op {
    ($Operation:ident $op_fn:ident $op_sym:tt) => {
        // Vec3
        impl ops::$Operation for Vec3 {
            type Output = Vec3;

            fn $op_fn(self) -> Vec3 {
                Vec3 {
                    e: [
                        $op_sym self.e[0],
                        $op_sym self.e[1],
                        $op_sym self.e[2],
                    ]
                }
            }
        }

        // &Vec3
        impl<'a> ops::$Operation for &'a Vec3 {
            type Output = Vec3;

            fn $op_fn(self) -> Vec3 {
                Vec3 {
                    e: [
                        $op_sym self.e[0],
                        $op_sym self.e[1],
                        $op_sym self.e[2],
                    ]
                }
            }
        }
    }
}

// Implement add assign operators like +=
// The first operand is always &mut Vec3 while the second can be Vec3, &Vec3, or f64
macro_rules! assignment_op {
    ($Operation:ident $op_fn:ident $op_sym:tt) => {
        // Vec3
        impl ops::$Operation<Vec3> for Vec3 {
            fn $op_fn(&mut self, other: Vec3) {
                self.e[0] $op_sym other.e[0];
                self.e[1] $op_sym other.e[1];
                self.e[2] $op_sym other.e[2];
            }
        }

        // &Vec3
        impl<'a> ops::$Operation<&'a Vec3> for Vec3 {
            fn $op_fn(&mut self, other: &'a Vec3) {
                self.e[0] $op_sym other.e[0];
                self.e[1] $op_sym other.e[1];
                self.e[2] $op_sym other.e[2];
            }
        }

        // f64
        impl ops::$Operation<f64> for Vec3 {
            fn $op_fn(&mut self, other: f64) {
                self.e[0] $op_sym other;
                self.e[1] $op_sym other;
                self.e[2] $op_sym other;
            }
        }
    }
}

binary_op!(Add add +);
assignment_op!(AddAssign add_assign +=);

binary_op!(Sub sub -);
assignment_op!(SubAssign sub_assign -=);

binary_op!(Mul mul *);
assignment_op!(MulAssign mul_assign *=);

binary_op!(Div div /);
assignment_op!(DivAssign div_assign /=);

unary_op!(Neg neg -);

#[cfg(test)]
mod tests {
    use approx::assert_ulps_eq;
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
    fn test_length_methods() {
        let v = Vec3 {e: [3.0, 4.0, 12.0]};

        assert_ulps_eq!(v.length_squared(), 169.0);
        assert_ulps_eq!(v.length(), 13.0);
    }

    #[test]
    fn test_unit_vector_method() {
        let v = Vec3 {e: [1.0, 1.0, 1.0]};
        let u = v.unit_vector();

        assert_ulps_eq!(u.x(), 0.5773502691896258);
        assert_ulps_eq!(u.y(), 0.5773502691896258);
        assert_ulps_eq!(u.z(), 0.5773502691896258);
    }
    #[test]
    fn test_dot() {
        let v1 = Vec3 {e: [ 2.0, 0.0, 0.0]};
        let v2 = Vec3 {e: [-3.0, 0.0, 0.0]};

        assert_ulps_eq!(Vec3::dot(&v1, &v2), -6.0)
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3 {e: [3.0, 0.0, 0.0]};
        let v2 = Vec3 {e: [0.0, 2.0, 0.0]};
        let v_out = Vec3::cross(&v1, &v2);

        assert_ulps_eq!(v_out.x(), 0.0);
        assert_ulps_eq!(v_out.y(), 0.0);
        assert_ulps_eq!(v_out.z(), 6.0);
    }

    mod ops {
        use super::*;

        #[test]
        fn test_neg() {
            // Ref
            let v = Vec3 {e: [1.0, 2.0, 3.0]};
            assert_ulps_eq!((-&v).x(), -1.0);
            assert_ulps_eq!((-&v).y(), -2.0);
            assert_ulps_eq!((-&v).z(), -3.0);

            // Value
            let v = Vec3 {e: [1.0, 2.0, 3.0]};
            assert_ulps_eq!((-v).x(), -1.0);

            let v = Vec3 {e: [1.0, 2.0, 3.0]};
            assert_ulps_eq!((-v).y(), -2.0);

            let v = Vec3 {e: [1.0, 2.0, 3.0]};
            assert_ulps_eq!((-v).z(), -3.0);
        }

        #[test]
        fn test_op_macro_combinations() {
            // We assume that if the macros correctly generate
            // impls for all combinations of Vec3, &Vec, and f64, for
            // one operation then it will do so for all operations.

            /* BINARY */
            // Ref, Ref
            let v1 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v2 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v_out = &v1 + &v2;
            assert_eq!(v_out.x(), 2.0);
            assert_eq!(v_out.y(), 4.0);
            assert_eq!(v_out.z(), 6.0);

            // Ref, Value
            let v1 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v2 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v_out = &v1 + v2;
            assert_eq!(v_out.x(), 2.0);
            assert_eq!(v_out.y(), 4.0);
            assert_eq!(v_out.z(), 6.0);

            // Value, Ref
            let v1 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v2 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v_out = v1 + &v2;
            assert_eq!(v_out.x(), 2.0);
            assert_eq!(v_out.y(), 4.0);
            assert_eq!(v_out.z(), 6.0);

            // Value, Value
            let v1 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v2 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v_out = v1 + v2;
            assert_eq!(v_out.x(), 2.0);
            assert_eq!(v_out.y(), 4.0);
            assert_eq!(v_out.z(), 6.0);

            // Ref, Scalar
            let v = Vec3 {e: [1.0, 2.0, 3.0]};
            let s = 2.0;
            let v_out = &v + s;
            assert_eq!(v_out.x(), 3.0);
            assert_eq!(v_out.y(), 4.0);
            assert_eq!(v_out.z(), 5.0);

            // Scalar, Ref
            let v = Vec3 {e: [1.0, 2.0, 3.0]};
            let s = 2.0;
            let v_out = s + &v;
            assert_eq!(v_out.x(), 3.0);
            assert_eq!(v_out.y(), 4.0);
            assert_eq!(v_out.z(), 5.0);

            // Value Scalar
            let v = Vec3 {e: [1.0, 2.0, 3.0]};
            let s = 2.0;
            let v_out = v + s;
            assert_eq!(v_out.x(), 3.0);
            assert_eq!(v_out.y(), 4.0);
            assert_eq!(v_out.z(), 5.0);

            // Scalar Value
            let v = Vec3 {e: [1.0, 2.0, 3.0]};
            let s = 2.0;
            let v_out = s + v;
            assert_eq!(v_out.x(), 3.0);
            assert_eq!(v_out.y(), 4.0);
            assert_eq!(v_out.z(), 5.0);

            /* UNARY */
            // Ref
            let v = Vec3 {e: [1.0, 2.0, 3.0]};
            assert_ulps_eq!((-&v).x(), -1.0);
            assert_ulps_eq!((-&v).y(), -2.0);
            assert_ulps_eq!((-&v).z(), -3.0);

            // Value
            let v = Vec3 {e: [1.0, 2.0, 3.0]};
            assert_ulps_eq!((-v).x(), -1.0);

            let v = Vec3 {e: [1.0, 2.0, 3.0]};
            assert_ulps_eq!((-v).y(), -2.0);

            let v = Vec3 {e: [1.0, 2.0, 3.0]};
            assert_ulps_eq!((-v).z(), -3.0);

            /* ASSIGNMENT */
            // Ref
            let mut v1 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v2 = Vec3 {e: [1.0, 2.0, 3.0]};
            v1 += &v2;
            assert_ulps_eq!(v1.x(), 2.0);
            assert_ulps_eq!(v1.y(), 4.0);
            assert_ulps_eq!(v1.z(), 6.0);

            // Value
            let mut v1 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v2 = Vec3 {e: [1.0, 2.0, 3.0]};
            v1 += v2;
            assert_ulps_eq!(v1.x(), 2.0);
            assert_ulps_eq!(v1.y(), 4.0);
            assert_ulps_eq!(v1.z(), 6.0);

            // Scalar
            let mut v = Vec3 {e: [1.0, 2.0, 3.0]};
            let s = 2.0;
            v += s;
            assert_ulps_eq!(v.x(), 3.0);
            assert_ulps_eq!(v.y(), 4.0);
            assert_ulps_eq!(v.z(), 5.0);
        }

        #[test]
        fn test_add() {
            // Binary op
            let v1 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v2 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v_out = v1 + v2;
            assert_eq!(v_out.x(), 2.0);
            assert_eq!(v_out.y(), 4.0);
            assert_eq!(v_out.z(), 6.0);

            // Assignment
            let mut v1 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v2 = Vec3 {e: [1.0, 2.0, 3.0]};
            v1 += v2;
            assert_ulps_eq!(v1.x(), 2.0);
            assert_ulps_eq!(v1.y(), 4.0);
            assert_ulps_eq!(v1.z(), 6.0);
        }

        #[test]
        fn test_sub() {
            // Binary op
            let v1 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v2 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v_out = v1 - v2;
            assert_eq!(v_out.x(), 0.0);
            assert_eq!(v_out.y(), 0.0);
            assert_eq!(v_out.z(), 0.0);

            // Assignment
            let mut v1 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v2 = Vec3 {e: [1.0, 2.0, 3.0]};
            v1 -= v2;
            assert_ulps_eq!(v1.x(), 0.0);
            assert_ulps_eq!(v1.y(), 0.0);
            assert_ulps_eq!(v1.z(), 0.0);
        }

        #[test]
        fn test_mul() {
            // Binary op
            let v1 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v2 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v_out = v1 * v2;
            assert_eq!(v_out.x(), 1.0);
            assert_eq!(v_out.y(), 4.0);
            assert_eq!(v_out.z(), 9.0);

            // Assignment
            let mut v1 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v2 = Vec3 {e: [1.0, 2.0, 3.0]};
            v1 *= v2;
            assert_ulps_eq!(v1.x(), 1.0);
            assert_ulps_eq!(v1.y(), 4.0);
            assert_ulps_eq!(v1.z(), 9.0);
        }

        #[test]
        fn test_div() {
            // Binary op
            let v1 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v2 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v_out = v1 / v2;
            assert_eq!(v_out.x(), 1.0);
            assert_eq!(v_out.y(), 1.0);
            assert_eq!(v_out.z(), 1.0);

            // Assignment
            let mut v1 = Vec3 {e: [1.0, 2.0, 3.0]};
            let v2 = Vec3 {e: [1.0, 2.0, 3.0]};
            v1 /= v2;
            assert_ulps_eq!(v1.x(), 1.0);
            assert_ulps_eq!(v1.y(), 1.0);
            assert_ulps_eq!(v1.z(), 1.0);
        }

        #[test]
        fn test_index() {
            let v = Vec3 {e: [1.0, 2.0, 3.0]};
            assert_ulps_eq!(v[0], 1.0);
            assert_ulps_eq!(v[1], 2.0);
            assert_ulps_eq!(v[2], 3.0);

            let v = &v;
            assert_ulps_eq!(v[0], 1.0);
            assert_ulps_eq!(v[1], 2.0);
            assert_ulps_eq!(v[2], 3.0);
        }
    }
}
