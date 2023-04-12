use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

use num_traits::{Float, Num};
use vim_math3d::{self, Vector2, Line2D, stateless_random, Vector4, math3d_ops, Vector3, Matrix4x4, Transformable3D, Quaternion};

fn equal<T: Float>(a: T, b: T) -> bool {
    (a - b).abs() < T::from(1e-5).unwrap()
}

fn hash_code<T: Float>(v: &Vector4<T>) -> u64 {
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn vector4_marshal_size_test() {
    assert_eq!(16, std::mem::size_of::<Vector4<f32>>());
}

#[test]
fn vector4_get_hash_code_test() {
    let mut hasher = DefaultHasher::new();
    7920.hash(&mut hasher);
    println!("Hash is {:x}!", hasher.finish());

    let v1 = Vector4::new(2.5f32, 2.0f32, 3.0f32, 3.3f32);
    let v2 = Vector4::new(2.5f32, 2.0f32, 3.0f32, 3.3f32);
    let v3 = Vector4::new(2.5f32, 2.0f32, 3.0f32, 3.3f32);
    let v5 = Vector4::new(3.3f32, 3.0f32, 2.0f32, 2.5f32);
    assert_eq!(hash_code(&v1), hash_code(&v1));
    assert_eq!(hash_code(&v1), hash_code(&v2));
    assert_ne!(hash_code(&v1), hash_code(&v5));
    assert_eq!(hash_code(&v1), hash_code(&v3));
    let v4 = Vector4::new(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    let v6 = Vector4::new(1.0f32, 0.0f32, 0.0f32, 0.0f32);
    let v7 = Vector4::new(0.0f32, 1.0f32, 0.0f32, 0.0f32);
    let v8 = Vector4::new(1.0f32, 1.0f32, 1.0f32, 1.0f32);
    let v9 = Vector4::new(1.0f32, 1.0f32, 0.0f32, 0.0f32);
    assert_ne!(hash_code(&v4), hash_code(&v6));
    assert_ne!(hash_code(&v4), hash_code(&v7));
    assert_ne!(hash_code(&v4), hash_code(&v8));
    assert_ne!(hash_code(&v7), hash_code(&v6));
    assert_ne!(hash_code(&v8), hash_code(&v6));
    assert_ne!(hash_code(&v8), hash_code(&v7));
    assert_ne!(hash_code(&v9), hash_code(&v7));
}

#[test]
fn vector4_distance_squared_test() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let b = Vector4::new(5.0f32, 6.0f32, 7.0f32, 8.0f32);

    let expected = 64.0f32;
    let actual = a.distance_squared(b);
    assert!((expected - actual).abs() < std::f32::EPSILON, "Vector4f.DistanceSquared did not return the expected value.");
}

#[test]
fn vector4_distance_test() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let b = Vector4::new(5.0f32, 6.0f32, 7.0f32, 8.0f32);

    let expected = 8.0f32;
    let actual = a.distance(b);
    assert!((expected - actual).abs() < std::f32::EPSILON, "Vector4f.Distance did not return the expected value.");
}

// A test for Distance (Vector4f, Vector4f)
// Distance from the same point
#[test]
fn vector4_distance_test1() {
    let a = Vector4::new_from_vector2(Vector2::new(1.051f32, 2.05f32), 3.478f32, 1.0f32);
    let mut b = Vector4::new_from_vector3(Vector3::new(1.051f32, 2.05f32, 3.478f32), 0.0f32);
    b = b.set_w(1.0f32);

    let actual = a.distance(b);
    assert_eq!(0.0f32, actual);
}

// A test for Dot (Vector4f, Vector4f)
#[test]
fn vector4_dot_test() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let b = Vector4::new(5.0f32, 6.0f32, 7.0f32, 8.0f32);

    let expected = 70.0f32;
    let actual = a.dot(b);
    assert_eq!(expected, actual, "Vector4f.Dot did not return the expected value.");
}

// A test for Dot (Vector4f, Vector4f)
// Dot test for perpendicular vector
#[test]
fn vector4_dot_test1() {
    let a = Vector3::new(1.55f32, 1.55f32, 1.0f32);
    let b = Vector3::new(2.5f32, 3.0f32, 1.5f32);
    let c = a.cross(b);

    let d = Vector4::new_from_vector3(a, 0.0f32);
    let e = Vector4::new_from_vector3(c, 0.0f32);

    let actual = d.dot(e);
    assert!(equal(0.0f32, actual), "Vector4f.Dot did not return the expected value.");
}

#[test]
fn vector4_length_test() {
    let a = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let w = 4.0f32;

    let target = Vector4::new_from_vector3(a, w);

    let expected = 30.0f32.sqrt();
    let actual = target.length();

    assert_eq!(expected, actual,  "Vector4f.Length did not return the expected value.");
}

