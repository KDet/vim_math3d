use num_traits::Float;
use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher, ops::{Add, Div, Mul}};
use vim_math3d::{self, Vector2, Vector3, Matrix4x4, Transformable3D, Quaternion};

fn equal<T: Float>(a: T, b: T) -> bool { (a - b).abs() < T::from(1e-5).unwrap() }
fn hash_code<T: Float>(v: &Vector3<T>) -> u64 {
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn vector3_marshal_size_test() {
    assert_eq!(12, std::mem::size_of::<Vector3<f32>>());
}

#[test]
fn vector3_get_hash_code_test() {
    let v1 = Vector3::new(2.0f32, 3.0, 3.3);
    let v2 = Vector3::new(2.0f32, 3.0, 3.3);
    let v3 = Vector3::new(2.0f32, 3.0, 3.3);
    let v5 = Vector3::new(3.0f32, 2.0, 3.3);

    assert_eq!(hash_code(&v1), hash_code(&v1));
    assert_eq!(hash_code(&v1), hash_code(&v2));
    assert_ne!(hash_code(&v1), hash_code(&v5));
    assert_eq!(hash_code(&v1), hash_code(&v3));

    let v4 = Vector3::new(0.0f32, 0.0, 0.0);
    let v6 = Vector3::new(1.0f32, 0.0, 0.0);
    let v7 = Vector3::new(0.0f32, 1.0, 0.0);
    let v8 = Vector3::new(1.0f32, 1.0, 1.0);
    let v9 = Vector3::new(1.0f32, 1.0, 0.0);
    assert_ne!(hash_code(&v4), hash_code(&v6));
    assert_ne!(hash_code(&v4), hash_code(&v7));
    assert_ne!(hash_code(&v4), hash_code(&v8));
    assert_ne!(hash_code(&v7), hash_code(&v6));
    assert_ne!(hash_code(&v8), hash_code(&v6));
    assert_ne!(hash_code(&v8), hash_code(&v9));
    assert_ne!(hash_code(&v7), hash_code(&v9));

 
}

#[test]
fn vector3_cross_test() {
    let a = Vector3::new(1.0f32, 0.0, 0.0);
    let b = Vector3::new(0.0f32, 1.0, 0.0);

    let expected = Vector3::new(0.0f32, 0.0, 1.0);

    let actual = a.cross(b);
    assert_eq!(expected, actual);
}

#[test]
fn vector3_cross_test1() {
    let a = Vector3::new(0.0f32, 1.0, 0.0);
    let b = Vector3::new(0.0f32, 1.0, 0.0);

    let expected = Vector3::new(0.0f32, 0.0, 0.0);
    let actual = a.cross(b);
    assert_eq!(expected, actual);
}

#[test]
fn vector3_distance_test() {
    let a = Vector3::new(1.0f32, 2.0, 3.0);
    let b = Vector3::new(4.0f32, 5.0, 6.0);

    let expected = (27f32).sqrt();

    let actual = a.distance(b);
    assert_eq!(expected, actual);
}

#[test]
fn vector3_distance_test1() {
    let a = Vector3::new(1.051f32, 2.05, 3.478);
    let b = Vector3::new(1.051f32, 2.05, 3.478);

    let actual = a.distance(b);
    assert_eq!(0.0, actual);
}





#[test]
fn vector3_distance_squared_test() {
    let a = Vector3::new(1.0f32, 2.0, 3.0);
    let b = Vector3::new(4.0f32, 5.0, 6.0);

    let expected = 27.0f32;
    let actual = a.distance_squared(b);
    assert_eq!(expected, actual);
}

#[test]
fn vector3_dot_test() {
    let a = Vector3::new(1.0f32, 2.0, 3.0);
    let b = Vector3::new(4.0f32, 5.0, 6.0);

    let expected = 32.0f32;
    let actual = a.dot(b);
    assert_eq!(expected, actual);
}

#[test]
fn vector3_dot_test1() {
    let a = Vector3::new(1.55f32, 1.55, 1.0);
    let b = Vector3::new(2.5f32, 3.0, 1.5);
    let c = a.cross(b);

    let expected = 0.0f32;
    let actual1 = a.dot(c);
    let actual2 = b.dot(c);
    assert!(equal(expected, actual1));
    assert!(equal(expected, actual2));
}

#[test]
fn vector3_length_test() {
    let a = Vector2::new(1.0f32, 2.0);
    let z = 3.0f32;
    let target = Vector3::new(a.x, a.y, z);

    let expected = 14.0f32.sqrt();
    let actual = target.length();
    assert_eq!(expected, actual);
}

#[test]
fn vector3_length_test1() {
    let target = Vector3::new(0.0f32, 0.0, 0.0);

    let expected = 0.0f32;
    let actual = target.length();
    assert_eq!(expected, actual);
}

#[test]
fn vector3_length_squared_test() {
    let a = Vector2::new(1.0f32, 2.0);
    let z = 3.0f32;
    let target = Vector3::new(a.x, a.y, z);

    let expected = 14.0f32;
    let actual = target.length_squared();
    assert_eq!(expected, actual);
}

#[test]
fn vector3_min_test() {
    let a = Vector3::new(-1.0f32, 4.0, -3.0);
    let b = Vector3::new(2.0f32, 1.0, -1.0);

    let expected = Vector3::new(-1.0f32, 1.0, -3.0);
    let actual = a.min(b);
    assert_eq!(expected, actual);
}

#[test]
fn vector3_max_test() {
    let a = Vector3::new(-1.0f32, 4.0, -3.0);
    let b = Vector3::new(2.0f32, 1.0, -1.0);

    let expected = Vector3::new(2.0f32, 4.0, -1.0);
    let actual = a.max(b);
    assert_eq!(expected, actual);
}

#[test]
fn test_vector3_min_max_code_coverage() {
    let min = Vector3::<f32>::zero();
    let max = Vector3::<f32>::one();
    
    // Min.
    let actual = min.min(max);
    assert_eq!(actual, min);

    let actual = max.min(min);
    assert_eq!(actual, min);

    // Max.
    let actual = min.max(max);
    assert_eq!(actual, max);

    let actual = max.max(min);
    assert_eq!(actual, max);
}

#[test]
fn vector3_lerp_test() {
    let a = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let b = Vector3::new(4.0f32, 5.0f32, 6.0f32);
    let t = 0.5;

    let expected = Vector3::new(2.5f32, 3.5f32, 4.5f32);
    let actual = a.lerp(b, t);
    assert_eq!(actual, expected);
}

#[test]
fn vector3_lerp_test_1() {
    let a = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let b = Vector3::new(4.0f32, 5.0f32, 6.0f32);
    let t = 0.0f32;

    let expected = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let actual = a.lerp(b, t);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z));
}

