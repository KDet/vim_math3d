use num_traits::Float;
use vim_math3d::{self, Matrix4x4, Quaternion, Vector3};

fn equal<T: Float>(a: T, b: T) -> bool { (a - b).abs() < T::from(1e-5).unwrap() }

#[test]
fn quaternion_dot_test() {
    let a = Quaternion::new(4.0f32, 1.0, 2.0, 3.0);
    let b = Quaternion::new(8.0f32, 5.0, 6.0, 7.0);

    let expected = 70.0f32;
    let actual = a.dot(b);
    assert!(equal(expected, actual));
}

#[test]
fn quaternion_length_test() {
    let target = Quaternion::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let expected = 5.477226f32;
    let actual = target.length();
    assert!(equal(expected, actual));
}

#[test]
fn quaternion_length_squared_test() {
    let v = Quaternion::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
    let expected = 30.0f32;
    let actual = v.length_squared();
    assert!(equal(expected, actual));
}

/** */
#[test]
fn quaternion_lerp_test() {
    let axis = Vector3::new(1.0f32, 2.0f32, 3.0f32).normalize();
    let a = Quaternion::new_from_axis_angle(axis, 10f32.to_radians());
    let b = Quaternion::new_from_axis_angle(axis, 30f32.to_radians());

    let t = 0.5f32;
    let expected = Quaternion::new_from_axis_angle(axis, 20f32.to_radians());
    let actual = Quaternion::lerp(a, b, t);
    assert_eq!(expected, actual);

    let expected = a;
    let actual = Quaternion::lerp(a, a, t);
    assert_eq!(expected, actual);
}

#[test]
fn quaternion_lerp_test1() {
    let axis = Vector3::new(1.0f32, 2.0, 3.0).normalize();
    let a = Quaternion::new_from_axis_angle(axis, 10f32.to_radians());
    let b = Quaternion::new_from_axis_angle(axis, 30f32.to_radians());

    let t = 0.0f32;
    let expected = Quaternion::new(a.x, a.y, a.z, a.w);
    let actual = Quaternion::lerp(a, b, t);
    assert_eq!(expected, actual);
}

/*** */
#[test]
fn quaternion_lerp_test2() {
    let axis = Vector3::new(1.0f32, 2.0, 3.0).normalize();
    let a = Quaternion::new_from_axis_angle(axis, 10f32.to_radians());
    let b = Quaternion::new_from_axis_angle(axis, 30f32.to_radians());

    let t = 1.0f32;
    let expected = Quaternion::new(b.x, b.y, b.z, b.w);
    let actual = Quaternion::lerp(a, b, t);
    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));
}

#[test]
fn quaternion_lerp_test3() {
    let axis = Vector3::new(1.0f32, 2.0, 3.0).normalize();
    let a = Quaternion::new_from_axis_angle(axis, 10f32.to_radians());
    let b = -a;

    let t = 1.0f32;
    let actual = Quaternion::lerp(a, b, t);
    assert_eq!(a, actual);
}

#[test]
fn quaternion_conjugate_test1() {
    let a = Quaternion::new(1.0f32, 2.0, 3.0, 4.0f32);
    let expected = Quaternion::new(-1.0f32, -2.0, -3.0, 4.0);
    let actual = a.conjugate();
    assert_eq!(expected, actual);
}

#[test]
fn quaternion_normalize_test() {
    let a = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);
    let expected = Quaternion::new(0.182574168f32, 0.365148336, 0.5477225, 0.7302967f32);
    let actual = a.normalize();
    //assert_eq!(expected, actual);
    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));
}


#[test]
fn quaternion_normalize_test1() {
    let a = Quaternion::new(0.0f32, 0.0, -0.0, 0.0);
    let actual = a.normalize();
    assert!(actual.x.is_nan());
    assert!(actual.y.is_nan());
    assert!(actual.z.is_nan());
    assert!(actual.w.is_nan());
}

#[test]
fn quaternion_concatenate_test1() {
    let b = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);
    let a = Quaternion::new(5.0f32, 6.0, 7.0, 8.0);

    let expected = Quaternion::new(24.0f32, 48.0, 48.0, -6.0);
    let actual = Quaternion::concatenate(a, b);

    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));
}

