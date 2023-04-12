use num_traits::Float;
use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher, ops::{Add, Div, Mul, Neg, Sub}};
use vim_math3d::{self, Vector2, Matrix4x4, Transformable3D, Quaternion};

fn equal<T: Float>(a: T, b: T) -> bool { (a - b).abs() < T::from(1e-5).unwrap() }
fn hash_code<T: Float>(v: &Vector2<T>) -> u64 {
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn vector2_marshal_size_test() {
    assert_eq!(8, std::mem::size_of::<Vector2<f32>>());
}

#[test]
fn vector2_get_hash_code_test() {
    let v1 = Vector2::new(2.0f32, 3.0);
    let v2 = Vector2::new(2.0f32, 3.0);
    let v3 = Vector2::new(3.0f32, 2.0);
    assert_eq!(hash_code(&v1), hash_code(&v1));
    assert_eq!(hash_code(&v1), hash_code(&v2));
    assert_ne!(hash_code(&v1), hash_code(&v3));
    let v4 = Vector2::new(0.0f32, 0.0);
    let v6 = Vector2::new(1.0f32, 0.0);
    let v7 = Vector2::new(0.0f32, 1.0);
    let v8 = Vector2::new(1.0f32, 1.0);
    assert_ne!(hash_code(&v4), hash_code(&v6));
    assert_ne!(hash_code(&v4), hash_code(&v7));
    assert_ne!(hash_code(&v4), hash_code(&v8));
    assert_ne!(hash_code(&v7), hash_code(&v6));
    assert_ne!(hash_code(&v8), hash_code(&v6));
    assert_ne!(hash_code(&v8), hash_code(&v7));
}

#[test]
fn vector2_distance_test() {
    let a = Vector2::new(1.0f32, 2.0);
    let b = Vector2::new(3.0f32, 4.0);
    let expected = 2.828427f32;
    let actual = a.distance(b);
    assert_eq!(expected, actual, "Vector2::distance did not return the expected value.");
}

#[test]
fn vector2_distance_test2() {
    let a = Vector2::new(1.051f32, 2.05);
    let b = Vector2::new(1.051f32, 2.05);
    let actual = a.distance(b);
    assert_eq!(0.0f32, actual);
}

#[test]
fn vector2_distance_squared_test() {
    let a = Vector2::new(1.0f32, 2.0);
    let b = Vector2::new(3.0f32, 4.0);
    let expected = 8.0f32;
    let actual = a.distance_squared(b);
    assert_eq!(expected, actual, "Vector2::distance_squared did not return the expected value.");
}

/** */
// A test for Dot (Vector2f, Vector2f)
#[test]
fn vector2_dot_test() {
    let a = Vector2::new(1.0f32, 2.0);
    let b = Vector2::new(3.0f32, 4.0);
    let expected = 11.0f32;
    let actual = a.dot(b);
    assert_eq!(expected, actual, "Vector2f.Dot did not return the expected value.");
}

// A test for Dot (Vector2f, Vector2f)
// Dot test for perpendicular vector
#[test]
fn vector2_dot_test1() {
    let a = Vector2::new(1.55f32, 1.55);
    let b = Vector2::new(-1.55f32, 1.55);
    let expected = 0.0f32;
    let actual = a.dot(b);
    assert_eq!(expected, actual);
}

// A test for Dot (Vector2f, Vector2f)
// Dot test with special float values
#[test]
fn vector2_dot_test2() {
    let a = Vector2::new(f32::MIN, f32::MIN);
    let b = Vector2::new(f32::MAX, f32::MAX);
    let actual = a.dot(b);
    assert!(actual.is_infinite() && actual.is_sign_negative(), "Vector2f.Dot did not return the expected value.");
}

// A test for Length ()
#[test]
fn vector2_length_test() {
    let a = Vector2::new(2.0f32, 4.0);
    let target = a;
    let expected = 20.0f32.sqrt();
    let actual = target.length();
    assert_eq!(expected, actual, "Vector2f.Length did not return the expected value.");
}

// A test for Length ()
// Length test where length is zero
#[test]
fn vector2_length_test1() {
    let target = Vector2::new(0.0f32, 0.0);
    let expected = 0.0f32;
    let actual = target.length();
    assert_eq!(expected, actual, "Vector2f.Length did not return the expected value.");
}

// A test for LengthSquared ()
#[test]
fn vector2_length_squared_test() {
    let a = Vector2::new(2.0f32, 4.0);
    let target = a;
    let expected = 20.0f32;
    let actual = target.length_squared();
    assert_eq!(expected, actual, "Vector2f.LengthSquared did not return the expected value.");
}

// A test for LengthSquared ()
// LengthSquared test where the result is zero
#[test]
fn vector2_length_squared_test1() {
    let a = Vector2::new(0.0f32, 0.0);
    let expected = 0.0f32;
    let actual = a.length_squared();
    assert_eq!(expected, actual);
}

#[test]
fn vector2_min_test() {
    let a = Vector2::new(-1.0f32, 4.0);
    let b = Vector2::new(2.0f32, 1.0);

    let expected = Vector2::new(-1.0f32, 1.0);
    let actual = a.min(b);
    assert_eq!(expected, actual, "Vector2f.Min did not return the expected value.");
}

#[test]
fn vector2_min_max_code_coverage_test() {
    let min = Vector2::new(0.0f32, 0.0);
    let max = Vector2::new(1.0f32, 1.0);
    
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
fn vector2_max_test() {
    let a = Vector2::new(-1.0f32, 4.0);
    let b = Vector2::new(2.0f32, 1.0);

    let expected = Vector2::new(2.0f32, 4.0);
    let actual = a.max(b);
    assert_eq!(expected, actual, "Vector2f.Max did not return the expected value.");
}

#[test]
fn vector2_clamp_test() {
    let a = Vector2::new(0.5f32, 0.3);
    let min = Vector2::new(0.0f32, 0.1);
    let max = Vector2::new(1.0f32, 1.1);

    // Normal case.
    // Case N1: specified value is in the range.
    let expected = Vector2::new(0.5f32, 0.3);
    let actual = a.clamp(min, max);
    assert_eq!(expected, actual, "Vector2f.Clamp did not return the expected value.");

    // Normal case.
    // Case N2: specified value is bigger than max value.
    let a = Vector2::new(2.0f32, 3.0);
    let expected = max;
    let actual = a.clamp(min, max);
    assert_eq!(expected, actual, "Vector2f.Clamp did not return the expected value.");

    // Case N3: specified value is smaller than max value.
    let a = Vector2::new(-1.0f32, -2.0);
    let expected = min;
    let actual = a.clamp(min, max);
    assert_eq!(expected, actual, "Vector2f.Clamp did not return the expected value.");

    // Case N4: combination case.
    let a = Vector2::new(-2.0f32, 4.0);
    let expected = Vector2::new(min.x, max.y);
    let actual = a.clamp(min, max);
    assert_eq!(expected, actual, "Vector2f.Clamp did not return the expected value.");

    // User specified min value is bigger than max value.
    let max = Vector2::new(0.0f32, 0.1);
    let min = Vector2::new(1.0f32, 1.1);

    // Case W1: specified value is in the range.
    let a = Vector2::new(0.5f32, 0.3);
    let expected = min;
    let actual = a.clamp(min, max);
    assert_eq!(expected, actual, "Vector2f.Clamp did not return the expected value.");

    // Normal case.
    // Case W2: specified value is bigger than max and min value.
    let a = Vector2::new(2.0f32, 3.0);
    let expected = min;
    let actual = a.clamp(min, max);
    assert_eq!(expected, actual, "Vector2f.Clamp did not return the expected value.");

    // Case W3: specified value is smaller than min and max value.
    let a = Vector2::new(-1.0f32, -2.0);
    let expected = min;
    let actual = a.clamp(min, max);
    assert_eq!(expected, actual, "Vector2f.Clamp did not return the expected value.");
}

#[test]
fn vector2_lerp_test() {
    let a = Vector2::new(1.0f32, 2.0);
    let b = Vector2::new(3.0f32, 4.0);
    let t = 0.5f32;

    let expected = Vector2::new(2.0f32, 3.0);
    let actual = a.lerp(b, t);
    assert_eq!(expected, actual, "Vector2f.lerp did not return the expected value.");
}

#[test]
fn vector2_lerp_test1() {
    let a = Vector2::new(0.0f32, 0.0);
    let b = Vector2::new(3.18f32, 4.25);
    let t = 0.0f32;
    let expected = Vector2::<f32>::zero();
    let actual = a.lerp(b, t);
    assert_eq!(expected, actual, "Vector2f.lerp did not return the expected value.");
}

#[test]
fn vector2_lerp_test2() {
    let a = Vector2::new(0.0f32, 0.0);
    let b = Vector2::new(3.18f32, 4.25);
    let t = 1.0f32;
    let expected = Vector2::new(3.18f32, 4.25);

    let actual = a.lerp(b, t);
    assert_eq!(expected, actual, "Vector2f.lerp did not return the expected value.");
}

#[test]
fn vector2_lerp_test3() {
    let a = Vector2::new(0.0f32, 0.0);
    let b = Vector2::new(3.18f32, 4.25);
    let t = 2.0f32;
    let expected = b * 2.0f32;

    let actual = a.lerp(b, t);
    assert_eq!(expected, actual, "Vector2f.lerp did not return the expected value.");
}

#[test]
fn vector2_lerp_test4() {
    let a = Vector2::new(0.0f32, 0.0);
    let b = Vector2::new(3.18f32, 4.25);
    let t = -2.0f32;
    let expected = -(b * 2.0f32);

    let actual = a.lerp(b, t);
    assert_eq!(expected, actual, "Vector2f.lerp did not return the expected value.");
}

#[test]
fn vector2_lerp_test5() {
    let a = Vector2::new(45.67f32, 90.0);
    let b = Vector2::new(f32::INFINITY, f32::NEG_INFINITY);
    let t = 0.408f32;

    let actual = a.lerp(b, t);
    assert!(actual.x.is_infinite() && actual.x.is_sign_positive(), "Vector2f.lerp did not return the expected value.");
    assert!(actual.y.is_infinite() && actual.y.is_sign_negative(), "Vector2f.lerp did not return the expected value.");
}

#[test]
fn vector2_lerp_test6() {
    let a = Vector2::new(1.0f32, 2.0);
    let b = Vector2::new(1.0f32, 2.0);
    let t = 0.5f32;

    let expected = Vector2::new(1.0f32, 2.0);
    let actual = a.lerp(b, t);
    assert_eq!(expected, actual)
}


/*/ */
// A test for Transform(Vector2f, Matrix4x4)
#[test]
fn vector2_transform_test() {
    let v = Vector2::new(1.0f32, 2.0);
    let mut m = Matrix4x4::create_rotation_x(30.0_f32.to_radians())
        * Matrix4x4::create_rotation_y(30.0_f32.to_radians())
        * Matrix4x4::create_rotation_z(30.0_f32.to_radians());
    m.m41 = 10.0f32;
    m.m42 = 20.0f32;
    m.m43 = 30.0f32;

    let expected = Vector2::new(10.316987f32, 22.183012f32);

    let actual = v.transform(m);
    assert_eq!(expected, actual, "Vector2f.Transform did not return the expected value.");
}

// A test for TransformNormal (Vector2f, Matrix4x4)
#[test]
fn vector2_transform_normal_test() {
    let v = Vector2::new(1.0f32, 2.0);
    let mut m = Matrix4x4::create_rotation_x(30.0_f32.to_radians())
        * Matrix4x4::create_rotation_y(30.0_f32.to_radians())
        * Matrix4x4::create_rotation_z(30.0_f32.to_radians());
    m.m41 = 10.0f32;
    m.m42 = 20.0f32;
    m.m43 = 30.0f32;

    let expected = Vector2::new(0.3169873f32, 2.18301272);

    let actual = v.transform_normal(m);
    assert_eq!(expected, actual, "Vector2f.Transform did not return the expected value.");
}

// A test for Transform (Vector2f, Quaternion)
#[test]
fn vector2_transform_by_quaternion_test() {
    let v = Vector2::new(1.0f32, 2.0);
    let m = Matrix4x4::create_rotation_x(30.0_f32.to_radians())
        * Matrix4x4::create_rotation_x(30.0_f32.to_radians())
        * Matrix4x4::create_rotation_x(30.0_f32.to_radians());
    let q = Quaternion::create_from_rotation_matrix(m);

    let expected = v.transform(m);
    let actual = v.transform_quaternion(q);
    assert!(expected.almost_equals(&actual, 0.000001));
}

// A test for Transform (Vector2f, Quaternion)
// Transform Vector2f with zero quaternion
#[test]
fn vector2_transform_by_quaternion_test1() {
    let v = Vector2::new(1.0f32, 2.0);
    let q = Quaternion::<f32>::identity();
    let expected = v;

    let actual = v.transform_quaternion(q);
    assert_eq!(expected, actual, "Vector2f.Transform did not return the expected value.");
}

/** */ 
// A test for Transform (Vector2f, Quaternion)
// Transform Vector2f with identity quaternion
#[test]
fn vector2_transform_by_quaternion_test2() {
    let v = Vector2::new(1.0f32, 2.0f32);
    let q = Quaternion::<f32>::identity();
    let expected = v;

    let actual = v.transform_quaternion(q);
    assert_eq!(expected, actual, "Vector2f.Transform did not return the expected value.");
}

// A test for Normalize (Vector2f)
#[test]
fn vector2_normalize_test() {
    let a = Vector2::new(2.0f32, 3.0f32);
    let expected = Vector2::new(0.554700196225229122018341733457f32, 0.8320502943378436830275126001855f32);
    assert_eq!(a.normalize(), expected);
}

// A test for Normalize (Vector2f)
// Normalize zero length vector
#[test]
fn vector2_normalize_test1() {
    let a = Vector2::new(0.0f32, 0.0f32);
    let actual = a.normalize();
    assert!(actual.x.is_nan() && actual.y.is_nan(), "Vector2f.Normalize did not return the expected value.");
}

// A test for Normalize (Vector2f)
// Normalize infinite length vector
#[test]
fn vector2_normalize_test2() {
    let a = Vector2::new(f32::MAX, f32::MAX);
    let actual = a.normalize();
    let expected = Vector2::new(0.0f32, 0.0);
    assert_eq!(expected, actual);
}

/* */
#[test]
fn vector2_unary_negation_test() {
    let a = Vector2::new(1.0f32, 2.0);
    let expected = Vector2::new(-1.0f32, -2.0);
    let actual = -a;
    assert_eq!(actual, expected, "Vector2f.operator - did not return the expected value.");
}

#[test]
fn vector2_unary_negation_test1() {
    let a = Vector2::new(f32::INFINITY, f32::NEG_INFINITY);
    let actual = -a;
    assert!(actual.x.is_infinite() && actual.x.is_sign_negative(), "Vector2f.operator - did not return the expected value.");
    assert!(actual.y.is_infinite() && actual.y.is_sign_positive(), "Vector2f.operator - did not return the expected value.");
}

#[test]
fn vector2_unary_negation_test2() {
    let a = Vector2::new(f32::NAN, 0.0);
    let actual = -a;
    assert!(actual.x.is_nan(), "Vector2f.operator - did not return the expected value.");
    assert_eq!(actual.y, 0.0f32, "Vector2f.operator - did not return the expected value.");
}

#[test]
fn vector2_subtraction_test() {
    let a = Vector2::new(1.0f32, 3.0);
    let b = Vector2::new(2.0f32, 1.5);
    let expected = Vector2::new(-1.0f32, 1.5);
    let actual = a - b;
    assert_eq!(actual, expected, "Vector2f.operator - did not return the expected value.");
}

/** */
// A test for operator * (Vector2f, float)
#[test]
fn vector2_multiply_operator_test() {
    let a = Vector2::new(2.0f32, 3.0);
    let factor = 2.0f32;
    let expected = Vector2::new(4.0f32, 6.0);
    let actual = a * factor;
    assert_eq!(actual, expected, "Vector2f.operator * did not return the expected value.");
}

// A test for operator * (float, Vector2f)
#[test]
fn vector2_multiply_operator_test2() {
    let a = Vector2::new(2.0f32, 3.0);
    let factor = 2.0f32;

    let expected = Vector2::new(4.0f32, 6.0);

    let actual = a * factor ;
    assert_eq!(actual, expected, "Vector2f.operator * did not return the expected value.");
}

// A test for operator * (Vector2f, Vector2f)
#[test]
fn vector2_multiply_operator_test3() {
    let a = Vector2::new(2.0f32, 3.0);
    let b = Vector2::new(4.0f32, 5.0);
    let expected = Vector2::new(8.0f32, 15.0);
    let actual = a * b;
    assert_eq!(actual, expected, "Vector2f.operator * did not return the expected value.");
}

// A test for operator / (Vector2f, float)
#[test]
fn vector2_division_test() {
    let a = Vector2::new(2.0f32, 3.0);
    let div = 2.0f32;
    let expected = Vector2::new(1.0f32, 1.5);
    let actual = a / div;
    assert_eq!(actual, expected, "Vector2f.operator / did not return the expected value.");
}

// A test for operator / (Vector2f, Vector2f)
#[test]
fn vector2_division_test1() {
    let a = Vector2::new(2.0f32, 3.0);
    let b = Vector2::new(4.0f32, 5.0);
    let expected = Vector2::new(2.0f32 / 4.0f32, 3.0f32 / 5.0f32);
    let actual = a / b;
    assert_eq!(actual, expected, "Vector2f.operator / did not return the expected value.");
}

// A test for operator / (Vector2f, float)
// Divide by zero
#[test]
fn vector2_division_test2() {
    let a = Vector2::new(-2.0f32, 3.0f32);
    let div = 0.0f32;
    let actual = a / div;
    assert!(actual.x.is_infinite() && actual.x.is_sign_negative(), "Vector2f.operator / did not return the expected value.");
    assert!(actual.y.is_infinite() && actual.y.is_sign_positive(), "Vector2f.operator / did not return the expected value.");
}

// A test for operator / (Vector2f, Vector2f)
// Divide by zero
#[test]
fn vector2_division_test3() {
    let a = Vector2::new(0.047f32, -3.0);
    let b = Vector2::new(0.0f32, 0.0);

    let actual = a / b;

    assert!(actual.x.is_infinite(), "Vector2f.operator / did not return the expected value.");
    assert!(actual.y.is_infinite(), "Vector2f.operator / did not return the expected value.");
}

// A test for operator + (Vector2f, Vector2f)
#[test]
fn vector2_addition_test() {
    let a = Vector2::new(1.0f32, 2.0);
    let b = Vector2::new(3.0f32, 4.0);
    let expected = Vector2::new(4.0f32, 6.0);
    let actual = a + b;
    assert_eq!(actual, expected, "Vector2f.operator + did not return the expected value.");
}

// A test for Vector2f (float, float)
#[test]
fn vector2_constructor_test() {
    let x = 1.0f32;
    let y = 2.0f32;

    let target = Vector2::new(x, y);
    assert!(equal(target.x, x) && equal(target.y, y), "Vector2f(x,y) constructor did not return the expected value.");
}

// A test for Vector2f ()
// Constructor with no parameter
#[test]
fn vector2_constructor_test2() {
    let target = Vector2::new(0.0, 0.0);
    assert!(target.x == 0.0);
    assert!(target.y == 0.0);
}

// A test for Vector2f (float, float)
// Constructor with special floating values
#[test]
fn vector2_constructor_test3() {
    let target = Vector2::new(f32::NAN, f32::MAX);
    assert!(target.x.is_nan());
    assert!(target.y.eq(&f32::MAX));
}

// A test for Vector2f (float)
#[test]
fn vector2_constructor_test4() {
    let value = 1.0f32;
    let target = Vector2::from_num(value);

    let expected = Vector2::new(value, value);
    assert_eq!(expected, target);

    let value = 2.0f32;
    let target = Vector2::from_num(value);
    let expected = Vector2::new(value, value);
    assert_eq!(expected, target);
}

// A test for Add (Vector2f, Vector2f)
#[test]
fn vector2_add_test() {
    let a = Vector2::new(1.0f32, 2.0);
    let b = Vector2::new(5.0f32, 6.0);
    let expected = Vector2::new(6.0f32, 8.0);
    let actual = a.add(b);
    assert_eq!(expected, actual);
}

/** */
#[test]
fn vector2_divide_test() {
    let a = Vector2::new(1.0f32, 2.0);
    let div = 2.0f32;
    let expected = Vector2::new(0.5f32, 1.0);
    assert_eq!(expected, a / div);
}

#[test]
fn vector2_divide_test1() {
    let a = Vector2::new(1.0f32, 6.0);
    let b = Vector2::new(5.0f32, 2.0);
    let expected = Vector2::new(1.0f32 / 5.0f32, 6.0f32 / 2.0f32);
    let actual = a.div(b);
    assert_eq!(expected, actual);
}

#[test]
fn vector2_equals_test() {
    let a = Vector2::new(1.0f32, 2.0);
    let b = Vector2::new(1.0f32, 2.0);

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

/** */
#[test]
fn vector2_multiply_test() {
    let a = Vector2::new(1.0f32, 2.0);
    let factor = 2.0f32;
    let expected = Vector2::new(2.0f32, 4.0);
    assert_eq!(expected, a * factor);
}

#[test]
fn vector2_multiply_test2() {
    let a = Vector2::new(1.0f32, 2.0);
    let factor = 2.0;
    let expected = Vector2::new(2.0f32, 4.0);
    assert_eq!(expected, a.mul(factor));
}

#[test]
fn vector2_multiply_test3() {
    let a = Vector2::new(1.0f32, 2.0);
    let b = Vector2::new(5.0f32, 6.0);
    let expected = Vector2::new(5.0f32, 12.0);
    let actual = a.mul(b);
    assert_eq!(expected, actual);
}

#[test]
fn vector2_negate_test() {
    let a = Vector2::new(1.0f32, 2.0f32);
    let expected = Vector2::new(-1.0f32, -2.0f32);
    let actual = a.neg();
    assert_eq!(expected, actual);
}

#[test]
fn vector2_inequality_test() {
    let a = Vector2::new(1.0f32, 2.0);
    let b = Vector2::new(1.0f32, 2.0);

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
fn vector2_equality_test() {
    let a = Vector2::new(1.0f32, 2.0);
    let b = Vector2::new(1.0f32, 2.0);

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

/** */
// A test for Subtract (Vector2f, Vector2f)
#[test]
fn vector2_subtract_test() {
    let a = Vector2::new(1.0f32, 6.0);
    let b = Vector2::new(5.0f32, 2.0);

    let expected = Vector2::new(-4.0f32, 4.0);

    let actual = a.sub(b);
    assert_eq!(expected, actual);
}

// A test for UnitX
#[test]
fn vector2_unit_x_test() {
    let val = Vector2::new(1.0f32, 0.0);
    assert_eq!(val, Vector2::unit_x());
}

// A test for UnitY
#[test]
fn vector2_unit_y_test() {
    let val = Vector2::new(0.0f32, 1.0);
    assert_eq!(val, Vector2::unit_y());
}

// A test for One
#[test]
fn vector2_one_test() {
    let val = Vector2::new(1.0f32, 1.0);
    assert_eq!(val, Vector2::one());
}

// A test for Zero
#[test]
fn vector2_zero_test() {
    let val = Vector2::new(0.0f32, 0.0);
    assert_eq!(val, Vector2::zero());
}

// A test for Equals (Vector2f)
#[test]
fn vector2_equals_test1() {
    let a = Vector2::new(1.0f32, 2.0);
    let b = Vector2::new(1.0f32, 2.0);

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

// A test for Vector2f comparison involving NaN values
#[test]
fn vector2_equals_nan_test() {
    let a = Vector2::new(f32::NAN, 0.0f32);
    let b = Vector2::new(0.0f32, f32::NAN);

    assert_ne!(a, Vector2::<f32>::zero());
    assert_ne!(b, Vector2::<f32>::zero());

    assert_ne!(a, b);
    assert_ne!(b, a);

    // Counterintuitive result - IEEE rules for NaN comparison are weird!
    assert_ne!(a, a);
    assert_ne!(b, b);
}

/*** */
#[test]
fn vector2_reflect_test() {
    let a = Vector2::new(1.0f32, 1.0).normalize();

    // Reflect on XZ plane.
    let n = Vector2::new(0.0f32, 1.0);
    let expected = Vector2::new(a.x, -a.y);
    let actual = a.reflect(n);
    assert_eq!(expected, actual, "Vector2f.Reflect did not return the expected value.");

    // Reflect on XY plane.
    let n = Vector2::new(0.0f32, 0.0);
    let expected = Vector2::new(a.x, a.y);
    let actual = a.reflect(n);
    assert_eq!(expected, actual, "Vector2f.Reflect did not return the expected value.");

    // Reflect on YZ plane.
    let n = Vector2::new(1.0f32, 0.0);
    let expected = Vector2::new(-a.x, a.y);
    let actual = a.reflect(n);
    assert_eq!(expected, actual, "Vector2f.Reflect did not return the expected value.");
}

#[test]
fn vector2_reflect_test_1() {
    let n = Vector2::new(0.45f32, 1.28).normalize();
    let a = n;

    let expected = -n;
    let actual = a.reflect(n);
    assert_eq!(expected, actual, "Vector2f.Reflect did not return the expected value.");
}

#[test]
fn vector2_reflect_test_2() {
    let n = Vector2::new(0.45f32, 1.28).normalize();
    let a = -n;

    let expected = n;
    let actual = a.reflect(n);
    assert_eq!(expected, actual, "Vector2f.Reflect did not return the expected value.");
}

#[test]
fn vector2_abs_test() {
    let v1 = Vector2::new(-2.5f32, 2.0);
    let v3 = Vector2::new(0.0f32, f32::NEG_INFINITY).abs();
    let v = v1.abs();
    assert_eq!(2.5f32, v.x);
    assert_eq!(2.0f32, v.y);
    assert_eq!(0.0f32, v3.x);
    assert!(v3.y.is_infinite() && v3.y.is_sign_positive());
}

#[test]
fn vector2_sqrt_test() {
    let v1 = Vector2::new(-2.5f32, 2.0);
    let v2 = Vector2::new(5.5f32, 4.5);
    assert_eq!(2, v2.square_root().x as i32);
    assert_eq!(2, v2.square_root().y as i32);
    assert!(v1.square_root().x.is_nan());
}
