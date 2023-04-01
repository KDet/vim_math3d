use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub};
use num_traits::{Float, Zero, One };

use vim_math3d_macro_derive::{Struct, AlmostEq, FromTuple, FloatHash, UnaryOps};

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct AABox<T: Float = f32> {
    pub min: Vector3::<T>,
    pub max: Vector3::<T>,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct AABox2D<T: Float = f32> {
    pub min: Vector2::<T>,
    pub max: Vector2::<T>,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct AABox4D<T: Float = f32> {
    pub min: Vector4::<T>,
    pub max: Vector4::<T>,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct AxisAngle<T: Float = f64> {
    pub axis: Vector3::<T>,
    pub angle: T,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Byte2 {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Byte3 {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Byte4 {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub w: u8,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct ColorHDR {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct ColorRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct ColorRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Complex<T: Float = f64> {
    pub real: T,
    pub imaginary: T,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct CylindricalCoordinate<T: Float = f64> {
    pub radius: T,
    pub azimuth: T,
    pub height: T,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Euler<T: Float = f32> {
    pub yaw: T,
    pub pitch: T,
    pub roll: T,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct GeoCoordinate<T: Float = f64> {
    pub latitude: T,
    pub longitude: T,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct HorizontalCoordinate<T: Float = f64> {
    pub azimuth: T,
    pub inclination: T,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Int2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Int3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Int4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Interval<T: Float = f32> {
    pub min: T,
    pub max: T,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Line<T: Float = f32> {
    pub a: Vector3::<T>,
    pub b: Vector3::<T>,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Line2D<T: Float = f32> {
    pub a: Vector2::<T>,
    pub b: Vector2::<T>,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct LogPolarCoordinate<T: Float = f64> {
    pub rho: T,
    pub azimuth: T,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Plane<T: Float = f32> {
    pub normal: Vector3::<T>,
    pub d: T,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct PolarCoordinate<T: Float = f64> {
    pub radius: T,
    pub azimuth: T,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Quad<T: Float = f32> {
    pub a: Vector3::<T>,
    pub b: Vector3::<T>,
    pub c: Vector3::<T>,
    pub d: Vector3::<T>,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Quad2D<T: Float = f32> {
    pub a: Vector2::<T>,
    pub b: Vector2::<T>,
    pub c: Vector2::<T>,
    pub d: Vector2::<T>,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Quaternion<T: Float = f32> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Ray<T: Float = f32> {
    pub position: Vector3::<T>,
    pub direction: Vector3::<T>,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Sphere<T: Float = f32> {
    pub center: Vector3::<T>,
    pub radius: T,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct SphericalCoordinate<T: Float = f64> {
    pub radius: T,
    pub azimuth: T,
    pub inclination: T,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Transform<T: Float = f32> {
    pub position: Vector3::<T>,
    pub orientation: Quaternion::<T>,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Triangle<T: Float = f32> {
    pub a: Vector3::<T>,
    pub b: Vector3::<T>,
    pub c: Vector3::<T>,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash)]
pub struct Triangle2D<T: Float = f32> {
    pub a: Vector2::<T>,
    pub b: Vector2::<T>,
    pub c: Vector2::<T>,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash, AlmostEq)]
pub struct Vector2<T: Float> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash, AlmostEq)]
pub struct Vector3<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, PartialEq, Struct, FromTuple, FloatHash, AlmostEq, UnaryOps)]
pub struct Vector4<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}
// impl<T: Float> Vector4<T> {
//     pub fn inverse(self) -> Self
//         {
//             Self
//             {
//                 x : T :: one() / self, y : T :: one() / self, z : T :: one() /
//                 self, w : T :: one() / self,
//             }
//         }
// }
impl<T: Float> Vector2<T> {
    pub fn unit_x() -> Self { Self { x: T::one(), y: T::zero() } }
    pub fn unit_y() -> Self { Self { x: T::zero(), y: T::one() } }

    pub fn from_value(v: T) -> Self { Self { x: v, y: v } }

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
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

impl<T: Float> PartialOrd for Vector2<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.magnitude_squared().partial_cmp(&other.magnitude_squared())
    }
}

impl<T: Float> Add for Vector2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: Float> Add<T> for Vector2<T> {
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        Self::new(self.x + other, self.y + other)
    }
}

// impl<T> Add<Vector2<T>> for T {
//     type Output = Vector2<T>;
//     fn add(self, other: Vector2<T>) -> Self::Output {
//         Vector2::new(self + other.x, self + other.y)
//     }
// }

impl<T: Float> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: Float> Sub<T> for Vector2<T> {
    type Output = Self;

    fn sub(self, other: T) -> Self::Output {
        Self::new(self.x - other, self.y - other)
    }
}

// impl<T> Sub<Vector2<T>> for T {
//     type Output = Vector2<T>;
//     fn sub(self, other: Vector2<T>) -> Self::Output {
//         Vector2::new(self - other.x, self - other.y)
//     }
// }

impl<T: Float> Mul for Vector2<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.x * other.x, self.y * other.y)
    }
}

impl<T: Float> Mul<T> for Vector2<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

// impl<T: Float> Mul<Vector2<T>> for Box<T> where T: Float {
//     type Output = Vector2<T>;
//     fn mul(self, other: Vector2<T>) -> Self::Output {
//         Vector2::new(self * other.x, self * other.y)
//     }
// }

impl<T: Float> Div for Vector2<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y)
    }
}

impl<T: Float> Div<T> for Vector2<T> {
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        Self::new(self.x / scalar, self.y / scalar)
    }
}

// impl<T: Float> Div<Vector2<T>> for T {
//     type Output = Vector2<T>;
//     fn div(self, other: Vector2<T>) -> Self::Output {
//         Vector2::new(self / other.x, self / other.y)
//     }
// }

impl<T: Float> Neg for Vector2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector2::zero() - self
    }
}

// impl<T: Float> From<(T, T)> for Vector2<T> {
//     fn from(tuple: (T, T)) -> Self {
//         Self::new(tuple.0, tuple.1)
//     }
// }

// impl<T: Float> Into<(T, T)> for Vector2<T> {
//     fn into(self) -> (T, T) {
//         (self.x, self.y)
//     }
// }


impl<T: Float> Vector3<T> {

    pub fn unit_x() -> Self { Self { x: T::one(), y: T::zero(), z: T::zero() } }
    pub fn unit_y() -> Self { Self { x: T::zero(), y: T::one(), z: T::zero() } }
    pub fn unit_z() -> Self { Self { x: T::zero(), y: T::zero(), z: T::one() } }

    pub fn from_value(v: T) -> Self { Self { x: v, y: v, z: v } }

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
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

// impl<T: Float> From<(T, T, T)> for Vector3<T> {
//     fn from(tuple: (T, T, T)) -> Self {
//         Self::new(tuple.0, tuple.1, tuple.2)
//     }
// }

// impl<T: Float> Into<(T, T, T)> for Vector3<T> {
//     fn into(self) -> (T, T, T) {
//         (self.x, self.y, self.z)
//     }
// }

// public static bool AlmostEquals(this float v1, float v2, float tolerance = Constants.Tolerance) => (v2 - v1).AlmostZero(tolerance);
// public static bool AlmostZero(this float v, float tolerance = Constants.Tolerance) => v.Abs() < tolerance;