#[test]
fn vector4_length_test1() {
    let target = Vector4::zero();

    let expected = 0.0f32;
    let actual = target.length();

    assert_eq!(expected, actual, "Vector4f.Length did not return the expected value.");
}
/*** */
#[test]
fn vector4_length_squared_test() {
    let a = Vector3::new(1.0f32, 2.0, 3.0);
    let w = 4.0f32;

    let target = Vector4::new_from_vector3(a, w);

    let expected = 30.0f32;
    let actual = target.length_squared();

    assert_eq!(expected, actual, "Vector4f.LengthSquared did not return the expected value.");
}

#[test]
fn vector4_min_test() {
    let a = Vector4::new(-1.0f32, 4.0f32, -3.0f32, 1000.0f32);
    let b = Vector4::new(2.0f32, 1.0f32, -1.0f32, 0.0f32);

    let expected = Vector4::new(-1.0f32, 1.0f32, -3.0f32, 0.0f32);
    let actual = a.min(b);
    assert_eq!(expected, actual, "Vector4f.Min did not return the expected value.");
}

#[test]
fn vector4_max_test() {
    let a = Vector4::new(-1.0f32, 4.0, -3.0, 1000.0);
    let b = Vector4::new(2.0f32, 1.0, -1.0, 0.0);

    let expected = Vector4::new(2.0f32, 4.0, -1.0, 1000.0);
    let actual = a.max(b);
    assert_eq!(expected, actual, "Vector4f.Max did not return the expected value.");
}

#[test]
fn vector4_min_max_code_coverage_test() {
    let min = Vector4::<f32>::zero();
    let max = Vector4::<f32>::one();
    
    // Min.
    let mut actual = min.min(max);
    assert_eq!(actual, min);

    actual = max.min(min);
    assert_eq!(actual, min);

    // Max.
    actual = min.max(max);
    assert_eq!(actual, max);

    actual = max.max( min);
    assert_eq!(actual, max);
}

#[test]
fn vector4_clamp_test() {
    let a = Vector4::new(0.5f32, 0.3, 0.33, 0.44);
    let min = Vector4::new(0.0f32, 0.1, 0.13, 0.14);
    let max = Vector4::new(1.0f32, 1.1, 1.13, 1.14);

    let expected = Vector4::new(0.5f32, 0.3, 0.33, 0.44);
    let actual = a.clamp(min, max);
    assert_eq!(expected, actual, "Vector4f.Clamp did not return the expected value.");

    let a = Vector4::new(2.0f32, 3.0, 4.0, 5.0);
    let expected = max;
    let actual = a.clamp(min, max);
    assert_eq!(expected, actual, "Vector4f.Clamp did not return the expected value.");

    let a = Vector4::new(-2.0f32, -3.0, -4.0, -5.0);
    let expected = min;
    let actual = a.clamp( min, max);
    assert_eq!(expected, actual, "Vector4f.Clamp did not return the expected value.");

    let a = Vector4::new(-2.0f32, 0.5, 4.0, -5.0);
    let expected = Vector4::new(min.x, a.y, max.z, min.w);
    let actual = a.clamp(min, max);
    assert_eq!(expected, actual, "Vector4f.Clamp did not return the expected value.");

    let max = Vector4::new(0.0f32, 0.1, 0.13, 0.14);
    let min = Vector4::new(1.0f32, 1.1, 1.13, 1.14);

    let a = Vector4::new(0.5f32, 0.3, 0.33, 0.44);
    let expected = min;
    let actual = a.clamp(min, max);
    assert_eq!(expected, actual, "Vector4f.Clamp did not return the expected value.");

    let a = Vector4::new(2.0f32, 3.0, 4.0, 5.0);
    let expected = min;
    let actual = a.clamp(min, max);
    assert_eq!(expected, actual, "Vector4f.Clamp did not return the expected value.");

    let a = Vector4::new(-2.0f32, -3.0, -4.0, -5.0);
    let expected = min;
    let actual = a.clamp(min, max);
    assert_eq!(expected, actual, "Vector4f.Clamp did not return the expected value.");
}

#[test]
fn vector4_lerp_test() {
    let a = Vector4::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Vector4::new(5.0f32, 6.0, 7.0, 8.0);

    let t = 0.5f32;

    let expected = Vector4::new(3.0f32, 4.0, 5.0, 6.0);

    let actual = a.lerp(b, t);
    assert_eq!(expected, actual, "Vector4f.Lerp did not return the expected value.");
}

// A test for Lerp (Vector4f, Vector4f, float)
// Lerp test with factor zero
#[test]
fn vector4_lerp_test1() {
    let a = Vector4::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Vector4::new(4.0f32, 5.0, 6.0, 7.0);

    let t = 0.0f32;
    let expected = Vector4::new(1.0f32, 2.0, 3.0, 4.0);
    let actual = a.lerp(b, t);
    assert_eq!(expected, actual, "Vector4f.Lerp did not return the expected value.");
}