#[test]
fn vector3_lerp_test_2() {
    let a = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let b = Vector3::new(4.0f32, 5.0f32, 6.0f32);
    let t = 1.0f32;

    let expected = Vector3::new(4.0f32, 5.0f32, 6.0f32);
    let actual = a.lerp(b, t);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z));
}

#[test]
fn vector3_lerp_test_3() {
    let a = Vector3::new(0.0f32, 0.0, 0.0);
    let b = Vector3::new(4.0f32, 5.0, 6.0);
    let t = 2.0f32;

    let expected = Vector3::new(8.0f32, 10.0, 12.0);
    let actual = a.lerp(b, t);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z));
}

#[test]
fn vector3_lerp_test_4() {
    let a = Vector3::new(0.0f32, 0.0f32, 0.0f32);
    let b = Vector3::new(4.0f32, 5.0f32, 6.0f32);
    let t = -2.0f32;

    let expected = Vector3::new(-8.0f32, -10.0f32, -12.0f32);
    let actual = a.lerp(b, t);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z));
}

#[test]
fn vector3_lerp_test_5() {
    let a = Vector3::new(1.68f32, 2.34, 5.43);
    let b = a;
    let t = 0.18f32;

    let expected = Vector3::new(1.68f32, 2.34, 5.43);
    let actual = a.lerp(b, t);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z));
}

 
#[test]
fn vector3_reflect_test() {
    let a = Vector3::new(1.0f32, 1.0f32, 1.0f32).normalize();

    // Reflect on XZ plane.
    let n = Vector3::new(0.0f32, 1.0f32, 0.0f32);
    let expected = Vector3::new(a.x, -a.y, a.z);
    let actual = a.reflect(n);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Reflect did not return the expected value.");

    // Reflect on XY plane.
    let n = Vector3::new(0.0f32, 0.0f32, 1.0f32);
    let expected = Vector3::new(a.x, a.y, -a.z);
    let actual = a.reflect(n);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Reflect did not return the expected value.");

    // Reflect on YZ plane.
    let n = Vector3::new(1.0f32, 0.0f32, 0.0f32);
    let expected = Vector3::new(-a.x, a.y, a.z);
    let actual = a.reflect(n);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Reflect did not return the expected value.");
}

