mod structs;
pub mod math3d_ops;

pub use structs::*;
//pub use math3d_ops::*;
//pub use num_traits::{Zero, One};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a: bool = false;
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