#[test]
fn vector4_lerp_test2() {
    let a = Vector4::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Vector4::new(4.0f32, 5.0, 6.0, 7.0);

    let t = 1.0f32;
    let expected = Vector4::new(4.0f32, 5.0, 6.0, 7.0);
    let actual = a.lerp(b, t);
    assert_eq!(expected, actual, "Vector4f.Lerp did not return the expected value.");
}

#[test]
fn vector4_lerp_test3() {
    let a = Vector4::new(0.0f32, 0.0, 0.0, 0.0);
    let b = Vector4::new(4.0f32, 5.0, 6.0, 7.0);

    let t = 2.0f32;
    let expected = Vector4::new(8.0f32, 10.0, 12.0, 14.0);
    let actual = a.lerp(b, t);
    assert_eq!(expected, actual, "Vector4f.Lerp did not return the expected value.");
}

#[test]
fn vector4_lerp_test4() {
    let a = Vector4::new(0.0f32, 0.0, 0.0, 0.0);
    let b = Vector4::new(4.0f32, 5.0, 6.0, 7.0);

    let t = -2.0f32;
    let expected = -(b * 2.0);
    let actual = a.lerp(b, t);
    assert_eq!(expected, actual, "Vector4f.Lerp did not return the expected value.");
}

#[test]
fn vector4_lerp_test5() {
    let a = Vector4::new(4.0f32, 5.0f32, 6.0f32, 7.0f32);
    let b = Vector4::new(4.0f32, 5.0f32, 6.0f32, 7.0f32);

    let t = 0.85f32;
    let expected = a;
    let actual = a.lerp(b, t);
    assert_eq!(expected, actual, "Vector4f.Lerp did not return the expected value.");
}

// A test for Transform (Vector2f, Matrix4x4)
#[test]
fn vector4_transform_test1() {
    let v = Vector2::new(1.0f32, 2.0f32);

    let mut m = Matrix4x4::create_rotation_x(30f32.to_radians())
        * Matrix4x4::create_rotation_y(30f32.to_radians())
        * Matrix4x4::create_rotation_z(30f32.to_radians());
    m.m41 = 10.0;
    m.m42 = 20.0;
    m.m43 = 30.0;

    let expected = Vector4::new(10.316987f32, 22.183012f32, 30.3660259f32, 1.0);
    let actual = v.transform_to_vector4(m);
    assert_eq!(expected, actual, "Vector4f.Transform did not return the expected value.");
}

// A test for Transform (Vector3f, Matrix4x4)
#[test]
fn vector4_transform_test2() {
    let v = Vector3::new(1.0f32, 2.0, 3.0);

    let mut m = Matrix4x4::create_rotation_x(30f32.to_radians())
        * Matrix4x4::create_rotation_y(30f32.to_radians())
        * Matrix4x4::create_rotation_z(30f32.to_radians());
    m.m41 = 10.0;
    m.m42 = 20.0;
    m.m43 = 30.0;

    let expected = Vector4::new(12.19198728f32, 21.53349376f32, 32.61602545f32, 1.0f32);
    let actual = v.transform_to_vector4( m);
    assert_eq!(expected, actual, "MathOps.Transform did not return the expected value.");
}

// A test for Transform (Vector4f, Matrix4x4)
#[test]
fn vector4_transform_vector4_test() {
    let mut v = Vector4::new(1.0f32, 2.0f32, 3.0f32, 0.0f32);

    let mut m = Matrix4x4::create_rotation_x(30f32.to_radians())
        * Matrix4x4::create_rotation_y(30f32.to_radians())
        * Matrix4x4::create_rotation_z(30f32.to_radians());
    m.m41 = 10.0;
    m.m42 = 20.0;
    m.m43 = 30.0;

    let expected = Vector4::new(2.19198728f32, 1.53349376f32, 2.61602545f32, 0.0f32);
    let actual = v.transform(m);
    assert_eq!(expected, actual, "Vector4f.Transform did not return the expected value.");

    v = v.set_w(1.0);

    let expected = Vector4::new(12.19198728f32, 21.53349376f32, 32.61602545f32, 1.0f32);
    let actual = v.transform(m);
    assert_eq!(expected, actual, "Vector4f.Transform did not return the expected value.");
}

#[test]
fn vector4_transform_vector4_test1() {
    let v = Vector4::new(1.0f32, 2.0f32, 3.0f32, 0.0f32);
    let m = Matrix4x4::zero();
    let expected = Vector4::new(0.0f32, 0.0f32, 0.0f32, 0.0f32);

    let actual = v.transform(m);
    assert_eq!(expected, actual);
}

