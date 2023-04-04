use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub};
use num_traits::{Float, Zero, One };

use vim_math3d_macro_derive::{StructOps, VectorOps, IntervalOps,  VectorStructOps};

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct AABox<T: Float = f32> {
    pub min: Vector3::<T>,
    pub max: Vector3::<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps, IntervalOps)]
pub struct AABox2D<T: Float = f32> {
    pub min: Vector2::<T>,
    pub max: Vector2::<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps, IntervalOps)]
pub struct AABox4D<T: Float = f32> {
    pub min: Vector4::<T>,
    pub max: Vector4::<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct AxisAngle<T: Float = f64> {
    pub axis: Vector3::<T>,
    pub angle: T,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct Byte2 {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct Byte3 {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct Byte4 {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub w: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct ColorHDR {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct ColorRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct ColorRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps, VectorStructOps)]
pub struct Complex<T: Float = f64> {
    pub real: T,
    pub imaginary: T,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct CylindricalCoordinate<T: Float = f64> {
    pub radius: T,
    pub azimuth: T,
    pub height: T,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps, VectorStructOps)]
pub struct Euler<T: Float = f32> {
    pub yaw: T,
    pub pitch: T,
    pub roll: T,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps, VectorStructOps)]
pub struct GeoCoordinate<T: Float = f64> {
    pub latitude: T,
    pub longitude: T,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps, VectorStructOps)]
pub struct HorizontalCoordinate<T: Float = f64> {
    pub azimuth: T,
    pub inclination: T,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps, VectorStructOps)]
pub struct Int2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps, VectorStructOps)]
pub struct Int3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps, VectorStructOps)]
pub struct Int4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps, IntervalOps)]
pub struct Interval<T: Float = f32> {
    pub min: T,
    pub max: T,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct Line<T: Float = f32> {
    pub a: Vector3::<T>,
    pub b: Vector3::<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct Line2D<T: Float = f32> {
    pub a: Vector2::<T>,
    pub b: Vector2::<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct LogPolarCoordinate<T: Float = f64> {
    pub rho: T,
    pub azimuth: T,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct Plane<T: Float = f32> {
    pub normal: Vector3::<T>,
    pub d: T,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct PolarCoordinate<T: Float = f64> {
    pub radius: T,
    pub azimuth: T,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct Quad<T: Float = f32> {
    pub a: Vector3::<T>,
    pub b: Vector3::<T>,
    pub c: Vector3::<T>,
    pub d: Vector3::<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct Quad2D<T: Float = f32> {
    pub a: Vector2::<T>,
    pub b: Vector2::<T>,
    pub c: Vector2::<T>,
    pub d: Vector2::<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps, VectorStructOps)]
pub struct Quaternion<T: Float = f32> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct Ray<T: Float = f32> {
    pub position: Vector3::<T>,
    pub direction: Vector3::<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct Sphere<T: Float = f32> {
    pub center: Vector3::<T>,
    pub radius: T,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct SphericalCoordinate<T: Float = f64> {
    pub radius: T,
    pub azimuth: T,
    pub inclination: T,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct Transform<T: Float = f32> {
    pub position: Vector3::<T>,
    pub orientation: Quaternion::<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct Triangle<T: Float = f32> {
    pub a: Vector3::<T>,
    pub b: Vector3::<T>,
    pub c: Vector3::<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps)]
pub struct Triangle2D<T: Float = f32> {
    pub a: Vector2::<T>,
    pub b: Vector2::<T>,
    pub c: Vector2::<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps, VectorStructOps, VectorOps)]
pub struct Vector2<T: Float> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps, VectorStructOps, VectorOps)]
pub struct Vector3<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Clone, Copy, PartialEq, StructOps, VectorStructOps, VectorOps)]
pub struct Vector4<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}
