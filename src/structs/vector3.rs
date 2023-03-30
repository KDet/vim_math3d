use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub};
use num_traits::Float;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vector3<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vector3<T> {
    pub const NUM_COMPONENTS: usize = 2;

    pub fn zero() -> Self { Self { x: T::zero(), y: T::zero(), z: T::zero() } }
    pub fn one() -> Self { Self { x: T::one(), y: T::one(), z: T::one() } }
    pub fn unit_x() -> Self { Self { x: T::one(), y: T::zero(), z: T::zero() } }
    pub fn unit_y() -> Self { Self { x: T::zero(), y: T::one(), z: T::zero() } }
    pub fn unit_z() -> Self { Self { x: T::zero(), y: T::zero(), z: T::one() } }

    pub fn new(x: T, y: T, z: T) -> Self { Self { x, y, z } }
    pub fn from_value(v: T) -> Self { Self { x: v, y: v, z: v } }

    pub fn min_value() -> Self { Self { x: T::min_value(), y: T::min_value(), z: T::min_value() } }
    pub fn max_value() -> Self { Self { x: T::max_value(), y: T::max_value(), z: T::max_value() } }

    pub fn set_x(&self, x: T) -> Self { Self { x, y: self.y, z: self.z } }
    pub fn set_y(&self, y: T) -> Self { Self { x: self.x, y, z: self.z } }

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn almost_equals(&self, other: &Self, tolerance: T) -> bool {
        (self.x - other.x).abs() < tolerance && (self.y - other.y).abs() < tolerance
    }
    pub fn almost_zero(&self, tolerance: T) -> bool {
        self.x.abs() < tolerance && self.y.abs() < tolerance
    }

    pub fn any_component_negative(&self) -> bool {
        self.min_component() < T::zero()
    }
    pub fn min_component(&self) -> T {
        self.x.min(self.y)
    }
    pub fn max_component(&self) -> T {
        self.x.max(self.y)
    }
    pub fn sum_components(&self) -> T {
        self.x + self.y
    }
    pub fn sum_sqr_components(&self) -> T {
        self.x.powi(2) + self.y.powi(2)
    }
    pub fn product_components(&self) -> T {
        self.x * self.y
    }
    pub fn get_component(&self, n: usize) -> Option<T> {
        match n {
            0 => Some(self.x),
            1 => Some(self.y),
            _ => None,
        }
    }
    pub fn magnitude_squared(&self) -> T {
        self.sum_sqr_components()
    }
    pub fn magnitude(&self) -> T {
        self.magnitude_squared().sqrt()
    }

    pub fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    pub fn is_infinity(&self) -> bool {
        self.x.is_infinite() || self.y.is_infinite()
    }
}

impl<T: Float> PartialOrd for Vector3<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.magnitude_squared().partial_cmp(&other.magnitude_squared())
    }
}

impl<T: Float> Add for Vector3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T: Float> Add<T> for Vector3<T> {
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        Self::new(self.x + other, self.y + other, self.z + other)
    }
}

// impl<T> Add<Vector3<T>> for T {
//     type Output = Vector3<T>;

//     fn add(self, other: Vector3<T>) -> Self::Output {
//         Vector3::new(self + other.x, self + other.y)
//     }
// }

impl<T: Float> Sub for Vector3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T: Float> Sub<T> for Vector3<T> {
    type Output = Self;

    fn sub(self, other: T) -> Self::Output {
        Self::new(self.x - other, self.y - other, self.z - other)
    }
}

// impl<T> Sub<Vector3<T>> for T {
//     type Output = Vector3<T>;

//     fn sub(self, other: Vector3<T>) -> Self::Output {
//         Vector3::new(self - other.x, self - other.y)
//     }
// }

impl<T: Float> Mul for Vector3<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl<T: Float> Mul<T> for Vector3<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Self::new(self.x * other, self.y * other, self.z * other)
    }
}

// impl<T: Float> Mul<Vector3<T>> for Box<T> where T: Float {
//     type Output = Vector3<T>;

//     fn mul(self, other: Vector3<T>) -> Self::Output {
//         Vector3::new(self * other.x, self * other.y)
//     }
// }

impl<T: Float> Div for Vector3<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl<T: Float> Div<T> for Vector3<T> {
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

// impl<T: Float> Div<Vector3<T>> for T {
//     type Output = Vector3<T>;

//     fn div(self, other: Vector3<T>) -> Self::Output {
//         Vector3::new(self / other.x, self / other.y)
//     }
// }

impl<T: Float> Neg for Vector3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector3::zero() - self
    }
}

impl<T: Float> From<(T, T, T)> for Vector3<T> {
    fn from(tuple: (T, T, T)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }
}

impl<T: Float> Into<(T, T, T)> for Vector3<T> {
    fn into(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }
}