#[test]
fn quaternion_subtraction_test() {
    let a = Quaternion::new(1.0f32, 6.0, 7.0, 4.0);
    let b = Quaternion::new(5.0f32, 2.0, 3.0, 8.0f32);

    let expected = Quaternion::new(-4.0f32, 4.0, 4.0, -4.0f32);
    let actual = a - b;

    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));
}

#[test]
fn quaternion_multiply_test() {
    let a = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);
    let factor = 0.5f32;

    let expected = Quaternion::new(0.5f32, 1.0, 1.5, 2.0);
    let actual = a * factor;

    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));
}

#[test]
fn quaternion_multiply_test1() {
    let a = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Quaternion::new(5.0f32, 6.0, 7.0, 8.0);

    let expected = Quaternion::new(24.0f32, 48.0, 48.0, -6.0);
    let actual = a * b;

    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));
}

// A test for operator / (Quaternion, Quaternion)
#[test]
fn quaternion_division_test1() {
    let a = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Quaternion::new(5.0f32, 6.0, 7.0, 8.0);

    let expected = Quaternion::new(-0.045977015f32, -0.09195402, -7.450581E-9, 0.402298868);
    let actual = a / b;

    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));
}

// A test for operator + (Quaternion, Quaternion)
#[test]
fn quaternion_addition_test() {
    let a = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Quaternion::new(5.0f32, 6.0, 7.0, 8.0);

    let expected = Quaternion::new(6.0f32, 8.0, 10.0, 12.0);
    let actual = a + b;

    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));
}

// A test for Quaternion (f32, f32, f32, f32)
#[test]
fn quaternion_constructor_test() {
    let x = 1.0f32;
    let y = 2.0f32;
    let z = 3.0f32;
    let w = 4.0f32;
    let target = Quaternion::new(x, y, z, w);

    assert!(equal(x, target.x));
    assert!(equal(y, target.y));
    assert!(equal(z, target.z));
    assert!(equal(w, target.w));
}

// A test for Quaternion (Vector3f, f32)
#[test]
fn quaternion_constructor_test1() {
    let v = Vector3::new(1.0f32, 2.0, 3.0);
    let w = 4.0f32;

    let target = Quaternion::new_from_vector(v, w);

    assert!(equal(v.x, target.x));
    assert!(equal(v.y, target.y));
    assert!(equal(v.z, target.z));
    assert!(equal(w, target.w));
}

// A test for create_from_axis_angle (Vector3, f32)
#[test]
fn quaternion_create_from_axis_angle_test() {
    let axis = Vector3::new(1.0f32, 2.0, 3.0).normalize();
    let angle = 30.0f32.to_radians();

    let expected = Quaternion::new(0.0691723f32, 0.1383446, 0.207516879, 0.9659258);
    let actual = Quaternion::new_from_axis_angle(axis, angle);
    
    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));
}

// A test for create_from_axis_angle (Vector3, f32)
// CreateFromAxisAngle of zero vector
#[test]
fn quaternion_create_from_axis_angle_test1() {
    let axis = Vector3::<f32>::zero();
    let angle = -30.0f32.to_radians();

    let cos = f32::cos(angle / 2.0f32);
    let actual = Quaternion::new_from_axis_angle(axis, angle);

    assert!(actual.x == 0.0 && actual.y == 0.0 && actual.z == 0.0 && equal(cos, actual.w)
        , "Quaternion::create_from_axis_angle did not return the expected value.");
}

// A test for create_from_axis_angle (Vector3, f32)
// CreateFromAxisAngle of angle = 30 && 750
#[test]
fn quaternion_create_from_axis_angle_test2() {
    let axis = Vector3::new(1.0f32, 0.0, 0.0);
    let angle1 = 30.0f32.to_radians();
    let angle2 = 750.0f32.to_radians();

    let actual1 = Quaternion::new_from_axis_angle(axis, angle1);
    let actual2 = Quaternion::new_from_axis_angle(axis, angle2);
    
    assert!(equal(actual1.x, actual2.x));
    assert!(equal(actual1.y, actual2.y));
    assert!(equal(actual1.z, actual2.z));
    assert!(equal(actual1.w, actual2.w));
}