// A test for Transform (Vector4f, Matrix4x4)
// Transform vector4 with identity matrix
#[test]
fn vector4_transform_vector4_test2() {
    let v = Vector4::new(1.0f32, 2.0f32, 3.0f32, 0.0f32);
    let m = Matrix4x4::<f32>::identity();
    let expected = Vector4::new(1.0f32, 2.0f32, 3.0f32, 0.0f32);

    let actual = v.transform(m);
    assert_eq!(expected, actual);
}

#[test]
fn vector4_transform_vector3_test() {
    let v = Vector3::new(1.0f32, 2.0, 3.0);

    let mut m = Matrix4x4::<f32>::create_rotation_x(30f32.to_radians())
        * Matrix4x4::<f32>::create_rotation_y(30f32.to_radians())
        * Matrix4x4::<f32>::create_rotation_z(30f32.to_radians());
    m.m41 = 10.0;
    m.m42 = 20.0;
    m.m43 = 30.0;

    let expected = Vector4::<f32>::new_from_vector3(v, 1.0f32).transform(m);
    let actual = v.transform_to_vector4(m);
    assert_eq!(expected, actual, "Vector4f.Transform did not return the expected value.");
}

// A test for Transform (Vector3f, Matrix4x4)
// Transform vector3 with zero matrix
#[test]
fn vector4_transform_vector3_test1() {
    let v = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let m = Matrix4x4::<f32>::zero();
    let expected = Vector4::new(0.0f32, 0.0f32, 0.0f32, 0.0f32);

    let actual = v.transform_to_vector4(m);
    assert_eq!(expected, actual, "Vector4f.Transform did not return the expected value.");
}

// A test for Transform (Vector3f, Matrix4x4)
// Transform vector3 with identity matrix
#[test]
fn vector4_transform_vector3_test2() {
    let v = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let m = Matrix4x4::<f32>::identity();
    let expected = Vector4::new(1.0f32, 2.0f32, 3.0f32, 1.0f32);

    let actual = v.transform_to_vector4(m);
    assert_eq!(expected, actual, "Vector4f.Transform did not return the expected value.");
}

// A test for Transform (Vector2f, Matrix4x4)
// Transform Vector2f test
#[test]
fn vector4_transform_vector2_test() {
    let v = Vector2::new(1.0f32, 2.0f32);

    let mut m = Matrix4x4::<f32>::create_rotation_x(30f32.to_radians())
        * Matrix4x4::<f32>::create_rotation_y(30f32.to_radians())
        * Matrix4x4::<f32>::create_rotation_z(30f32.to_radians());
    m.m41 = 10.0;
    m.m42 = 20.0;
    m.m43 = 30.0;

    
    let expected = Vector4::new_from_vector2(v, 0f32, 1f32).transform(m);
    let actual = v.transform_to_vector4(m);
    assert_eq!(expected, actual);
}
// A test for Transform (Vector2f, Matrix4x4)
// Transform Vector2f with zero matrix
#[test]
fn vector4_transform_vector2_test1() {
    let v = Vector2::new(1.0f32, 2.0f32);
    let m = Matrix4x4::<f32>::zero();
    let expected = Vector4::new(0f32, 0f32, 0f32, 0f32);

    let actual = v.transform_to_vector4(m);
    assert_eq!(expected, actual, "Vector4f.Transform did not return the expected value.");
}
// A test for Transform (Vector2f, Matrix4x4)
// Transform vector2 with identity matrix
#[test]
fn vector4_transform_vector2_test2() {
    let v = Vector2::new(1f32, 2f32);
    let m = Matrix4x4::<f32>::identity();
    let expected = Vector4::new(1f32, 2f32, 0f32, 1f32);

    let actual = v.transform_to_vector4(m);
    assert_eq!(expected, actual, "Vector4f.Transform did not return the expected value.");
}

// A test for Transform (Vector2f, Quaternion)
#[test]
fn vector4_transform_vector2_quaternion_test() {
    let v = Vector2::new(1.0f32, 2.0f32);
    let m = Matrix4x4::create_rotation_x(30.0f32.to_radians())
        * Matrix4x4::create_rotation_y(30.0f32.to_radians())
        * Matrix4x4::create_rotation_z(30.0f32.to_radians());

    let q = Quaternion::<f32>::create_from_rotation_matrix(m);

    let expected = v.transform_to_vector4(m);
    let actual = v.transform_to_vector4_quaternion(q);
    assert!(equal(expected.x, actual.x) && equal(expected.y, actual.y) && equal(expected.z, actual.z), "Vector4f.Transform did not return the expected value.");
}

