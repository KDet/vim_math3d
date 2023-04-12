use vim_math3d::{self, Vector3, Ray, AABox, Triangle};
use std::time::Instant;

#[test]
fn ray_intersect_box_is_false_outside_box() {
    let ray = Ray::new(Vector3::new(-2.0f32, 0.0, -2.0), Vector3::new(0.0, 0.0, 1.0));
    let box_ = AABox::new(Vector3::new(-1.0f32, -1.0, -1.0), Vector3::new(1.0, 1.0, 1.0));
    assert!(ray.intersects_box(&box_).is_none());
}

#[test]
fn ray_intersect_box_is_true_through() {
    let rays = [
        Ray::new(Vector3::new(0.0f32, 0.0, -2.0), Vector3::new(0.0, 0.0, 1.0)),
        Ray::new(Vector3::new(0.0f32, 0.0, 2.0), Vector3::new(0.0, 0.0, -1.0)),
        Ray::new(Vector3::new(-2.0f32, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0)),
        Ray::new(Vector3::new(2.0f32, 0.0, 0.0), Vector3::new(-1.0, 0.0, 0.0)),
        Ray::new(Vector3::new(0.0f32, 2.0, 0.0), Vector3::new(0.0, -1.0, 0.0)),
        Ray::new(Vector3::new(0.0f32, -2.0, 0.0), Vector3::new(0.0, 1.0, 0.0)),
    ];

    let box_ = AABox::new(Vector3::new(-1.0f32, -1.0, -1.0), Vector3::new(1.0, 1.0, 1.0));

    for ray in &rays {
        assert!(ray.intersects_box(&box_).is_some());
    }
}

#[test]
fn ray_intersect_box_is_true_through_diagonals() {
    let rays = [
        Ray::new(Vector3::new(2.0f32, 2.0, -2.0), Vector3::new(-1.0, -1.0, 1.0)),
        Ray::new(Vector3::new(-2.0f32, 2.0, -2.0), Vector3::new(1.0, -1.0, 1.0)),
        Ray::new(Vector3::new(-2.0f32, -2.0, -2.0), Vector3::new(1.0, 1.0, 1.0)),
        Ray::new(Vector3::new(2.0f32, -2.0, -2.0), Vector3::new(-1.0, 1.0, 1.0)),
    ];

    let box_ = AABox::new(Vector3::new(-1.0f32, -1.0, -1.0), Vector3::new(1.0, 1.0, 1.0));

    for ray in &rays {
        assert!(ray.intersects_box(&box_).is_some());
    }
}

#[test]
fn ray_intersect_box_is_false_away_from_box() {
    let sides = [
        Ray {position:Vector3::new(0.0f32,0.0, -2.0),direction:Vector3::new(0.0f32,0.0, -1.0)},
        Ray {position: Vector3::new(0.0f32, 0.0, 2.0), direction: Vector3::new(0.0f32, 0.0, 1.0)},
        Ray {position: Vector3::new(-2.0f32, 0.0, 0.0), direction: Vector3::new(-1.0f32, 0.0, 0.0)},
        Ray {position: Vector3::new(2.0f32, 0.0, 0.0), direction: Vector3::new(1.0f32, 0.0, 0.0)},
        Ray {position: Vector3::new(0.0f32, 2.0, 0.0), direction: Vector3::new(0.0f32, 1.0, 0.0)},
        Ray {position: Vector3::new(0.0f32, -2.0, 0.0), direction: Vector3::new(0.0f32, -1.0, 0.0)},
    ];

    let aabox = AABox {
        min: Vector3::new(-1.0f32, -1.0, -1.0),
        max: Vector3::new(1.0f32, 1.0, 1.0),
    };

    for ray in &sides {
        assert!(ray.intersects_box(&aabox).is_none());
    }
}

#[test]
fn ray_intersect_box_is_true_on_edge() {
    let sides = [
        Ray::new(Vector3::new(0.0f32, 2.0, -1.0), Vector3::new(0.0f32, -1.0, 0.0)),
        Ray::new(Vector3::new(0.0f32, 2.0, 1.0), Vector3::new(0.0f32, -1.0, 0.0)),
        Ray::new(Vector3::new(-1.0f32, 0.0, -2.0), Vector3::new(0.0f32, 0.0, 1.0)),
        Ray::new(Vector3::new(1.0f32, 0.0, -2.0), Vector3::new(0.0f32, 0.0, 1.0)),
        Ray::new(Vector3::new(0.0f32, 1.0, -2.0), Vector3::new(0.0f32, 0.0, 1.0)),
        Ray::new(Vector3::new(0.0f32, -1.0, -2.0), Vector3::new(0.0f32, 0.0, 1.0)),
    ];

    let box_ = AABox::new(Vector3::new(-1.0f32, -1.0, -1.0), Vector3::new(1.0f32, 1.0, 1.0));

    for ray in &sides {
        assert!(ray.intersects_box(&box_).is_some());
    }
}