// A test for create_from_axis_angle (Vector3, f32)
// CreateFromAxisAngle of angle = 30 && 390
#[test]
fn quaternion_create_from_axis_angle_test3() {
    let axis = Vector3::new(1.0f32, 0.0, 0.0);
    let angle1 = 30.0f32.to_radians();
    let angle2 = 390.0f32.to_radians();

    let mut actual1 = Quaternion::new_from_axis_angle(axis, angle1);
    let actual2 = Quaternion::new_from_axis_angle(axis, angle2);
    actual1.x = -actual1.x;
    actual1.w = -actual1.w;

    assert!(equal(actual1.x, actual2.x));
    assert!(equal(actual1.y, actual2.y));
    assert!(equal(actual1.z, actual2.z));
    assert!(equal(actual1.w, actual2.w));
}

#[test]
fn quaternion_create_from_yaw_pitch_roll_test1() {
    let yaw_angle = 30.0f32.to_radians();
    let pitch_angle = 40.0f32.to_radians();
    let roll_angle = 50.0f32.to_radians();

    let yaw = Quaternion::new_from_axis_angle(Vector3::unit_y(), yaw_angle);
    let pitch = Quaternion::new_from_axis_angle(Vector3::unit_x(), pitch_angle);
    let roll = Quaternion::new_from_axis_angle(Vector3::unit_z(), roll_angle);

    let expected = yaw * pitch * roll;
    let actual = Quaternion::new_from_yaw_pitch_roll(yaw_angle, pitch_angle, roll_angle);

    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));
}

#[test]
fn quaternion_create_from_yaw_pitch_roll_test2() {
    const STEP: i32 = 35;
    for yaw_angle in -720..=720 {
        if yaw_angle % STEP != 0 { continue; }

        for pitch_angle in -720..=720 {
            if pitch_angle % STEP != 0 { continue; }

            for roll_angle in -720..=720 {
                if roll_angle % STEP != 0 { continue; }

                let yaw_rad = (yaw_angle as f32).to_radians();
                let pitch_rad = (pitch_angle as f32).to_radians();
                let roll_rad = (roll_angle as f32).to_radians();

                let yaw = Quaternion::new_from_axis_angle(Vector3::unit_y(), yaw_rad);
                let pitch = Quaternion::new_from_axis_angle(Vector3::unit_x(), pitch_rad);
                let roll = Quaternion::new_from_axis_angle(Vector3::unit_z(), roll_rad);

                let expected = yaw * pitch * roll;
                let actual = Quaternion::new_from_yaw_pitch_roll(yaw_rad, pitch_rad, roll_rad);

                assert!(equal(expected.x, actual.x));
                assert!(equal(expected.y, actual.y));
                assert!(equal(expected.z, actual.z));
                assert!(equal(expected.w, actual.w));
            }
        }
    }
}

// A test for Slerp (Quaternion, Quaternion, float)
#[test]
fn quaternion_slerp_test() {
    let axis = Vector3::new(1.0f32, 2.0f32, 3.0).normalize();
    let a = Quaternion::new_from_axis_angle(axis, 10.0f32.to_radians());
    let b = Quaternion::new_from_axis_angle(axis, 30.0f32.to_radians());

    let t = 0.5f32;
    let expected = Quaternion::new_from_axis_angle(axis, 20.0f32.to_radians());
    let actual = Quaternion::slerp(a, b, t);

    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));

    // Case a and b are same.
    let expected = a;
    let actual = Quaternion::slerp(a, a, t);

    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));
}

// A test for Slerp (Quaternion, Quaternion, float)
// Slerp test where t = 0
#[test]
fn quaternion_slerp_test1() {
    let axis = Vector3::new(1.0f32, 2.0f32, 3.0f32).normalize();
    let a = Quaternion::new_from_axis_angle(axis, 10.0f32.to_radians());
    let b = Quaternion::new_from_axis_angle(axis, 30.0f32.to_radians());

    let t = 0.0f32;
    let expected = a;
    let actual = Quaternion::slerp(a, b, t);

    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));
}

// A test for Slerp (Quaternion, Quaternion, float)
// Slerp test where t = 1
#[test]
fn quaternion_slerp_test2() {
    let axis = Vector3::new(1.0f32, 2.0, 3.0).normalize();
    let a = Quaternion::new_from_axis_angle(axis, 10.0f32.to_radians());
    let b = Quaternion::new_from_axis_angle(axis, 30.0f32.to_radians());

    let t = 1.0f32;

    let expected = b;
    let actual = Quaternion::slerp(a, b, t);
    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));
}