// A test for Transform (Vector3f, Quaternion)
#[test]
fn vector4_transform_vector3_quaternion() {
    let v = Vector3::new(1.0f32, 2.0, 3.0);
    let m = Matrix4x4::create_rotation_x(30.0f32.to_radians())
        * Matrix4x4::create_rotation_y(30.0f32.to_radians())
        * Matrix4x4::create_rotation_z(30.0f32.to_radians());

    let q = Quaternion::create_from_rotation_matrix(m);

    let expected = v.transform_to_vector4(m);
    let actual = v.transform_to_vector4_quaternion(q);
    assert_eq!(expected, actual, "MathOps.Transform did not return the expected value.");
}

#[test]
fn vector4_transform_vector4_quaternion_test() {
    let mut v = Vector4::new(1.0f32, 2.0, 3.0, 0.0);
    let m = Matrix4x4::create_rotation_x(30.0f32.to_radians())
        * Matrix4x4::create_rotation_y(30.0f32.to_radians())
        * Matrix4x4::create_rotation_z(30.0f32.to_radians());

    let q = Quaternion::create_from_rotation_matrix(m);

    let expected = v.transform(m);
    let actual = v.transform_quaternion(q);
    assert_eq!(expected, actual, "Vector4f.Transform did not return the expected value.");

    v = v.set_w(1f32);
    let expected = expected.set_w(1f32);
    let actual = v.transform_quaternion(q);
    assert_eq!(expected, actual, "Vector4f.Transform did not return the expected value.");
}

#[test]
fn vector4_transform_vector4_quaternion_test1() {
    let v = Vector4::new(1.0f32, 2.0, 3.0, 0f32);
    let q = Quaternion::zero();
    let expected = v;

    let actual = v.transform_quaternion(q);
    assert_eq!(expected, actual);
}



#[test]
fn vector4_transform_vector4_quaternion_test2() {
    let v = Vector4::new(1.0f32, 2.0f32, 3.0f32, 0.0f32);
    let q = Quaternion::<f32>::identity();
    let expected = Vector4::new(1.0f32, 2.0f32, 3.0f32, 0.0f32);

    let actual = v.transform_quaternion(q);
    assert_eq!(expected, actual);
}

#[test]
fn vector4_transform_vector3_quaternion_test() {
    let v = Vector3::new(1.0f32, 2.0f32, 3.0f32);

    let m = Matrix4x4::create_rotation_x(30f32.to_radians()) *
        Matrix4x4::create_rotation_y(30f32.to_radians()) *
        Matrix4x4::create_rotation_z(30f32.to_radians());
    let q = Quaternion::create_from_rotation_matrix(m);

    let expected = v.transform_to_vector4(m);
    let actual = v.transform_to_vector4_quaternion(q);
    assert_eq!(expected, actual, "Vector4f.Transform did not return the expected value.");
}

#[test]
fn vector4_transform_vector3_quaternion_test1() {
    let v = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let q = Quaternion::<f32>::zero();
    let expected = Vector4::new_from_vector3(v, 1f32);

    let actual = v.transform_to_vector4_quaternion(q);
    assert_eq!(expected, actual, "Vector4f.Transform did not return the expected value.");
}

#[test]
fn vector4_transform_vector3_quaternion_test2() {
    let v = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let q = Quaternion::<f32>::identity();
    let expected = Vector4::new(1.0f32, 2.0f32, 3.0f32, 1.0f32);

    let actual = v.transform_to_vector4_quaternion(q);
    assert_eq!(expected, actual, "Vector4f.Transform did not return the expected value.");
}

// #[test]
// fn vector4_transform_vector2_quaternion_test() {
//     let v = Vector2::new(1.0f32, 2.0f32);

//     let m = Matrix4x4::create_rotation_x(30f32.to_radians()) *
//         Matrix4x4::create_rotation_y(30f32.to_radians()) *
//         Matrix4x4::create_rotation_z(30f32.to_radians());
//     let q = Quaternion::create_from_rotation_matrix(m);

//     let expected = v.transform_to_vector4(m);
//     let actual = v.transform_to_vector4_quaternion(q);
//     assert_eq!(expected, actual, "Vector4f.Transform did not return the expected value.");
// }

#[test]
fn vector4_transform_vector2_quaternion_test1() {
    let v = Vector2::new(1.0f32, 2.0f32);
    let q = Quaternion::<f32>::zero();
    let expected = Vector4::new(1.0f32, 2.0f32, 0f32, 1.0f32);

    let actual = v.transform_to_vector4_quaternion(q);
    assert_eq!(expected, actual, "Vector4f.Transform did not return the expected value.");
}

#[test]
fn vector4_transform_vector2_quaternion_test2() {
    let v = Vector2::new(1.0f32, 2.0f32);
    let q = Quaternion::<f32>::identity();
    let expected = Vector4::new(1.0f32, 2.0f32, 0f32, 1.0f32);

    let actual = v.transform_to_vector4_quaternion(q);
    assert_eq!(expected, actual, "Vector4f.Transform did not return the expected value.");
}

