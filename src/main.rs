pub use vim_math3d;

fn main() {
    let sum = vim_math3d::Vector2::new(2.0, 3.0);
    println!("Hello, world! {:?}", sum.magnitude());
}