// A test for Slerp (Quaternion, Quaternion, float)
// Slerp test where dot product is < 0
#[test]
fn quaternion_slerp_test3() {
    let axis = Vector3::new(1.0f32, 2.0f32, 3.0f32).normalize();
    let a = Quaternion::new_from_axis_angle(axis, 10.0f32.to_radians());
    let b = -a;

    let t = 1.0;

    let expected = a;
    let actual = Quaternion::slerp(a, b, t);
    // Note that in quaternion world, Q == -Q. In the case of quaternions dot product is zero,
    // one of the quaternion will be flipped to compute the shortest distance. When t = 1, we
    // expect the result to be the same as quaternion b but flipped.
    assert_eq!(expected, actual);
}

#[test]
fn quaternion_slerp_test4() {
    let axis = Vector3::new(1.0f32, 2.0, 3.0).normalize();
    let a = Quaternion::new_from_axis_angle(axis, 10.0f32.to_radians());
    let b = Quaternion::new_from_axis_angle(axis, 30.0f32.to_radians());
    let t = 0.0f32;

    let expected = Quaternion::new(a.x, a.y, a.z, a.w);
    let actual = Quaternion::slerp(a, b, t);
    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));
}

#[test]
fn quaternion_unary_negation_test() {
    let a = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);
    let expected = Quaternion::new(-1.0f32, -2.0, -3.0, -4.0);
    let actual = -a;
    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));
}

#[test]
fn quaternion_inverse_test() {
    let a = Quaternion::new(5.0f32, 6.0, 7.0, 8.0);
    let expected = Quaternion::new(-0.0287356321f32, -0.03448276, -0.0402298868, 0.04597701);
    let actual = a.inverse();
    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));
}

#[test]
fn quaternion_inverse_test1() {
    let a = Quaternion::new(0.0f32, 0.0, 0.0, 0.0);
    let actual = a.inverse();
    assert!(actual.x.is_nan() && actual.y.is_nan() && actual.z.is_nan() && actual.w.is_nan());
}

#[test]
fn quaternion_add_test() {
    let a = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Quaternion::new(5.0f32, 6.0, 7.0, 8.0);
    let expected = Quaternion::new(6.0f32, 8.0, 10.0, 12.0);
    assert_eq!(expected, a + b);
}

#[test]
fn quaternion_divide_test() {
    let a = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Quaternion::new(5.0f32, 6.0, 7.0, 8.0);
    let expected = Quaternion::new(-0.045977015f32, -0.09195402, -7.450581E-9, 0.402298868);
    let actual = a / b;
    assert!(equal(expected.x, actual.x));
    assert!(equal(expected.y, actual.y));
    assert!(equal(expected.z, actual.z));
    assert!(equal(expected.w, actual.w));
}

#[test]
fn quaternion_multiply_test2() {
    let a = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);
    let factor = 0.5f32;
    let expected = Quaternion::new(0.5f32, 1.0, 1.5, 2.0);
    assert_eq!(expected, a * factor);
}

#[test]
fn quaternion_multiply_test3() {
    let a = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Quaternion::new(5.0f32, 6.0, 7.0, 8.0);
    let expected = Quaternion::new(24.0f32, 48.0, 48.0, -6.0);
    assert_eq!(expected, a * b);
}

#[test]
fn quaternion_negate_test() {
    let a = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);
    let expected = Quaternion::new(-1.0f32, -2.0, -3.0, -4.0);
    assert_eq!(expected, -a);
}


// A test for Subtract (Quaternion, Quaternion)
#[test]
fn quaternion_subtract_test() {
    let a = Quaternion::new(1.0f32, 6.0, 7.0, 4.0);
    let b = Quaternion::new(5.0f32, 2.0, 3.0, 8.0);

    let expected = Quaternion::new(-4.0f32, 4.0, 4.0, -4.0);
    assert_eq!(expected, a - b);
}

// A test for operator != (Quaternion, Quaternion)
#[test]
fn quaternion_inequality_test() {
    let a = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);

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

