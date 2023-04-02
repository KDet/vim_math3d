pub use vim_math3d::*;
use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};
//use num_traits::{Zero, One};

fn main() {
    let a1: AABox<f32> = AABox::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(4.0, 5.0, 6.0));
    let a2: AABox<f32> = AABox::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(4.0, 5.0, 6.0));
    let mut hasher = DefaultHasher::new();
    a1.hash(&mut hasher);
    a2.hash(&mut hasher);
    let hash_code = hasher.finish();
 
    println!("Hello, eq! {:?}", !math3d_ops::and(true, false));
    println!("Hello, hash_code! {:?}", math3d_ops::cube(2)); 

    let a3 = a1.set_max(Vector3::new(10.0, 20.0, 30.0));
    let v4 = Vector4::new(1.0, 2.0, 3.0, 10.0);
    let v5 = Vector4::new(1.0, 2.0, 3.0, 10.0);

    println!("Hello, v4 inverse! {:?}", v4.dot(&v5)); 
    println!("Hello, eq2! {:?}", a3 == a2);
    println!("Hello, float! {:?}", 0.0);

    println!("Hello, zero! {:?}", AABox::<f32>::zero());
    //println!("Hello, one! {:?}", mo);
    // Create a Vector3<f32> from a tuple of f32 values
    let vector: Vector3<f32> = (1.0, 2.0, 3.0).into();
    println!("vector = {:?}", vector);

    // Convert a Vector3<f32> back into a tuple of f32 values
    let tuple: (f32, f32, f32) = vector.into();
    println!("tuple = {:?}", tuple);

}
