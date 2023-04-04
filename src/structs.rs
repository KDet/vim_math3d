use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub};
use num_traits::{Float, Zero, One };

use vim_math3d_macro_derive::{StructOps, VectorOps, IntervalOps,  VectorStructOps};

#[derive(Debug, Clone, Copy, PartialEq, StructOps, IntervalOps)]
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

pub enum ContainmentType {
    // Indicates that there is no overlap between two bounding volumes.
    Disjoint,
    // Indicates that one bounding volume completely contains another volume.
    Contains,
    // Indicates that bounding volumes partially overlap one another.
    Intersects,
}

pub enum PlaneIntersectionType
{
    /// <summary>
    /// There is no intersection, the bounding volume is in the negative half space of the plane.
    /// </summary>
    Front,
    /// <summary>
    /// There is no intersection, the bounding volume is in the positive half space of the plane.
    /// </summary>
    Back,
    /// <summary>
    /// The plane is intersected.
    /// </summary>
    Intersecting
}

// pub trait Transformable3D<T: Float> {
//     type SelfType;

//     fn transform(&self, mat: Matrix4x4<T>) -> Self::SelfType;
// }

impl<T: Float + std::ops::AddAssign> AABox<T> {
    pub fn count(&self) -> usize { 2 }
    pub fn center_bottom(&self) -> Vector3<T> { self.center().set_z(self.min.z) }
    pub fn corners(&self) -> [Vector3<T>; 8] { self.get_corners() }
    pub fn is_empty(&self) -> bool { !self.is_valid() }
    pub fn is_valid(&self) -> bool { self.min.x <= self.max.x && self.min.y <= self.max.y && self.min.z <= self.max.z }
    pub fn distance(&self, point: Vector3<T>) -> T { Vector3::zero().max(self.min - point).max(point - self.max).length() }
    pub fn center_distance(&self, point: Vector3<T>) -> T { self.center().distance(point) }
    pub fn translate(&self, offset: Vector3<T>) -> Self { Self { min: self.min + offset, max: self.max + offset } }
    pub fn distance_to_origin(&self) -> T { self.distance(Vector3::zero()) }
    pub fn center_distance_to_origin(&self) -> T { self.center_distance(Vector3::zero()) }
    pub fn volume(&self) -> T { if self.is_empty() { T::zero() } else { self.extent().product_components() } }
    pub fn max_side(&self) -> T { self.extent().max_component() }
    pub fn max_face_area(&self) -> T {
        if self.extent().x > self.extent().y { self.extent().x * self.extent().z.max(self.extent().y) } 
        else { self.extent().y * self.extent().z.max(self.extent().x) }
    }
    pub fn min_side(&self) -> T { self.extent().min_component() }
    pub fn diagonal(&self) -> T { self.extent().length() }
    pub fn contains(&self, other: &AABox<T>) -> ContainmentType {
        if other.max.x < self.min.x
            || other.min.x > self.max.x
            || other.max.y < self.min.y
            || other.min.y > self.max.y
            || other.max.z < self.min.z
            || other.min.z > self.max.z
        {
            ContainmentType::Disjoint
        } else if other.min.x >= self.min.x
            && other.max.x <= self.max.x
            && other.min.y >= self.min.y
            && other.max.y <= self.max.y
            && other.min.z >= self.min.z
            && other.max.z <= self.max.z
        {
            ContainmentType::Contains
        } else {
            ContainmentType::Intersects
        }
    }
    pub fn contains_sphere(&self, sphere: &Sphere<T>) -> ContainmentType {
        if sphere.center.x - self.min.x >= sphere.radius
            && sphere.center.y - self.min.y >= sphere.radius
            && sphere.center.z - self.min.z >= sphere.radius
            && self.max.x - sphere.center.x >= sphere.radius
            && self.max.y - sphere.center.y >= sphere.radius
            && self.max.z - sphere.center.z >= sphere.radius
        {
            return ContainmentType::Contains;
        }
    
        let mut dmin = T::zero();
        let mut e = sphere.center.x - self.min.x;
        if e < T::zero() {
            if e < -sphere.radius {
                return ContainmentType::Disjoint;
            }
            dmin += e * e;
        } else {
            e = sphere.center.x - self.max.x;
            if e > T::zero() {
                if e > sphere.radius {
                    return ContainmentType::Disjoint;
                }
                dmin += e * e;
            }
        }
    
        e = sphere.center.y - self.min.y;
        if e < T::zero() {
            if e < -sphere.radius {
                return ContainmentType::Disjoint;
            }
            dmin += e * e;
        } else {
            e = sphere.center.y - self.max.y;
            if e > T::zero() {
                if e > sphere.radius {
                    return ContainmentType::Disjoint;
                }
                dmin += e * e;
            }
        }
    
        e = sphere.center.z - self.min.z;
        if e < T::zero() {
            if e < -sphere.radius {
                return ContainmentType::Disjoint;
            }
            dmin += e * e;
        } else {
            e = sphere.center.z - self.max.z;
            if e > T::zero() {
                if e > sphere.radius {
                    return ContainmentType::Disjoint;
                }
                dmin += e * e;
            }
        }
    
        if dmin <= sphere.radius * sphere.radius {
            return ContainmentType::Intersects;
        }
    
        ContainmentType::Disjoint
    }   
    pub fn contains_point(&self, point: &Vector3<T>) -> bool {
        !(point.x < self.min.x
            || point.x > self.max.x
            || point.y < self.min.y
            || point.y > self.max.y
            || point.z < self.min.z
            || point.z > self.max.z)
    }