// A test for operator == (Quaternion, Quaternion)
#[test]
fn quaternion_equality_test() {
    let a = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);

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
fn quaternion_from_rotation_matrix_test1() {
    let matrix = Matrix4x4::<f32>::identity();

    let expected = Quaternion::new(0.0f32, 0.0, 0.0, 1.0);
    let actual = Quaternion::new_from_rotation_matrix(matrix);
    assert_eq!(expected, actual);

    // make sure convert back to matrix is same as we passed matrix.
    let m2 = Matrix4x4::create_from_quaternion(actual);
    assert_eq!(matrix, m2);
}

// A test for from_rotation_matrix (Matrix4x4)
// Convert X axis rotation matrix
#[test]
fn quaternion_from_rotation_matrix_test2() {
    for angle in (0..720).step_by(10) {
        let matrix = Matrix4x4::create_rotation_x(angle as f32);
        let expected = Quaternion::new_from_axis_angle(Vector3::unit_x(), angle as f32);
        let actual = Quaternion::new_from_rotation_matrix(matrix);

        assert!(equal(expected.x, actual.x) || equal(expected.x, -actual.x));
        assert!(equal(expected.y, actual.y) || equal(expected.y, -actual.y));
        assert!(equal(expected.z, actual.z) || equal(expected.z, -actual.z));
        assert!(equal(expected.w, actual.w) || equal(expected.w, -actual.w));

        // make sure convert back to matrix is same as we passed matrix.
        let m2 = Matrix4x4::create_from_quaternion(actual);
        assert!(matrix.almost_equals(&m2, 1e-5));
    }
}

/** */
// A test for CreateFromRotationMatrix (Matrix4x4)
// Convert Y axis rotation matrix
#[test]
fn quaternion_from_rotation_matrix_test3() {
    for angle in (0..720).step_by(10) {
        let matrix = Matrix4x4::create_rotation_y(Float::to_radians(angle as f32));

        let expected = Quaternion::new_from_axis_angle(Vector3::unit_y(), Float::to_radians(angle as f32));
        let actual = Quaternion::new_from_rotation_matrix(matrix);
        assert!(equal(expected.x, actual.x) || equal(expected.x, -actual.x));
        assert!(equal(expected.y, actual.y) || equal(expected.y, -actual.y));
        assert!(equal(expected.z, actual.z) || equal(expected.z, -actual.z));
        assert!(equal(expected.w, actual.w) || equal(expected.w, -actual.w));

        // make sure convert back to matrix is same as we passed matrix.
        let m2 = Matrix4x4::create_from_quaternion(actual);
        assert!(matrix.almost_equals(&m2, 1e-5));
    }
}

// A test for CreateFromRotationMatrix (Matrix4x4)
// Convert Z axis rotation matrix
#[test]
fn quaternion_from_rotation_matrix_test4() {
    for angle in (0..720).step_by(10) {
        let matrix = Matrix4x4::create_rotation_z((angle as f32).to_radians());

        let expected = Quaternion::new_from_axis_angle(Vector3::unit_z(), (angle as f32).to_radians());
        let actual = Quaternion::new_from_rotation_matrix(matrix);
        assert!(equal(expected.x, actual.x) || equal(expected.x, -actual.x), "angle={angle}");
        assert!(equal(expected.y, actual.y) || equal(expected.y, -actual.y), "angle={angle}");
        assert!(equal(expected.z, actual.z) || equal(expected.z, -actual.z), "angle={angle}");
        assert!(equal(expected.w, actual.w) || equal(expected.w, -actual.w), "angle={angle}");

        // make sure convert back to matrix is same as we passed matrix.
        let m2 = Matrix4x4::create_from_quaternion(actual);
        assert!(matrix.almost_equals(&m2, 1e-5), "angle={angle}");
    }
}