#[test]
fn vector3_reflect_test1() {
    let mut n = Vector3::new(0.45f32, 1.28f32, 0.86f32);
    n = n.normalize();
    let a = n;

    let expected = -n;
    let actual = a.reflect(n);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Reflect did not return the expected value.");
}

#[test]
fn vector3_reflect_test2() {
    let mut n = Vector3::new(0.45f32, 1.28f32, 0.86f32);
    n = n.normalize();
    let a = -n;

    let expected = n;
    let actual = a.reflect(n);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Reflect did not return the expected value.");
}

#[test]
fn vector3_reflect_test3() {
    let n = Vector3::new(0.45f32, 1.28f32, 0.86f32);
    let temp = Vector3::new(1.28f32, 0.45f32, 0.01f32);
    // find a perpendicular vector of n
    let a = temp.cross(n);

    let expected = a;
    let actual = a.reflect(n);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Reflect did not return the expected value.");
}

#[test]
fn vector3_transform_test() {
    let v = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let mut m =
        Matrix4x4::create_rotation_x(30.0f32.to_radians()) *
        Matrix4x4::create_rotation_y(30.0f32.to_radians()) *
        Matrix4x4::create_rotation_z(30.0f32.to_radians());
    m.m41 = 10.0f32;
    m.m42 = 20.0f32;
    m.m43 = 30.0f32;

    let expected = Vector3::new(12.191987f32, 21.533493f32, 32.616024f32);

    let actual = v.transform(m);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Transform did not return the expected value.");
}

#[test]
fn vector3_clamp_test() {
    let a = Vector3::new(0.5f32, 0.3f32, 0.33f32);
    let min = Vector3::new(0.0f32, 0.1f32, 0.13f32);
    let max = Vector3::new(1.0f32, 1.1f32, 1.13f32);

    let expected = Vector3::new(0.5f32, 0.3f32, 0.33f32);
    let actual = a.clamp( min, max);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Clamp did not return the expected value.");

    let a = Vector3::new(2.0f32, 3.0, 4.0);
    let expected = max;
    let actual = a.clamp(min, max);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Clamp did not return the expected value.");

    let a = Vector3::new(-2.0f32, -3.0, -4.0);
    let expected = min;
    let actual = a.clamp(min, max);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Clamp did not return the expected value.");

    let a = Vector3::new(-2.0f32, 0.5, 4.0);
    let expected = Vector3::new(min.x, a.y, max.z);
    let actual = a.clamp(min, max);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Clamp did not return the expected value.");

    let max = Vector3::new(0.0f32, 0.1, 0.13);
    let min = Vector3::new(1.0f32, 1.1, 1.13);

    let a = Vector3::new(0.5f32, 0.3, 0.33);
    let expected = min;
    let actual = a.clamp(min, max);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Clamp did not return the expected value.");

    let a = Vector3::new(2.0f32, 3.0, 4.0);
    let expected = min;
    let actual = a.clamp(min, max);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Clamp did not return the expected value.");

    let a = Vector3::new(-2.0f32, -3.0, -4.0);
    let expected = min;
    let actual = a.clamp(min, max);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Clamp did not return the expected value.");
}

 
#[test]
fn vector3_transform_normal_test() {
    let v = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let mut m = Matrix4x4::create_rotation_x(30.0f32.to_radians())
        * Matrix4x4::create_rotation_y(30.0f32.to_radians())
        * Matrix4x4::create_rotation_z(30.0f32.to_radians());
    m.m41 = 10.0f32;
    m.m42 = 20.0f32;
    m.m43 = 30.0f32;

    let expected = Vector3::new(2.19198728f32, 1.53349364f32, 2.61602545f32);
    let actual = v.transform_normal(m);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.TransformNormal did not return the expected value.");
}

