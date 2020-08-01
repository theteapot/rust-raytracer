use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    vector: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 {
            vector: [e0, e1, e2],
        }
    }

    pub fn dot(&self, v: Vec3) -> f32 {
        dot(*self, v)
    }

    pub fn cross(&self, v: Vec3) -> Vec3 {
        cross(*self, v)
    }

    pub fn length(&self) -> f32 {
        length(*self)
    }

    pub fn squared_length(&self) -> f32 {
        squared_length(*self)
    }

    pub fn make_unit_vector(&mut self) {
        *self = make_unit_vector(*self)
    }

    pub fn x(&self) -> f32 {
        self.vector[0]
    }
    pub fn y(&self) -> f32 {
        self.vector[1]
    }
    pub fn z(&self) -> f32 {
        self.vector[2]
    }
    pub fn r(&self) -> f32 {
        self.vector[0]
    }
    pub fn g(&self) -> f32 {
        self.vector[1]
    }
    pub fn b(&self) -> f32 {
        self.vector[2]
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.vector[0] * v.vector[0] + u.vector[1] * v.vector[1] + u.vector[2] * v.vector[2]
}

fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.vector[1] * v.vector[2] - u.vector[2] * v.vector[1],
        -(u.vector[0] * v.vector[2] - u.vector[2] * v.vector[0]),
        u.vector[0] * v.vector[1] - u.vector[1] * v.vector[0],
    )
}

fn length(v: Vec3) -> f32 {
    (v.vector[0].powi(2) + v.vector[1].powi(2) + v.vector[2].powi(2)).sqrt()
}

pub fn squared_length(v: Vec3) -> f32 {
    v.vector[0].powi(2) + v.vector[1].powi(2) + v.vector[2].powi(2)
}

pub fn make_unit_vector(v: Vec3) -> Vec3 {
    let k = 1.0 / v.length();
    v * k
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "[{}, {}, {}]",
            self.vector[0], self.vector[1], self.vector[2]
        )
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3 {
            vector: [
                self.vector[0] + rhs.vector[0],
                self.vector[1] + rhs.vector[1],
                self.vector[2] + rhs.vector[2],
            ],
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.vector = [
            self.vector[0] + rhs.vector[0],
            self.vector[1] + rhs.vector[1],
            self.vector[2] + rhs.vector[2],
        ]
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            vector: [
                self.vector[0] - rhs.vector[0],
                self.vector[1] - rhs.vector[1],
                self.vector[2] - rhs.vector[2],
            ],
        }
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.vector = [
            self.vector[0] - rhs.vector[0],
            self.vector[1] - rhs.vector[1],
            self.vector[2] - rhs.vector[2],
        ]
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            vector: [-self.vector[0], -self.vector[1], -self.vector[2]],
        }
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        &self.vector[i]
    }
}

impl std::ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) -> () {
        self.vector = [
            rhs.vector[0] * self.vector[0],
            rhs.vector[1] * self.vector[1],
            rhs.vector[2] * self.vector[2],
        ]
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) -> () {
        self.vector = [
            self.vector[0] * rhs,
            self.vector[1] * rhs,
            self.vector[2] * rhs,
        ]
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self {
        Vec3 {
            vector: [
                self.vector[0] * rhs.vector[0],
                self.vector[1] * rhs.vector[1],
                self.vector[2] * rhs.vector[2],
            ],
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Vec3 {
            vector: [
                self.vector[0] * rhs,
                self.vector[1] * rhs,
                self.vector[2] * rhs,
            ],
        }
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            vector: [self * rhs[0], self * rhs[1], self * rhs[2]],
        }
    }
}

impl std::ops::Div for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Vec3 {
        Vec3 {
            vector: [
                self.vector[0] / rhs.vector[0],
                self.vector[1] / rhs.vector[1],
                self.vector[2] / rhs.vector[2],
            ],
        }
    }
}
impl std::ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            vector: [
                self.vector[0] / rhs,
                self.vector[1] / rhs,
                self.vector[2] / rhs,
            ],
        }
    }
}

impl std::ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) -> () {
        self.vector = [
            rhs.vector[0] / self.vector[0],
            rhs.vector[1] / self.vector[1],
            rhs.vector[2] / self.vector[2],
        ]
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) -> () {
        self.vector = [
            self.vector[0] / rhs,
            self.vector[1] / rhs,
            self.vector[2] / rhs,
        ]
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self[0] == other[0] && self[1] == other[1] && self[2] == other[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vec() {
        let a = Vec3::new(1.0, 1.0, 1.0);
        let b = Vec3::new(2.0, 2.0, 2.0);
        let c = a + b;
        assert_eq!(Vec3::new(3.0, 3.0, 3.0), c);
    }

    #[test]
    fn test_mul_vec_float() {
        let a = Vec3::new(1.0, 1.0, 1.0);
        let s = 2.0;
        assert_eq!(a * s, Vec3::new(2.0, 2.0, 2.0));
        assert_eq!(s * a, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_mul_vec_vec() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(a * b, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_div_vec() {
        let a = Vec3::new(2.0, 4.0, 6.0);
        let b = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(a / b, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_dot() {
        let a = Vec3::new(1.0, 3.0, -5.0);
        let b = Vec3::new(4.0, -2.0, -1.0);
        assert_eq!(dot(a, b), 3.0);
    }

    #[test]
    fn test_cross() {
        let a = Vec3::new(1.0, 3.0, -5.0);
        let b = Vec3::new(4.0, -2.0, -1.0);
        assert_eq!(cross(a, b), Vec3::new(-13.0, -19.0, -14.0))
    }

    #[test]
    fn test_make_unit_vector() {
        let a = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(
            make_unit_vector(a),
            Vec3::new(1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0)
        )
    }

    #[test]
    fn test_squared_length() {
        let a = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(squared_length(a), 9.0);
    }

    #[test]
    fn test_length() {
        let a = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(length(a), 3.0);
    }
}