// A test for CreateFromRotationMatrix (Matrix4x4)
// Convert XYZ axis rotation matrix
#[test]
fn quaternion_from_rotation_matrix_test5() {
    for angle in (0..720).step_by(10) {
        let matrix = Matrix4x4::create_rotation_x(Float::to_radians(angle as f32))
            * Matrix4x4::create_rotation_y(Float::to_radians(angle as f32))
            * Matrix4x4::create_rotation_z(Float::to_radians(angle as f32));

        let expected =
            Quaternion::new_from_axis_angle(Vector3::unit_z(), Float::to_radians(angle as f32))
            * Quaternion::new_from_axis_angle(Vector3::unit_y(), Float::to_radians(angle as f32))
            * Quaternion::new_from_axis_angle(Vector3::unit_x(), Float::to_radians(angle as f32));

        let actual = Quaternion::new_from_rotation_matrix(matrix);
        assert!(equal(expected.x, actual.x) || equal(expected.x, -actual.x));
        assert!(equal(expected.y, actual.y) || equal(expected.y, -actual.y));
        assert!(equal(expected.z, actual.z) || equal(expected.z, -actual.z));
        assert!(equal(expected.w, actual.w) || equal(expected.w, -actual.w));

        // make sure convert back to matrix is same as we passed matrix.
        let m2 = Matrix4x4::create_from_quaternion(actual);
        assert!(matrix.almost_equals(&m2, 1e-5));
    }
}


/* */
#[test]
fn quaternion_from_rotation_matrix_with_scaled_matrix_test1() {
    let angle = 180.0f32.to_radians();
    let matrix = 
        Matrix4x4::create_rotation_y(angle) * 
        Matrix4x4::create_rotation_z(angle);
    let expected = 
        Quaternion::new_from_axis_angle(Vector3::unit_z(), angle) * 
        Quaternion::new_from_axis_angle(Vector3::unit_y(), angle);
    let actual = Quaternion::new_from_rotation_matrix(matrix);

    assert!(equal(expected.x, actual.x) || equal(expected.x, -actual.x));
    assert!(equal(expected.y, actual.y) || equal(expected.y, -actual.y));
    assert!(equal(expected.z, actual.z) || equal(expected.z, -actual.z));
    assert!(equal(expected.w, actual.w) || equal(expected.w, -actual.w));

    // make sure convert back to matrix is same as we passed matrix.
    let m2 = Matrix4x4::create_from_quaternion(actual);
    assert!(matrix.almost_equals(&m2, 1e-5));
}

#[test]
fn quaternion_from_rotation_matrix_with_scaled_matrix_test2() {
    let angle = 180.0f32.to_radians();
    let matrix = 
        Matrix4x4::create_rotation_x(angle) * 
        Matrix4x4::create_rotation_z(angle);

    let expected = Quaternion::new_from_axis_angle(Vector3::unit_z(), angle)
        * Quaternion::new_from_axis_angle(Vector3::unit_x(), angle);
    let actual = Quaternion::new_from_rotation_matrix(matrix);
    assert!(equal(expected.x, actual.x) || equal(expected.x, -actual.x));
    assert!(equal(expected.y, actual.y) || equal(expected.y, -actual.y));
    assert!(equal(expected.z, actual.z) || equal(expected.z, -actual.z));
    assert!(equal(expected.w, actual.w) || equal(expected.w, -actual.w));

    // make sure convert back to matrix is same as we passed matrix.
    let m2 = Matrix4x4::create_from_quaternion(actual);
    assert!(matrix.almost_equals(&m2, 1e-5));
}

#[test]
fn quaternion_from_rotation_matrix_with_scaled_matrix_test3() {
    let angle = 180.0f32.to_radians();
    let matrix = 
        Matrix4x4::create_rotation_x(angle) * 
        Matrix4x4::create_rotation_y(angle);

    let expected = 
        Quaternion::new_from_axis_angle(Vector3::unit_y(), angle) * 
        Quaternion::new_from_axis_angle(Vector3::unit_x(), angle);
    let actual = Quaternion::new_from_rotation_matrix(matrix);
    assert!(equal(expected.x, actual.x) || equal(expected.x, -actual.x));
    assert!(equal(expected.y, actual.y) || equal(expected.y, -actual.y));
    assert!(equal(expected.z, actual.z) || equal(expected.z, -actual.z));
    assert!(equal(expected.w, actual.w) || equal(expected.w, -actual.w));

    // make sure convert back to matrix is same as we passed matrix.
    let m2 = Matrix4x4::create_from_quaternion(actual);
    assert!(matrix.almost_equals(&m2, 1e-5));
}

