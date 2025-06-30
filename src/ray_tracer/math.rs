use rand::Rng;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn near_zero(&self) -> bool {
        let lim = 1e-8;
        (self.x.abs() < lim) && (self.y.abs() < lim) && (self.z.abs() < lim)
    }

    pub fn reflect(&self, other: &Self) -> Vec3 {
        *self - (2.0 * Vec3::dot(self, other) * *other)
    }

    pub fn random(lower_bound: f64, upper_bound: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(lower_bound..=upper_bound),
            rng.gen_range(lower_bound..=upper_bound),
            rng.gen_range(lower_bound..=upper_bound),
        )
    }

    pub fn unit_random() -> Vec3 {
        loop {
            let v = Vec3::random(-1.0, 1.0);
            let v_len = v.length_squared();
            if 1e-160 < v_len && v_len <= 1.0 {
                return v / v_len.sqrt();
            }
        }
    }

    pub fn unit_random_hemisphere(n: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::unit_random();
        if on_unit_sphere.dot(&n) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: other.x() * self,
            y: other.y() * self,
            z: other.z() * self,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: other.x() * self.x(),
            y: other.y() * self.y(),
            z: other.z() * self.z(),
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::zero()
    }
}