    pub fn create(points: Option<&[Vector3<T>]>) -> AABox<T> {
        let mut min_vec = Vector3::new(T::max_value(), T::max_value(), T::max_value());
        let mut max_vec = Vector3::new(T::min_value(), T::min_value(), T::min_value());
        if let Some(points) = points {
            for pt in points {
                min_vec = min_vec.min(*pt);
                max_vec = max_vec.max(*pt);
            }
        }
        AABox::new(min_vec, max_vec)
    }
    ////#[inline(always)]
    pub fn create_from_sphere(sphere: Sphere<T>) -> AABox<T> {
        AABox::new(sphere.center - Vector3::create_from_value(sphere.radius), sphere.center + Vector3::create_from_value(sphere.radius))
    }

    /// This is the four front corners followed by the four back corners all as if looking from the front
    /// going in counter-clockwise order from bottom left. 
    pub fn get_corners(&self) -> [Vector3<T>; 8] { [
        // Bottom (looking down)
        Vector3::new(self.min.x, self.min.y, self.min.z),
        Vector3::new(self.max.x, self.min.y, self.min.z),
        Vector3::new(self.max.x, self.max.y, self.min.z),
        Vector3::new(self.min.x, self.max.y, self.min.z),
        // Top (looking down)
        Vector3::new(self.min.x, self.min.y, self.max.z),
        Vector3::new(self.max.x, self.min.y, self.max.z),
        Vector3::new(self.max.x, self.max.y, self.max.z),
        Vector3::new(self.min.x, self.max.y, self.max.z), ]
    }

    pub const TOP_INDICES: [usize; 4] = [0, 1, 2, 3];
    pub const BOTTOM_INDICES: [usize; 4] = [7, 6, 5, 4];
    pub const FRONT_INDICES: [usize; 4] = [4, 5, 1, 0];
    pub const RIGHT_INDICES: [usize; 4] = [5, 6, 2, 1];
    pub const BACK_INDICES: [usize; 4] = [6, 7, 3, 2];
    pub const LEFT_INDICES: [usize; 4] = [7, 4, 0, 3];

    pub fn intersects(&self, other: &AABox<T>) -> bool {
        if self.max.x >= other.min.x && self.min.x <= other.max.x {
            if self.max.y < other.min.y || self.min.y > other.max.y {
                return false;
            }
            return self.max.z >= other.min.z && self.min.z <= other.max.z;
        }
        return false;
    }
    pub fn intersects_sphere(&self, sphere: &Sphere<T>) -> bool {
        if sphere.center.x - self.min.x > sphere.radius
            && sphere.center.y - self.min.y > sphere.radius
            && sphere.center.z - self.min.z > sphere.radius
            && self.max.x - sphere.center.x > sphere.radius
            && self.max.y - sphere.center.y > sphere.radius
            && self.max.z - sphere.center.z > sphere.radius
        {
            return true;
        }
    
        let mut dmin = T::zero();
        if sphere.center.x - self.min.x <= sphere.radius {
            dmin += (sphere.center.x - self.min.x) * (sphere.center.x - self.min.x);
        } else if self.max.x - sphere.center.x <= sphere.radius {
            dmin += (sphere.center.x - self.max.x) * (sphere.center.x - self.max.x);
        }
    
        if sphere.center.y - self.min.y <= sphere.radius {
            dmin += (sphere.center.y - self.min.y) * (sphere.center.y - self.min.y);
        } else if self.max.y - sphere.center.y <= sphere.radius {
            dmin += (sphere.center.y - self.max.y) * (sphere.center.y - self.max.y);
        }
    
        if sphere.center.z - self.min.z <= sphere.radius {
            dmin += (sphere.center.z - self.min.z) * (sphere.center.z - self.min.z);
        } else if self.max.z - sphere.center.z <= sphere.radius {
            dmin += (sphere.center.z - self.max.z) * (sphere.center.z - self.max.z);
        }
    
        if dmin <= sphere.radius * sphere.radius {
            return true;
        }
    
        false
    }
    pub fn intersects_plane(&self, plane: &Plane<T>) -> PlaneIntersectionType {
        let (mut ax, mut ay, mut az, mut bx, mut by, mut bz) = (T::zero(), T::zero(), T::zero(), T::zero(), T::zero(), T::zero());
    
        if plane.normal.x >= T::zero() {
            ax = self.max.x;
            bx = self.min.x;
        } else {
            ax = self.min.x;
            bx = self.max.x;
        }
    
        if plane.normal.y >= T::zero() {
            ay = self.max.y;
            by = self.min.y;
        } else {
            ay = self.min.y;
            by = self.max.y;
        }
    
        if plane.normal.z >= T::zero() {
            az = self.max.z;
            bz = self.min.z;
        } else {
            az = self.min.z;
            bz = self.max.z;
        }
    
        let distance = plane.normal.x * bx + plane.normal.y * by + plane.normal.z * bz + plane.d;
        if distance > T::zero() {
            return PlaneIntersectionType::Front;
        }
    
        let distance = plane.normal.x * ax + plane.normal.y * ay + plane.normal.z * az + plane.d;
        if distance < T::zero() {
            return PlaneIntersectionType::Back;
        }
    
        PlaneIntersectionType::Intersecting
    }