#[test]
fn vector3_transform_by_quaternion_test() {
    let v = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let m = Matrix4x4::create_rotation_x(30.0f32.to_radians())
        * Matrix4x4::create_rotation_y(30.0f32.to_radians())
        * Matrix4x4::create_rotation_z(30.0f32.to_radians());
    let q = Quaternion::create_from_rotation_matrix(m);

    let expected = v.transform(m);
    let actual = v.transform_quaternion(q);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Transform did not return the expected value.");
}

#[test]
fn vector3_transform_by_quaternion_test1() {
    let v = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let q = Quaternion::<f32>::zero();
    let expected = v;

    let actual = v.transform_quaternion(q);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Transform did not return the expected value.");
}

#[test]
fn vector3_transform_by_quaternion_test2() {
    let v = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let q = Quaternion::<f32>::identity();
    let expected = v;

    let actual = v.transform_quaternion(q);
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Transform did not return the expected value.");
}

#[test]
fn vector3_normalize_test() {
    let a = Vector3::new(1.0f32, 2.0f32, 3.0f32);

    let expected = Vector3::new(
        0.26726124191242438468455348087975f32,
        0.53452248382484876936910696175951f32,
        0.80178372573727315405366044263926f32);

    let actual = a.normalize();
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Normalize did not return the expected value.");
}

#[test]
fn vector3_normalize_test1() {
    let a = Vector3::new(1.0f32, 0.0f32, 0.0f32);

    let expected = Vector3::new(1.0f32, 0.0f32, 0.0f32);
    let actual = a.normalize();
    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.Normalize did not return the expected value.");
}

#[test]
fn vector3_normalize_test2() {
    let a = Vector3::new(0.0f32, 0.0f32, 0.0f32);

   // let expected = Vector3::new(0.0f32, 0.0f32, 0.0f32);
    let actual = a.normalize();
    assert!(actual.x.is_nan() && actual.y.is_nan() && actual.z.is_nan(), "Vector3f.Normalize did not return the expected value.");
}

#[test]
fn vector3_unary_negation_test() {
    let a = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let expected = Vector3::new(-1.0f32, -2.0f32, -3.0f32);
    let actual = -a;

    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.operator - did not return the expected value.");
}

#[test]
fn vector3_unary_negation_test1() {
    let a = -Vector3::new(f32::NAN, f32::INFINITY, f32::NEG_INFINITY);
    let b = -Vector3::new(0.0f32, 0.0f32, 0.0f32);
    assert!(a.x.is_nan());
    assert!(a.y.is_infinite() && a.y < 0f32);
    assert!(a.z.is_infinite() && a.z > 0f32);
    assert_eq!(0.0, b.x);
    assert_eq!(0.0, b.y);
    assert_eq!(0.0, b.z);
}

#[test]
fn vector3_subtraction_test() {
    let a = Vector3::new(4.0f32, 2.0f32, 3.0f32);
    let b = Vector3::new(1.0f32, 5.0f32, 7.0f32);
    let expected = Vector3::new(3.0f32, -3.0f32, -4.0f32);
    let actual = a - b;

    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.operator - did not return the expected value.");
}

#[test]
fn vector3_multiply_operator_test() {
    let a = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let factor = 2.0;
    let expected = Vector3::new(2.0f32, 4.0f32, 6.0f32);
    let actual = a * factor;

    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.operator * did not return the expected value.");
}

#[test]
fn vector3_multiply_operator_test2() {
    let a = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let factor = 2.0;
    let expected = Vector3::new(2.0f32, 4.0f32, 6.0f32);
    let actual = a * factor;

    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.operator * did not return the expected value.");
}