#[test]
fn vector4_normalize_test() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);

    let expected = Vector4::new(
        0.1825741858350553711523232609336f32,
        0.3651483716701107423046465218672f32,
        0.5477225575051661134569697828008f32,
        0.7302967433402214846092930437344f32,
    );

    let actual = a.normalize();
    assert!(equal(expected.x, actual.x) && equal(expected.y, actual.y) && equal(expected.z, actual.z) && equal(expected.w, actual.w));
}

#[test]
fn vector4_normalize_test1() {
    let a = Vector4::new(1.0f32, 0.0f32, 0.0f32, 0.0f32);

    let expected = Vector4::new(1.0f32, 0.0f32, 0.0f32, 0.0f32);
    let actual = a.normalize();
    assert_eq!(expected, actual);
}

#[test]
fn vector4_normalize_test2() {
    let a = Vector4::new(0.0f32, 0.0f32, 0.0f32, 0.0f32);

    let actual = a.normalize();
    assert!(actual.x.is_nan() && actual.y.is_nan() && actual.z.is_nan() && actual.w.is_nan());
}

#[test]
fn vector4_unary_negation_test() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);

    let expected = Vector4::new(-1.0f32, -2.0f32, -3.0f32, -4.0f32);
    let actual = -a;

    assert_eq!(expected, actual);
}

#[test]
fn vector4_subtraction_test() {
    let a = Vector4::new(1.0f32, 6.0f32, 3.0f32, 4.0f32);
    let b = Vector4::new(5.0f32, 2.0f32, 3.0f32, 9.0f32);

    let expected = Vector4::new(-4.0f32, 4.0f32, 0.0f32, -5.0f32);
    let actual = a - b;

    assert_eq!(expected, actual);
}


#[test]
fn vector4_multiply_operator_test() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let factor = 2.0f32;
    let expected = Vector4::new(2.0f32, 4.0f32, 6.0f32, 8.0f32);
    let actual = a * factor;
    assert_eq!(expected, actual);
}

#[test]
fn vector4_multiply_operator_test2() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let factor = 2.0f32;
    let expected = Vector4::new(2.0f32, 4.0f32, 6.0f32, 8.0f32);
    let actual = a * factor;
    assert_eq!(expected, actual);
}

#[test]
fn vector4_multiply_operator_test3() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let b = Vector4::new(5.0f32, 6.0f32, 7.0f32, 8.0f32);

    let expected = Vector4::new(5.0f32, 12.0f32, 21.0f32, 32.0f32);

    let actual = a * b;

    assert_eq!(expected, actual);
}

#[test]
fn vector4_division_test() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);

    let div = 2.0f32;

    let expected = Vector4::new(0.5f32, 1.0f32, 1.5f32, 2.0f32);

    let actual = a / div;

    assert_eq!(expected, actual);
}

#[test]
fn vector4_division_test1() {
    let a = Vector4::new(1.0f32, 6.0f32, 7.0f32, 4.0f32);
    let b = Vector4::new(5.0f32, 2.0f32, 3.0f32, 8.0f32);

    let expected = Vector4::new(1.0f32 / 5.0f32, 6.0f32 / 2.0f32, 7.0f32 / 3.0f32, 4.0f32 / 8.0f32);

    let actual = a / b;

    assert_eq!(expected, actual);
}




#[test]
fn vector4_division_test2() {
    let a = Vector4::new(-2.0f32, 3.0f32, f32::MAX, f32::NAN);

    let div = 0.0f32;

    let actual = a / div;

    assert!(actual.x.is_infinite() && actual.x < 0.0f32);
    assert!(actual.y.is_infinite() && actual.y > 0.0f32);
    assert!(actual.z.is_infinite() && actual.z > 0.0f32);
    assert!(actual.w.is_nan());
}

#[test]
fn vector4_division_test3() {
    let a = Vector4::new(0.047f32, -3.0f32, f32::NEG_INFINITY, f32::MIN);
    let b = Vector4::default();

    let actual = a / b;

    assert!(actual.x.is_infinite() && actual.x > 0.0f32);
    assert!(actual.y.is_infinite() && actual.y < 0.0f32);
    assert!(actual.z.is_infinite() && actual.z < 0.0f32);
    assert!(actual.w.is_infinite() && actual.w < 0.0f32);
}

#[test]
fn vector4_addition_test() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let b = Vector4::new(5.0f32, 6.0f32, 7.0f32, 8.0f32);

    let expected = Vector4::new(6.0f32, 8.0f32, 10.0f32, 12.0f32);

    let actual = a + b;

    assert_eq!(expected, actual);
}

