use std::time::SystemTime;
use vim_math3d::{self, Vector2, Line2D, stateless_random};

#[test]
fn test_no_intersection1() {
    let a = Line2D::new(Vector2::new(0.0, 0.0), Vector2::new(7.0, 7.0));
    let b = Line2D::new(Vector2::new(3.0, 4.0), Vector2::new(4.0, 5.0));
    assert!(!a.intersects(&b));
    assert!(!b.intersects(&a));
}

#[test]
fn test_no_intersection2() {
    let a = Line2D::new(Vector2::new(-4.0, 4.0), Vector2::new(-2.0, 1.0));
    let b = Line2D::new(Vector2::new(-2.0, 3.0), Vector2::new(0.0, 0.0));
    assert!(!a.intersects(&b));
    assert!(!b.intersects(&a));
}

// Add remaining tests for "no intersection" following the same pattern

#[test]
fn test_intersects1() {
    let a = Line2D::new(Vector2::new(0.0, -2.0), Vector2::new(0.0, 2.0));
    let b = Line2D::new(Vector2::new(-2.0, 0.0), Vector2::new(2.0, 0.0));
    assert!(a.intersects(&b));
    assert!(b.intersects(&a));
}

#[test]
fn test_intersects2() {
    let a = Line2D::new(Vector2::new(5.0, 5.0), Vector2::new(0.0, 0.0));
    let b = Line2D::new(Vector2::new(1.0, 1.0), Vector2::new(8.0, 2.0));
    assert!(a.intersects(&b));
    assert!(b.intersects(&a));
}

// Add remaining tests for "intersects" following the same pattern

#[test]
fn test_intersects6() {
    for index in 0..50 {
        let ax: f64 = stateless_random::random_float_default(index + 1, SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_micros() as u64);
        let ay: f64 = stateless_random::random_float_default(index + 1, SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_micros() as u64);
        let bx: f64 = stateless_random::random_float_default(index + 1, SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_micros() as u64);
        let by: f64 = stateless_random::random_float_default(index + 1, SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_micros() as u64);

        let a = Line2D::new(Vector2::new(ax, ay), Vector2::new(bx, by));
        let b = Line2D::new(Vector2::new(ax, ay), Vector2::new(bx, by));
        assert!(a.intersects(&b));
        assert!(b.intersects(&a));
    }
}