#[test]
fn vector3_multiply_operator_test3() {
    let a = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let b = Vector3::new(4.0f32, 5.0f32, 6.0f32);
    let expected = Vector3::new(4.0f32, 10.0f32, 18.0f32);
    let actual = a * b;

    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.operator * did not return the expected value.");
}

#[test]
fn vector3_division_test() {
    let a = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let div = 2.0f32;
    let expected = Vector3::new(0.5f32, 1.0f32, 1.5f32);
    let actual = a / div;

    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.operator / did not return the expected value.");
}

#[test]
fn vector3_division_test1() {
    let a = Vector3::new(4.0f32, 2.0f32, 3.0f32);
    let b = Vector3::new(1.0f32, 5.0f32, 6.0f32);
    let expected = Vector3::new(4.0f32, 0.4f32, 0.5f32);
    let actual = a / b;

    assert!(equal(actual.x, expected.x) && equal(actual.y, expected.y) && equal(actual.z, expected.z), "Vector3f.operator / did not return the expected value.");
}

#[test]
fn vector3_division_test2() {
    let a = Vector3::new(-2.0f32, 3.0f32, f32::MAX);
    let div = 0.0f32;
    let actual = a / div;

    assert!(actual.x.is_infinite() && actual.x.is_sign_negative(), "Vector3f.operator / did not return the expected value.");
    assert!(actual.y.is_infinite() && actual.y.is_sign_positive(), "Vector3f.operator / did not return the expected value.");
    assert!(actual.z.is_infinite() && actual.z.is_sign_positive(), "Vector3f.operator / did not return the expected value.");
}

// A test for operator / (Vector3f, Vector3f)
// Divide by zero
#[test]
fn vector3_division_test3() {
    let a = Vector3::new(0.047f32, -3.0f32, f32::NEG_INFINITY);
    let b = Vector3::<f32>::default();

    let actual = a / b;

    assert!(actual.x.is_infinite() && actual.x.is_sign_positive(), "Vector3f.operator / did not return the expected value.");
    assert!(actual.y.is_infinite() && actual.y.is_sign_negative(), "Vector3f.operator / did not return the expected value.");
    assert!(actual.z.is_infinite() && actual.z.is_sign_negative(), "Vector3f.operator / did not return the expected value.");
}

// A test for operator + (Vector3f, Vector3f)
#[test]
fn vector3_addition_test() {
    let a = Vector3::new(1.0f32, 2.0f32, 3.0f32);
    let b = Vector3::new(4.0f32, 5.0f32, 6.0f32);
    let expected = Vector3::new(5.0f32, 7.0f32, 9.0f32);
    let actual = a + b;

    assert_eq!(expected, actual, "Vector3f.operator + did not return the expected value.");
}

// A test for Vector3f (float, float, float)
#[test]
fn vector3_constructor_test() {
    let x = 1.0f32;
    let y = 2.0f32;
    let z = 3.0f32;

    let target = Vector3::new(x, y, z);

    assert_eq!(target.x, x);
    assert_eq!(target.y, y);
    assert_eq!(target.z, z);
}

// A test for Vector3f (Vector2f, float)
#[test]
fn vector3_constructor_test1() {
    let a = Vector2::new(1.0f32, 2.0f32);
    let z = 3.0f32;
    let target = Vector3::new_from_vector2(a, z);

    assert_eq!(target.x, a.x);
    assert_eq!(target.y, a.y);
    assert_eq!(target.z, z);
}

// A test for Vector3f ()
// Constructor with no parameter
#[test]
fn vector3_constructor_test3() {
    let a = Vector3::<f32>::default();

    assert_eq!(a.x, 0.0f32);
    assert_eq!(a.y, 0.0f32);
    assert_eq!(a.z, 0.0f32);
}

// A test for Vector2f (float, float)
// Constructor with special floating values
#[test]
fn vector3_constructor_test4() {
    let target = Vector3::new(f32::NAN, f32::MAX, f32::INFINITY);

    assert!(target.x.is_nan(), "Vector3f.constructor (Vector3f) did not return the expected value.");
    assert_eq!(f32::MAX, target.y, "Vector3f.constructor (Vector3f) did not return the expected value.");
    assert!(target.z.is_infinite() && target.z.is_sign_positive(), "Vector3f.constructor (Vector3f) did not return the expected value.");
}