#[test]
fn operator_add_test() {
    let v1 = Vector4::new(2.5f32, 2.0f32, 3.0f32, 3.3f32);
    let v2 = Vector4::new(5.5f32, 4.5f32, 6.5f32, 7.5f32);

    let v3 = v1 + v2;
    let v5 = Vector4::new(-1.0f32, 0.0f32, 0.0f32, f32::NAN);
    let v4 = v1 + v5;
    assert_eq!(8.0f32, v3.x);
    assert_eq!(6.5f32, v3.y);
    assert_eq!(9.5f32, v3.z);
    assert_eq!(10.8f32, v3.w);
    assert_eq!(1.5f32, v4.x);
    assert_eq!(2.0f32, v4.y);
    assert_eq!(3.0f32, v4.z);
    assert!(v4.w.is_nan());
}


#[test]
fn vector4_constructor_test() {
    let x = 1.0f32;
    let y = 2.0f32;
    let z = 3.0f32;
    let w = 4.0f32;

    let target = Vector4::new(x, y, z, w);

    assert!(equal(target.x, x) && equal(target.y, y) && equal(target.z, z) && equal(target.w, w));
}

#[test]
fn vector4_constructor_test1() {
    let a = Vector2::new(1.0f32, 2.0f32);
    let z = 3.0f32;
    let w = 4.0f32;

    let target = Vector4::new(a.x, a.y, z, w);
    assert!(equal(target.x, a.x) && equal(target.y, a.y) && equal(target.z, z) && equal(target.w, w));
}

#[test]
fn vector4_constructor_test2() {
    let a = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let w = 4.0f32;

    let target = Vector4::new(a.x, a.y, a.z, w);

    assert!(equal(target.x, a.x) && equal(target.y, a.y) && equal(target.z, a.z) && equal(target.w, w));
}

#[test]
fn vector4_constructor_test4() {
    let a = Vector4::<f32>::default();

    assert_eq!(a.x, 0.0f32);
    assert_eq!(a.y, 0.0f32);
    assert_eq!(a.z, 0.0f32);
    assert_eq!(a.w, 0.0f32);
}

#[test]
fn vector4_constructor_test5() {
    let target = Vector4::new(f32::NAN, f32::MAX, f32::INFINITY, f32::EPSILON);

    assert!(target.x.is_nan());
    assert_eq!(target.y, f32::MAX);
    assert!(target.z.is_infinite() && target.z > 0.0f32);
    assert_eq!(target.w, f32::EPSILON);
}



#[test]
fn vector4_add_test() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let b = Vector4::new(5.0f32, 6.0f32, 7.0f32, 8.0f32);

    let expected = Vector4::new(6.0f32, 8.0f32, 10.0f32, 12.0f32);

    let actual = a + b;
    assert_eq!(expected, actual);
}

#[test]
fn vector4_divide_test() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let div = 2.0f32;
    let expected = Vector4::new(0.5f32, 1.0f32, 1.5f32, 2.0f32);
    assert_eq!(expected, a / div);
}

#[test]
fn vector4_divide_test1() {
    let a = Vector4::new(1.0f32, 6.0f32, 7.0f32, 4.0f32);
    let b = Vector4::new(5.0f32, 2.0f32, 3.0f32, 8.0f32);

    let expected = Vector4::new(1.0f32 / 5.0f32, 6.0f32 / 2.0f32, 7.0f32 / 3.0f32, 4.0f32 / 8.0f32);

    let actual = a / b;
    assert_eq!(expected, actual);
}

#[test]
fn vector4_equals_test() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let b = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);

    // case 1: compare between same values
    let expected = true;
    let actual = a == b;
    assert_eq!(expected, actual);

    // case 2: compare between different values
    let b = Vector4::new(10.0f32, 2.0f32, 3.0f32, 4.0f32);
    let expected = false;
    let actual = a == b;
    assert_eq!(expected, actual);
}


#[test]
fn vector4_multiply_test() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let factor = 2.0f32;
    let expected = Vector4::new(2.0f32, 4.0f32, 6.0f32, 8.0f32);
    assert_eq!(expected, a * factor);
}

#[test]
fn vector4_multiply_test2() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let factor = 2.0f32;
    let expected = Vector4::new(2.0f32, 4.0f32, 6.0f32, 8.0f32);
    assert_eq!(expected, a * factor);
}

#[test]
fn vector4_multiply_test3() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let b = Vector4::new(5.0f32, 6.0f32, 7.0f32, 8.0f32);

    let expected = Vector4::new(5.0f32, 12.0f32, 21.0f32, 32.0f32);

    let actual = a * b;
    assert_eq!(expected, actual);
}

#[test]
fn vector4_negate_test() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);

    let expected = Vector4::new(-1.0f32, -2.0f32, -3.0f32, -4.0f32);

    let actual = -a;
    assert_eq!(expected, actual);
}