    pub fn unit() -> AABox::<T> {
        AABox::<T> {
            min: Vector3::<T>::zero(),
            max: Vector3::<T>::new(T::one(), T::one(), T::one()),
        }
    }
    
    /// Returns where a point is relative to the bounding box on a scale of 0..1
    pub fn relative_position(&self, v: Vector3<T>) -> Vector3<T> { v.inverse_lerp(self.min, self.max) }

    /// Moves the box so that it's origin is on the center
    pub fn recenter(&self) -> AABox<T> { self.translate(-self.center()) }

    /// Rescales the box
    pub fn scale(&self, scale: T) -> AABox<T> {
        let recentered = self.recenter();
        AABox::new(
            recentered.min * scale,
            recentered.max * scale
        ).translate(self.center())
    }

    /// Returns the center of each face.
    pub fn face_centers(&self) -> [Vector3<T>; 6] {
        let corners = self.get_corners();
        [
            corners[Self::FRONT_INDICES[0]].average(corners[Self::FRONT_INDICES[2]]),
            corners[Self::RIGHT_INDICES[0]].average(corners[Self::RIGHT_INDICES[2]]),
            corners[Self::BACK_INDICES[0]].average(corners[Self::BACK_INDICES[2]]),
            corners[Self::LEFT_INDICES[0]].average(corners[Self::LEFT_INDICES[2]]),
            corners[Self::TOP_INDICES[0]].average(corners[Self::TOP_INDICES[2]]),
            corners[Self::BOTTOM_INDICES[0]].average(corners[Self::BOTTOM_INDICES[2]]),
        ]
    }

    /// Returns corners and face centers as an iterator.
    pub fn get_corners_and_face_centers(&self) -> [Vector3<T>; 14] {
        let corners = self.get_corners();
        let face_centers = self.face_centers();
        [
            corners[0],
            corners[1],
            corners[2],
            corners[3],
            corners[4],
            corners[5],
            corners[6],
            corners[7],
            
            face_centers[0],
            face_centers[1],
            face_centers[2],
            face_centers[3],
            face_centers[4],
            face_centers[5],
        ] 
    }

    /// Returns the enclosing bounding sphere.
    //#[inline(always)]
    pub fn to_sphere(&self) -> Sphere<T> {
        Sphere::create(*self)
    }

    /// Given a normalized position in bounding box, returns the actual position.
    //#[inline(always)]
    pub fn lerp(&self, v: Vector3<T>) -> Vector3<T> {
        self.min + self.extent() * v
    }

    /// Sets the center of the box.
    //#[inline(always)]
    pub fn set_center(&self, v: Vector3<T>) -> AABox<T> {
        Self::from_center_and_extent(v, self.extent())
    }

    /// Sets the extent of the box.
    //#[inline(always)]
    pub fn set_extent(&self, v: Vector3<T>) -> AABox<T> {
        Self::from_center_and_extent(self.center(), v)
    }

    /// Creates a new axis-aligned bounding box from its center and extent.
    //#[inline(always)]
    pub fn from_center_and_extent(center: Vector3<T>, extent: Vector3<T>) -> AABox<T> {
        let half_extent = extent / (T::from(2).unwrap());
        AABox::new(center - half_extent, center + half_extent)
    }

    /// Transforms the box by a 4x4 matrix.
    //#[inline(always)]
    pub fn transform(&self, mat: Matrix4<T>) -> AABox<T> {
        Self::create(self.corners().iter().map(|v| v.transform(mat)))
    }

}