// A test for Add (Vector3f, Vector3f)
#[test]
fn vector3_add_test() {
    let a = Vector3::new(1.0f32, 2.0, 3.0);
    let b = Vector3::new(5.0f32, 6.0, 7.0);
    let expected = Vector3::new(6.0f32, 8.0, 10.0);
    let actual = a.add(b);
    assert_eq!(expected, actual);
}

// A test for Divide (Vector3f, float)
#[test]
fn vector3_divide_test() {
    let a = Vector3::new(1.0f32, 2.0, 3.0);
    let div = 2.0f32;
    let expected = Vector3::new(0.5f32, 1.0, 1.5);
    assert_eq!(expected, a / div);
}

// A test for Divide (Vector3f, Vector3f)
#[test]
fn vector3_divide_test1() {
    let a = Vector3::new(1.0f32, 6.0f32, 7.0f32);
    let b = Vector3::new(5.0f32, 2.0f32, 3.0f32);
    let expected = Vector3::new(1.0f32 / 5.0f32, 6.0f32 / 2.0f32, 7.0f32 / 3.0f32);
    let actual = a.div(b);
    assert_eq!(expected, actual);
}

// A test for PartialEq
#[test]
fn vector3_eq_test() {
    let a = Vector3::new(1.0f32, 2.0, 3.0);
    let b = Vector3::new(1.0f32, 2.0, 3.0);

    // case 1: compare between same values
    let expected = true;
    let actual = a.eq(&b);
    assert_eq!(expected, actual);

    // case 2: compare between different values
    let b = b.set_x(10.0f32);
    let expected = false;
    let actual = a.eq(&b);
    assert_eq!(expected, actual);
}

// A test for Multiply (Vector3f, float)
#[test]
fn vector3_multiply_test() {
    let a = Vector3::new(1.0f32, 2.0, 3.0);
    let factor = 2.0f32;
    let expected = Vector3::new(2.0f32, 4.0, 6.0);
    assert_eq!(expected, a * factor);
}

// A test for Multiply (float, Vector3f)
#[test]
fn vector3_multiply_test2() {
    let a = Vector3::new(1.0f32, 2.0, 3.0);
    let factor = 2.0f32;
    let expected = Vector3::new(2.0f32, 4.0, 6.0);
    assert_eq!(expected, a * factor);
}

#[test]
fn vector3_multiply_test3() {
    let a = Vector3::new(1.0f32, 2.0, 3.0);
    let b = Vector3::new(5.0f32, 6.0, 7.0);
    let expected = Vector3::new(5.0f32, 12.0, 21.0);
    let actual = a.mul(b);
    assert_eq!(expected, actual);
}

#[test]
fn vector3_negate_test() {
    let a = Vector3::new(1.0f32, 2.0, 3.0);
    let expected = Vector3::new(-1.0f32, -2.0, -3.0);
    let actual = -a;
    assert_eq!(expected, actual);
}

#[test]
fn vector3_inequality_test() {
    let a = Vector3::new(1.0f32, 2.0, 3.0);
    let b = Vector3::new(1.0f32, 2.0, 3.0);

    // case 1: compare between same values
    let expected = false;
    let actual = a != b;
    assert_eq!(expected, actual);

    // case 2: compare between different values
    let b = b.set_x(10.0f32);
    let expected = true;
    let actual = a != b;
    assert_eq!(expected, actual);
}

#[test]
fn vector3_equality_test() {
    let a = Vector3::new(1.0f32, 2.0, 3.0);
    let b = Vector3::new(1.0f32, 2.0, 3.0);

    // case 1: compare between same values
    let expected = true;
    let actual = a == b;
    assert_eq!(expected, actual);

    // case 2: compare between different values
    let b = b.set_x(10.0f32);
    let expected = false;
    let actual = a == b;
    assert_eq!(expected, actual);
}