#[test]
fn vector4_inequality_test() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let b = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);

    // case 1: compare between same values
    let expected = false;
    let actual = a != b;
    assert_eq!(expected, actual);

    // case 2: compare between different values
    let b = Vector4::new(10.0f32, 2.0f32, 3.0f32, 4.0f32);
    let expected = true;
    let actual = a != b;
    assert_eq!(expected, actual);
}




#[test]
fn vector4_equality_test() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let b = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);

    // case 1: compare between same values
    let expected = true;
    let actual = a == b;
    assert_eq!(expected, actual);

    // case 2: compare between different values
    let b = Vector4::new(10.0f32, 2.0f32, 3.0f32, 4.0f32);
    let expected = false;
    let actual = a == b;
    assert_eq!(expected, actual);
}

#[test]
fn vector4_subtract_test() {
    let a = Vector4::new(1.0f32, 6.0f32, 3.0f32, 4.0f32);
    let b = Vector4::new(5.0f32, 2.0f32, 3.0f32, 9.0f32);

    let expected = Vector4::new(-4.0f32, 4.0f32, 0.0f32, -5.0f32);

    let actual = a - b;

    assert_eq!(expected, actual);
}

#[test]
fn vector4_unit_w_test() {
    let val = Vector4::new(0.0f32, 0.0f32, 0.0f32, 1.0f32);
    assert_eq!(val, Vector4::unit_w());
}

#[test]
fn vector4_unit_x_test() {
    let val = Vector4::new(1.0f32, 0.0f32, 0.0f32, 0.0f32);
    assert_eq!(val, Vector4::unit_x());
}

#[test]
fn vector4_unit_y_test() {
    let val = Vector4::new(0.0f32, 1.0f32, 0.0f32, 0.0f32);
    assert_eq!(val, Vector4::unit_y());
}

#[test]
fn vector4_unit_z_test() {
    let val = Vector4::new(0.0f32, 0.0f32, 1.0f32, 0.0f32);
    assert_eq!(val, Vector4::unit_z());
}



#[test]
fn vector4_one_test() {
    let val = Vector4::new(1.0f32, 1.0f32, 1.0f32, 1.0f32);
    assert_eq!(val, Vector4::one());
}

#[test]
fn vector4_zero_test() {
    let val = Vector4::new(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    assert_eq!(val, Vector4::zero());
}

#[test]
fn vector4_equals_test1() {
    let a = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let b = Vector4::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);

    // case 1: compare between same values
    assert!(a == b);

    // case 2: compare between different values
    let b = Vector4::new(10.0f32, 2.0f32, 3.0f32, 4.0f32);
    assert!(a != b);
}

#[test]
fn vector4_constructor_test6() {
    let value = 1.0f32;
    let target = Vector4::new_from_num(value);

    let expected = Vector4::new(value, value, value, value);
    assert_eq!(expected, target);

    let value = 2.0f32;
    let target = Vector4::new_from_num(value);
    let expected = Vector4::new(value, value, value, value);
    assert_eq!(expected, target);
}

#[test]
fn vector4_equals_nan_test() {
    let a = Vector4::new(f32::NAN, 0.0, 0.0, 0.0);
    let b = Vector4::new(0.0, f32::NAN, 0.0, 0.0);
    let c = Vector4::new(0.0, 0.0, f32::NAN, 0.0);
    let d = Vector4::new(0.0, 0.0, 0.0, f32::NAN);

    assert!(a != Vector4::zero());
    assert!(b != Vector4::zero());
    assert!(c != Vector4::zero());
    assert!(d != Vector4::zero());

    assert!(a != a);
    assert!(b != b);
    assert!(c != c);
    assert!(d != d);
}


#[test]
fn vector4_abs_test() {
    let v1 = Vector4::new(-2.5f32, 2.0f32, 3.0f32, 3.3f32);
    let v3 = Vector4::new(f32::INFINITY, 0.0f32, f32::NEG_INFINITY, f32::NAN).abs();
    let v = v1.abs();
    assert_eq!(2.5f32, v.x);
    assert_eq!(2.0f32, v.y);
    assert_eq!(3.0f32, v.z);
    assert_eq!(3.3f32, v.w);
    assert!(v3.x.is_infinite());
    assert_eq!(0.0f32, v3.y);
    assert!(v3.z.is_infinite());
    assert!(v3.w.is_nan());
}

#[test]
fn vector4_sqrt_test() {
    let v1 = Vector4::new(-2.5f32, 2.0f32, 3.0f32, 3.3f32);
    let v2 = Vector4::new(5.5f32, 4.5f32, 6.5f32, 7.5f32);
    assert_eq!(2, (v2.sqrt().x as i32));
    assert_eq!(2, (v2.sqrt().y as i32));
    assert_eq!(2, (v2.sqrt().z as i32));
    assert_eq!(2, (v2.sqrt().w as i32));
    assert!(v1.sqrt().x.is_nan());
}
