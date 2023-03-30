pub use vim_math3d::{Vector2};

fn main() {
    let sum: Vector2<f64> = Vector2::new(2.0, 3.0);
    println!("Hello, world! {:?}", sum.set_x(3.0).magnitude());
}