#[test]
fn quaternion_equals_test1() {
    let a = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);
    let mut b = Quaternion::new(1.0f32, 2.0, 3.0, 4.0);

    // case 1: compare between same values
    let expected = true;
    let actual = a == b;
    assert_eq!(expected, actual);

    // case 2: compare between different values
    b.x = 10.0f32;
    let expected = false;
    let actual = a == b;
    assert_eq!(expected, actual);
}

#[test]
fn quaternion_identity_test() {
    let val = Quaternion::new(0.0f32, 0.0, 0.0, 1.0);
    assert_eq!(val, Quaternion::<f32>::identity());
}

#[test]
fn quaternion_is_identity_test() {
    assert!(Quaternion::<f32>::identity().is_identity());
    assert!(Quaternion::new(0.0, 0.0, 0.0, 1.0).is_identity());
    assert!(!Quaternion::new(1.0, 0.0, 0.0, 1.0).is_identity());
    assert!(!Quaternion::new(0.0, 1.0, 0.0, 1.0).is_identity());
    assert!(!Quaternion::new(0.0, 0.0, 1.0, 1.0).is_identity());
    assert!(!Quaternion::new(0.0, 0.0, 0.0, 0.0).is_identity());
}

#[test]
fn quaternion_equals_nan_test() {
    let a = Quaternion::new(f32::NAN, 0.0, 0.0, 0.0);
    let b = Quaternion::new(0.0, f32::NAN, 0.0, 0.0);
    let c = Quaternion::new(0.0, 0.0, f32::NAN, 0.0);
    let d = Quaternion::new(0.0, 0.0, 0.0, f32::NAN);

    assert_ne!(a, Quaternion::new(0.0f32, 0.0, 0.0, 0.0));
    assert_ne!(b, Quaternion::new(0.0f32, 0.0, 0.0, 0.0));
    assert_ne!(c, Quaternion::new(0.0f32, 0.0, 0.0, 0.0));
    assert_ne!(d, Quaternion::new(0.0f32, 0.0, 0.0, 0.0));

    assert!(a != Quaternion::new(0.0f32, 0.0, 0.0, 0.0));
    assert!(b != Quaternion::new(0.0f32, 0.0, 0.0, 0.0));
    assert!(c != Quaternion::new(0.0f32, 0.0, 0.0, 0.0));
    assert!(d != Quaternion::new(0.0f32, 0.0, 0.0, 0.0));

    assert_ne!(a, Quaternion::new(0.0f32, 0.0, 0.0, 0.0));
    assert_ne!(b, Quaternion::new(0.0f32, 0.0, 0.0, 0.0));
    assert_ne!(c, Quaternion::new(0.0f32, 0.0, 0.0, 0.0));
    assert_ne!(d, Quaternion::new(0.0f32, 0.0, 0.0, 0.0));

    assert!(!a.is_identity());
    assert!(!b.is_identity());
    assert!(!c.is_identity());
    assert!(!d.is_identity());

    // Counterintuitive result - IEEE rules for NaN comparison are weird!
    assert_ne!(a, a);
    assert_ne!(b, b);
    assert_ne!(c, c);
    assert_ne!(d, d);
}

#[test]
fn to_euler_and_back() {
    let x = std::f32::consts::PI / 5.0;
    let y = 2.0 * std::f32::consts::PI / 7.0;
    let z = std::f32::consts::PI / 3.0;
    let euler = Vector3::new(x, y, z);
    let quat = Quaternion::new_from_euler_angles(euler);
    let euler2 = quat.to_euler_angles();
    assert!(equal(euler.x, euler2.x));
    assert!(equal(euler.y, euler2.y));
    assert!(equal(euler.z, euler2.z));
}

// A test to avoid a floating point NaN precision issue
// when two input normalized vectors are almost parallel
// and pointing in the same direction.
#[test]
fn create_rotation_from_a_to_b_test() {
    let a = Vector3::new(0.57731324f32, 0.57728577, 0.5774519);
    let b = Vector3::new(0.57738256f32, 0.57728577, 0.57738256);

    // Assert precondition that a and b are normalized.
    assert!(equal(a.normalize().length(), a.length()));
    assert!(equal(b.normalize().length(), b.length()));

    // Validate that the returned quaternion does not contain NaN due to precision issues
    let quat = Quaternion::new_rotation_from_a_to_b(a, b, None);
    assert_eq!(Quaternion::identity(), quat);
}