#[test]
fn ray_intersect_box_is_false_near_edge() {
    let ray = Ray::new(Vector3::new(0.0f32, 0.0, -2.0), Vector3::new(0.0f32, 1.1, 1.0));
    let box_ = AABox::new(Vector3::new(-1.0f32, -1.0, -1.0), Vector3::new(1.0f32, 1.0, 1.0));

    assert!(ray.intersects_box(&box_).is_none());
}

#[test]
fn ray_intersect_box_is_true_flat_box() {
    let box_ = AABox::new(Vector3::new(-1.0f32, -1.0, 0.0), Vector3::new(1.0f32, 1.0, 0.0));
    let ray = Ray::new(Vector3::new(0.0f32, 0.0, -1.0), Vector3::new(0.0f32, 0.0, 1.0));

    assert!(ray.intersects_box(&box_).is_some());
}


#[test]
fn ray_intersect_triangle_is_true_inside() {
    let ray = Ray::new(Vector3::new(0.0f32, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));

    let triangle = Triangle::new(
        Vector3::new(0.0f32, 1.0, 0.0),
        Vector3::new(1.0f32, -1.0, 0.0),
        Vector3::new(-1.0f32, -1.0, 0.0),
    );

    assert!(ray.intersects_triangle(&triangle, None).is_some());
}

#[test]
fn ray_intersect_triangle_is_false_parallel() {
    let ray = Ray::new(Vector3::new(0.0f32, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));

    let triangle = Triangle::new(
        Vector3::new(1.0f32, 0.0, 0.0),
        Vector3::new(-1.0f32, 0.0, 0.0),
        Vector3::new(0.0f32, 0.0, 1.0),
    );

    assert!(ray.intersects_triangle(&triangle, None).is_none());
}

#[test]
fn ray_intersect_triangle_is_true_on_corner() {
    let ray = Ray::new(Vector3::new(0.0f32, 1.0, -1.0), Vector3::new(0.0f32, 0.0, 1.0));

    let triangle = Triangle::new(
        Vector3::new(0.0f32, 1.0, 0.0),
        Vector3::new(1.0f32, -1.0, 0.0),
        Vector3::new(-1.0f32, -1.0, 0.0),
    );

    assert!(ray.intersects_triangle(&triangle, None).is_some());
}

#[test]
fn ray_intersect_triangle_is_false_in_tricky_corner() {
    let ray = Ray::new(Vector3::new(-0.1f32, 0.0, -1.0), Vector3::new(0.0f32, 0.0, 1.0));
    let triangle = Triangle::new(
        Vector3::new(0.0f32, 0.0, 0.0),
        Vector3::new(-1.0f32, 1.0, 0.0),
        Vector3::new(1.0f32, 0.0, 0.0),
    );

    assert!(ray.intersects_triangle(&triangle, None).is_none());
}

#[test]
fn ray_intersect_triangle_perf_test() {
    let ray1 = Ray::new(Vector3::new(-0.1f32, 0.0, -1.0), Vector3::new(0.0f32, 0.0, 1.0));
    let triangle1 = Triangle::new(
        Vector3::new(0.0f32, 0.0, 0.0),
        Vector3::new(-1.0f32, 1.0, 0.0),
        Vector3::new(1.0f32, 0.0, 0.0),
    );

    let ray2 = Ray::new(Vector3::new(0.0f32, 1.0, -1.0), Vector3::new(0.0f32, 0.0, 1.0));
    let triangle2 = Triangle::new(
        Vector3::new(0.0f32, 1.0, 0.0),
        Vector3::new(1.0f32, -1.0, 0.0),
        Vector3::new(-1.0f32, -1.0, 0.0),
    );

    let ray3 = Ray::new(Vector3::new(0.0f32, 0.0, -1.0), Vector3::new(0.0, 0.0, 1.0));
    let triangle3 = Triangle::new(
        Vector3::new(1.0f32, 0.0, 0.0),
        Vector3::new(-1.0f32, 0.0, 0.0),
        Vector3::new(0.0f32, 0.0, 1.0),
    );

    let ray4 = Ray::new(Vector3::new(0.0f32, 0.0, -1.0), Vector3::new(0.0f32, 0.0, 1.0));
    let triangle4 = Triangle::new(
        Vector3::new(1.0f32, 0.0, 0.0),
        Vector3::new(-1.0f32, 0.0, 0.0),
        Vector3::new(0.0f32, 0.0, 1.0),
    );

    for _ in 0..10 {
        let start = Instant::now();
        for _ in 0..1_000_000 {
            let _ = ray1.intersects_triangle(&triangle1, None);
            let _ = ray2.intersects_triangle(&triangle2, None);
            let _ = ray3.intersects_triangle(&triangle3, None);
            let _ = ray4.intersects_triangle(&triangle4, None);
        }
        let duration = start.elapsed();
        println!("TomboreMoller {} ms", duration.as_millis());
    }
}

// cargo test --release -- --nocapture