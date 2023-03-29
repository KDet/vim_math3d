use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::f32;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const NUM_COMPONENTS: usize = 2;
    pub const ZERO: Vector2 = Self { x: 0.0, y: 0.0 };
    pub const ONE: Vector2 =  Self { x: 1.0, y: 1.0 };
    pub const UNIT_X: Vector2 = Self { x: 1.0, y: 0.0 };
    pub const UNIT_Y: Vector2 = Self { x: 0.0, y: 1.0 };

    pub fn new(x: f32, y: f32) -> Self {  Self { x, y } }
    pub fn value(v: f32) -> Self { Self { x: v, y: v } }

    pub fn min_value() -> Self { Self { x: f32::MIN, y: f32::MIN } }
    pub fn max_value() -> Self { Self { x: f32::MAX, y: f32::MAX } }

    pub fn set_x(&self, x: f32) -> Self { Self { x, y: self.y } }
    pub fn set_y(&self, y: f32) -> Self { Self { x: self.x, y } }

    pub fn almost_equals(&self, other: &Self, tolerance: f32) -> bool {
        (self.x - other.x).abs() < tolerance && (self.y - other.y).abs() < tolerance
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    pub fn is_infinity(&self) -> bool {
        self.x.is_infinite() || self.y.is_infinite()
    }
}

impl PartialOrd for Vector2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.magnitude_squared().partial_cmp(&other.magnitude_squared())
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

impl Div for Vector2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y)
    }
}