#[test]
fn vector3_subtract_test() {
    let a = Vector3::new(1.0f32, 6.0, 3.0);
    let b = Vector3::new(5.0f32, 2.0, 3.0);

    let expected = Vector3::new(-4.0f32, 4.0, 0.0);
    let actual = a - b;
    assert_eq!(expected, actual);
}


#[test]
fn vector3_one_test() {
    let val = Vector3::new(1.0f32, 1.0, 1.0);
    assert_eq!(val, Vector3::<f32>::one());
}

#[test]
fn vector3_unit_x_test() {
    let val = Vector3::new(1.0f32, 0.0, 0.0);
    assert_eq!(val, Vector3::<f32>::unit_x());
}

#[test]
fn vector3_unit_y_test() {
    let val = Vector3::new(0.0f32, 1.0, 0.0);
    assert_eq!(val, Vector3::<f32>::unit_y());
}

#[test]
fn vector3_unit_z_test() {
    let val = Vector3::new(0.0f32, 0.0, 1.0);
    assert_eq!(val, Vector3::<f32>::unit_z());
}

#[test]
fn vector3_zero_test() {
    let val = Vector3::new(0.0f32, 0.0, 0.0);
    assert_eq!(val, Vector3::<f32>::zero());
}

#[test]
fn vector3_equals_test() {
    let a = Vector3::new(1.0f32, 2.0, 3.0);
    let b = Vector3::new(1.0f32, 2.0, 3.0);

    // case 1: compare between same values
    let expected = true;
    let actual = a == b;
    assert_eq!(expected, actual);

    // case 2: compare between different values
    let b = b.set_x(10.0f32);
    let expected = false;
    let actual = a == b;
    assert_eq!(expected, actual);
}

#[test]
fn vector3_constructor_test5() {
    let value = 1.0_f32;
    let target = Vector3::new(value, value, value);

    let expected = Vector3::new(value, value, value);
    assert_eq!(expected, target);

    let value = 2.0_f32;
    let target = Vector3::new(value, value, value);

    let expected = Vector3::new(value, value, value);
    assert_eq!(expected, target);
}

#[test]
fn vector3_equals_nan_test() {
    let a = Vector3::new(f32::NAN, 0.0f32, 0.0);
    let b = Vector3::new(0.0f32, f32::NAN, 0.0);
    let c = Vector3::new(0.0f32, 0.0, f32::NAN);

    assert!(!(a == Vector3::<f32>::zero()));
    assert!(!(b == Vector3::<f32>::zero()));
    assert!(!(c == Vector3::<f32>::zero()));

    assert!(a != Vector3::<f32>::zero());
    assert!(b != Vector3::<f32>::zero());
    assert!(c != Vector3::<f32>::zero());

    assert!(!a.eq(&Vector3::<f32>::zero()));
    assert!(!b.eq(&Vector3::<f32>::zero()));
    assert!(!c.eq(&Vector3::<f32>::zero()));

    assert!(!a.eq(&a));
    assert!(!b.eq(&b));
    assert!(!c.eq(&c));
}

#[test]
fn vector3_abs_test() {
    let v1 = Vector3::new(-2.5f32, 2.0, 0.5);
    let v3 = Vector3::new(0.0f32, f32::NEG_INFINITY, f32::NAN).abs();
    let v = v1.abs();
    assert_eq!(2.5f32, v.x);
    assert_eq!(2.0f32, v.y);
    assert_eq!(0.5f32, v.z);
    assert_eq!(0.0f32, v3.x);
    assert!(v3.y.is_infinite() && v3.y.is_sign_positive());
    assert!(v3.z.is_nan());
}

#[test]
fn vector3_sqrt_test() {
    let a = Vector3::new(-2.5f32, 2.0, 0.5);
    let b = Vector3::new(5.5f32, 4.5, 16.5);
    let root_b = b.square_root();
    let root_a = a.square_root();
    
    assert_eq!(2, root_b.x as i32);
    assert_eq!(2, root_b.y as i32);
    assert_eq!(4, root_b.z as i32);
    assert!(root_a.x.is_nan());
}

