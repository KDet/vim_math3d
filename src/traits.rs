use num_traits::Float;
use crate::{Vector2,Vector3,Matrix4x4};

pub trait Transformable3D<T: Float> {
    type SelfType;

    fn transform(self, mat: &Matrix4x4<T>) -> Self::SelfType;
}

pub trait Points<T: Float> {
    fn num_points(&self) -> usize;
    fn get_point(&self, n: usize) -> Vector3<T>;
}

pub trait Points2D<T: Float> {
    fn num_points(&self) -> usize;
    fn get_point(&self, n: usize) -> Vector2<T>;
}

pub trait Mappable<TPart> {
    type Container;
    
    fn map<F>(self, f: F) -> Self::Container
    where
        F: Fn(TPart) -> TPart;
}