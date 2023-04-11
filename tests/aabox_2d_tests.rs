use vim_math3d::{self, AABox2D, Vector2};

// No intersection tests
#[test]
fn test_no_intersection1() {
    let box1 = AABox2D::new(Vector2::new(0.0, 0.0), Vector2::new(3.0, 3.0));
    let box2 = AABox2D::new(Vector2::new(4.0, 4.0), Vector2::new(5.0, 5.0));
    assert!(!box1.intersects(&box2));
    assert!(!box2.intersects(&box1));
}

#[test]
fn test_no_intersection2() {
    let box1 = AABox2D::new(Vector2::new(0.0, 0.0), Vector2::new(3.0, 3.0));
    let box2 = AABox2D::new(Vector2::new(-5.0, -5.0), Vector2::new(-4.0, -4.0));
    assert!(!box1.intersects(&box2));
    assert!(!box2.intersects(&box1));
}

// Intersection tests
#[test]
fn test_intersects1() {
    let box1 = AABox2D::new(Vector2::new(0.0, 0.0), Vector2::new(5.0, 5.0));
    let box2 = AABox2D::new(Vector2::new(1.0, 1.0), Vector2::new(2.0, 2.0));
    assert!(box1.intersects(&box2));
    assert!(box2.intersects(&box1));
}

#[test]
fn test_intersects2() {
    let box1 = AABox2D::new(Vector2::new(0.0, 0.0), Vector2::new(3.0, 3.0));
    let box2 = AABox2D::new(Vector2::new(1.0, -1.0), Vector2::new(2.0, 7.0));
    assert!(box1.intersects(&box2));
    assert!(box2.intersects(&box1));
}

#[test]
fn test_intersects3() {
    let box1 = AABox2D::new(Vector2::new(0.0, 0.0), Vector2::new(3.0, 3.0));
    let box2 = AABox2D::new(Vector2::new(1.0, -1.0), Vector2::new(2.0, 2.0));
    assert!(box1.intersects(&box2));
    assert!(box2.intersects(&box1));
}

#[test]
fn test_intersects4() {
    let box1 = AABox2D::new(Vector2::new(0.0, 0.0), Vector2::new(3.0, 3.0));
    let box2 = AABox2D::new(Vector2::new(3.0, 3.0), Vector2::new(5.0, 5.0));
    assert!(box1.intersects(&box2));
    assert!(box2.intersects(&box1));
}
 