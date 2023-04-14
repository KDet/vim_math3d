use num_traits::Float;
use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher, f32::consts::{PI, FRAC_PI_2, FRAC_PI_4}, ops::{Add, Mul, Neg, Sub}};
use vim_math3d::{self, Vector3, Matrix4x4, Transformable3D, Quaternion, Plane};

fn equal<T: Float>(a: T, b: T) -> bool { (a - b).abs() < T::from(1e-5).unwrap() }
fn equal_tolerance<T: Float>(a: T, b: T, t: T) -> bool { (a - b).abs() < t }

fn hash_code<T: Float>(v: &Matrix4x4<T>) -> u64 {
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    hasher.finish()
}

fn hash_float<T: Float>(v: T, hasher: &mut DefaultHasher) {
    let (mantissa, exponent, sign) = v.integer_decode();
    mantissa.hash(hasher);
    exponent.hash(hasher);
    sign.hash(hasher);
} 

fn generate_matrix_number_from_1_to_16() -> Matrix4x4<f32> {
    Matrix4x4::<f32>::new(
        1.0f32, 2.0f32, 3.0f32, 4.0,
        5.0f32, 6.0f32, 7.0f32, 8.0,
        9.0f32, 10.0f32, 11.0f32, 12.0,
        13.0f32, 14.0f32, 15.0f32, 16.0,
    )
}

fn generate_test_matrix() -> Matrix4x4<f32> {
    let m = Matrix4x4::create_rotation_x(30.0f32.to_radians())
        * Matrix4x4::create_rotation_y(30.0f32.to_radians())
        * Matrix4x4::create_rotation_z(30.0f32.to_radians());
    m * m.set_translation(Vector3::new(111.0f32, 222.0f32, 333.0))
}

#[test]
fn matrix4x4_identity_test() {
    let val = Matrix4x4::<f32>::identity();
    assert_eq!(val.m11, 1.0f32);
    assert_eq!(val.m22, 1.0f32);
    assert_eq!(val.m33, 1.0f32);
    assert_eq!(val.m44, 1.0f32);

    assert!(val == Matrix4x4::identity(), "Matrix4x4.identity was not set correctly.");
}

// A test for Determinant
#[test]
fn matrix4x4_determinant_test() {
    let target =
        Matrix4x4::create_rotation_x(30.0f32.to_radians()) * 
        Matrix4x4::create_rotation_y(30.0f32.to_radians()) * 
        Matrix4x4::create_rotation_z(30.0f32.to_radians());

    let val = 1.0f32;
    let det = target.get_determinant();

    assert!(equal(val, det), "Matrix4x4.Determinant was not set correctly.");
}

// A test for Determinant
// Determinant test |A| = 1 / |A'|
#[test]
fn matrix4x4_determinant_test1() {
    let mut a = Matrix4x4::<f32>::identity();
    a.m11 = 5.0f32;
    a.m12 = 2.0;
    a.m13 = 8.25;
    a.m14 = 1.0;
    a.m21 = 12.0;
    a.m22 = 6.8;
    a.m23 = 2.14;
    a.m24 = 9.6;
    a.m31 = 6.5;
    a.m32 = 1.0;
    a.m33 = 3.14;
    a.m34 = 2.22;
    a.m41 = 0.0;
    a.m42 = 0.86;
    a.m43 = 4.0;
    a.m44 = 1.0;
    let i = a.inverse();

    let det_a = a.get_determinant();
    let det_i = i.get_determinant();
    let t = 1.0f32 / det_i;

    // only accurate to 3 precision
    assert!((det_a - t).abs() < 1e-3, "Matrix4x4.Determinant was not set correctly.");
}

/** */
#[test]
fn matrix4x4_invert_test() {
    let mtx = 
        Matrix4x4::create_rotation_x(30.0f32.to_radians()) * 
        Matrix4x4::create_rotation_y(30.0f32.to_radians()) * 
        Matrix4x4::create_rotation_z(30.0f32.to_radians());

    let expected = Matrix4x4 {
        m11: 0.75f32,
        m12: -0.216506317,
        m13: 0.625,
        m14: 0.0,
        m21: 0.433012635,
        m22: 0.875,
        m23: -0.216506317,
        m24: 0.0,
        m31: -0.5,
        m32: 0.433012635,
        m33: 0.75,
        m34: 0.0,
        m41: 0.0,
        m42: 0.0,
        m43: 0.0,
        m44: 1.0,
    };

    let actual = mtx.invert();
    assert!(actual.is_some());
    let actual = actual.unwrap();
    assert!(expected.almost_equals(&actual, 1e-5));

    // Make sure M*M is identity matrix
    let i = mtx * actual;
    assert!(i.almost_equals(&Matrix4x4::<f32>::identity(), 1e-5));
}

#[test]
fn matrix4x4_invert_identity_test() {
    let mtx = Matrix4x4::<f32>::identity();
    let actual = mtx.invert();
    assert!(actual.is_some());
    let actual = actual.unwrap();
    assert!(actual.almost_equals(&Matrix4x4::<f32>::identity(), 1e-5));
}

#[test]
fn matrix4x4_invert_translation_test() {
    let mtx = Matrix4x4::create_translation_xyz(23.0f32, 42.0f32, 666.0f32);
    let actual = mtx.invert();
    assert!(actual.is_some());
    let actual = actual.unwrap();
    let i = mtx * actual;
    assert!(i.almost_equals(&Matrix4x4::<f32>::identity(), 1e-5));
}

#[test]
fn matrix4x4_invert_rotation_test() {
    let mtx = Matrix4x4::create_from_yaw_pitch_roll(3.0f32, 4.0f32, 5.0);
    let actual = mtx.invert();
    assert!(actual.is_some());
    let actual = actual.unwrap();
    let i = mtx * actual;
    assert!(i.almost_equals(&Matrix4x4::<f32>::identity(), 1e-5));
}

/** */
// A test for Invert (Matrix4x4)
#[test]
fn matrix4x4_invert_scale_test() {
    let mtx = Matrix4x4::create_scale_xyz(23.0f32, 42.0f32, -666.0);
    let actual = mtx.invert();
    assert!(actual.is_some());
    let actual = actual.unwrap();
    let i = mtx * actual;
    assert!(i.almost_equals(&Matrix4x4::<f32>::identity(), 1e-5));
}

// A test for Invert (Matrix4x4)
#[test]
fn matrix4x4_invert_projection_test() {
    let mtx = Matrix4x4::create_perspective_field_of_view(1.0f32, 1.333, 0.1, 666.0);
    let actual = mtx.invert();
    assert!(actual.is_some());
    let actual = actual.unwrap();

    let i = mtx * actual;
    assert!(i.almost_equals(&Matrix4x4::<f32>::identity(), 1e-5));
}

// A test for Invert (Matrix4x4)
#[test]
fn matrix4x4_invert_affine_test() {
    let mtx = 
        Matrix4x4::create_from_yaw_pitch_roll(3.0f32, 4.0f32, 5.0) *
        Matrix4x4::create_scale_xyz(23.0f32, 42.0f32, -666.0) *
        Matrix4x4::create_translation_xyz(17.0f32, 53.0f32, 89.0);
    let actual = mtx.invert();
    assert!(actual.is_some());
    let actual = actual.unwrap();

    let i = mtx * actual;
    assert!(i.almost_equals(&Matrix4x4::<f32>::identity(), 1e-5));
}

/** */
fn decompose_test(yaw: f32, pitch: f32, roll: f32, expected_translation: Vector3<f32>, expected_scales: Vector3<f32>) {
    let expected_rotation = Quaternion::new_from_yaw_pitch_roll(
        yaw.to_radians(), 
        pitch.to_radians(), 
        roll.to_radians());
    let m = Matrix4x4::create_scale(expected_scales)
        * Matrix4x4::create_from_quaternion(expected_rotation)
        * Matrix4x4::create_translation(expected_translation);

    let actual_result = Matrix4x4::decompose(&m);
    assert!(actual_result.is_some(), "Matrix4x4.Decompose did not return expected value.");
    let actual_result = actual_result.unwrap();
    let scales = actual_result.0;
    let rotation = actual_result.1;
    let translation = actual_result.2;

    let scale_is_zero_or_negative = expected_scales.x <= 0.0 || expected_scales.y <= 0.0 || expected_scales.z <= 0.0;
    if scale_is_zero_or_negative {
        assert!(equal_tolerance(expected_scales.x.abs(), scales.x.abs(), 0.001f32), "Matrix4x4.Decompose did not return expected value.");
        assert!(equal_tolerance(expected_scales.y.abs(), scales.y.abs(), 0.001f32), "Matrix4x4.Decompose did not return expected value.");
        assert!(equal_tolerance(expected_scales.z.abs(), scales.z.abs(), 0.001f32), "Matrix4x4.Decompose did not return expected value.");
    } else {
        assert!(equal_tolerance(expected_scales.x, scales.x, 0.001f32));
        assert!(equal_tolerance(expected_scales.y, scales.y, 0.001f32));
        assert!(equal_tolerance(expected_scales.z, scales.z, 0.001f32));
        
        assert!(equal(expected_rotation.x, rotation.x) || equal(expected_rotation.x, -rotation.x));
        assert!(equal(expected_rotation.y, rotation.y) || equal(expected_rotation.y, -rotation.y));
        assert!(equal(expected_rotation.z, rotation.z) || equal(expected_rotation.z, -rotation.z));
        assert!(equal(expected_rotation.w, rotation.w) || equal(expected_rotation.w, -rotation.w));
    }

    assert!(equal_tolerance(expected_translation.x, translation.x, 0.001f32));
    assert!(equal_tolerance(expected_translation.y, translation.y, 0.001f32));
    assert!(equal_tolerance(expected_translation.z, translation.z, 0.001f32));
}

// Various rotation decompose test.
#[test]
fn matrix4x4_decompose_test01() {
    decompose_test(10.0f32, 20.0f32, 30.0f32, Vector3::new(10.0f32, 20.0f32, 30.0), Vector3::new(2.0f32, 3.0f32, 4.0));
    let step: usize = 35;
    for yaw_angle in (-720..=720).step_by(step) {
        for pitch_angle in (-720..=720).step_by(step) {
            for roll_angle in (-720..=720).step_by(step) {
                decompose_test(yaw_angle as f32, pitch_angle as f32, roll_angle as f32, Vector3::new(10.0f32, 20.0f32, 30.0), Vector3::new(2.0f32, 3.0f32, 4.0));
            }
        }
    }
}

// Various scaled matrix decompose test.
#[test]
fn matrix4x4_decompose_test02() {
    decompose_test(10.0f32, 20.0f32, 30.0f32, Vector3::new(10.0f32, 20.0f32, 30.0), Vector3::new(2.0f32, 3.0f32, 4.0));

    // Various scales.
    decompose_test(0.0f32, 0.0f32, 0.0f32, Vector3::zero(), Vector3::new(1.0f32, 2.0f32, 3.0));
    decompose_test(0.0f32, 0.0f32, 0.0f32, Vector3::zero(), Vector3::new(1.0f32, 3.0f32, 2.0));
    decompose_test(0.0f32, 0.0f32, 0.0f32, Vector3::zero(), Vector3::new(2.0f32, 1.0f32, 3.0));
    decompose_test(0.0f32, 0.0f32, 0.0f32, Vector3::zero(), Vector3::new(2.0f32, 3.0f32, 1.0));
    decompose_test(0.0f32, 0.0f32, 0.0f32, Vector3::zero(), Vector3::new(3.0f32, 1.0f32, 2.0));
    decompose_test(0.0f32, 0.0f32, 0.0f32, Vector3::zero(), Vector3::new(3.0f32, 2.0f32, 1.0));

    decompose_test(0.0f32, 0.0f32, 0.0f32, Vector3::zero(), Vector3::new(-2.0f32, 1.0f32, 1.0));

    // Small scales.
    decompose_test(0f32, 0f32, 0f32, Vector3::zero(), Vector3::new(1e-4f32, 2e-4f32, 3e-4f32));
    decompose_test(0f32, 0f32, 0f32, Vector3::zero(), Vector3::new(1e-4f32, 3e-4f32, 2e-4f32));
    decompose_test(0f32, 0f32, 0f32, Vector3::zero(), Vector3::new(2e-4f32, 1e-4f32, 3e-4f32));
    decompose_test(0f32, 0f32, 0f32, Vector3::zero(), Vector3::new(2e-4f32, 3e-4f32, 1e-4f32));
    decompose_test(0f32, 0f32, 0f32, Vector3::zero(), Vector3::new(3e-4f32, 1e-4f32, 2e-4f32));
    decompose_test(0f32, 0f32, 0f32, Vector3::zero(), Vector3::new(3e-4f32, 2e-4f32, 1e-4f32));

    // Zero scales.
    decompose_test(0f32, 0f32, 0f32, Vector3::new(10f32, 20f32, 30f32), Vector3::new(0f32, 0f32, 0f32));
    decompose_test(0f32, 0f32, 0f32, Vector3::new(10f32, 20f32, 30f32), Vector3::new(1f32, 0f32, 0f32));
    decompose_test(0f32, 0f32, 0f32, Vector3::new(10f32, 20f32, 30f32), Vector3::new(0f32, 1f32, 0f32));
    decompose_test(0f32, 0f32, 0f32, Vector3::new(10f32, 20f32, 30f32), Vector3::new(0f32, 0f32, 1f32));
    decompose_test(0f32, 0f32, 0f32, Vector3::new(10f32, 20f32, 30f32), Vector3::new(0f32, 1f32, 1f32));
    decompose_test(0f32, 0f32, 0f32, Vector3::new(10f32, 20f32, 30f32), Vector3::new(1f32, 0f32, 1f32));
    decompose_test(0f32, 0f32, 0f32, Vector3::new(10f32, 20f32, 30f32), Vector3::new(1f32, 1f32, 0f32));

    // Negative scales.
    decompose_test(0f32, 0f32, 0f32, Vector3::new(10f32, 20f32, 30f32), Vector3::new(-1f32, -1f32, -1f32));
    decompose_test(0f32, 0f32, 0f32, Vector3::new(10f32, 20f32, 30f32), Vector3::new(1f32, -1f32, -1f32));
    decompose_test(0f32, 0f32, 0f32, Vector3::new(10f32, 20f32, 30f32), Vector3::new(-1f32, 1f32, -1f32));
    decompose_test(0f32, 0f32, 0f32, Vector3::new(10f32, 20f32, 30f32), Vector3::new(-1f32, -1f32, 1f32));
    decompose_test(0f32, 0f32, 0f32, Vector3::new(10f32, 20f32, 30f32), Vector3::new(-1f32, 1f32, 1f32));
    decompose_test(0f32, 0f32, 0f32, Vector3::new(10f32, 20f32, 30f32), Vector3::new(1f32, -1f32, 1f32));
    decompose_test(0f32, 0f32, 0f32, Vector3::new(10f32, 20f32, 30f32), Vector3::new(1f32, 1f32, -1f32));
}

fn decompose_scale_test(sx: f32, sy: f32, sz: f32) {
    let m = Matrix4x4::create_scale_xyz(sx, sy, sz);
    let expected_scales = Vector3::new(sx, sy, sz);
    let scales_rotation_translation = m.decompose();
    assert!(scales_rotation_translation.is_some(), "Matrix4x4.Decompose did not return expected value.");
    let (scales, rotation, translation) = scales_rotation_translation.unwrap();
    
    assert!(equal(expected_scales.x, scales.x));
    assert!(equal(expected_scales.y, scales.y));
    assert!(equal(expected_scales.z, scales.z));
    
    let identity = Quaternion::<f32>::identity();
    assert!(equal(identity.x, rotation.x) || equal(identity.x, -rotation.x));
    assert!(equal(identity.y, rotation.y) || equal(identity.y, -rotation.y));
    assert!(equal(identity.z, rotation.z) || equal(identity.z, -rotation.z));
    assert!(equal(identity.w, rotation.w) || equal(identity.w, -rotation.w));

    let zero = Vector3::<f32>::zero();
    assert!(equal(zero.x, translation.x));
    assert!(equal(zero.y, translation.y));
    assert!(equal(zero.z, translation.z));
}

// Tiny scale decompose test.
#[test]
fn matrix4x4_decompose_test03() {
    decompose_scale_test(1.0f32, 2e-4, 3e-4);
    decompose_scale_test(1.0f32, 3e-4, 2e-4);
    decompose_scale_test(2e-4f32, 1.0f32, 3e-4);
    decompose_scale_test(2e-4f32, 3e-4, 1.0);
    decompose_scale_test(3e-4f32, 1.0f32, 2e-4);
    decompose_scale_test(3e-4f32, 2e-4, 1.0);
}

#[test]
fn matrix4x4_extract_scale_test() {
    fn extract_scale_test(s: Vector3<f32>, r: Vector3<f32>) {
        let m = Matrix4x4::create_scale(s) * Matrix4x4::create_from_quaternion(Quaternion::new_from_euler_angles(r));
        assert!(m.extract_direct_scale().almost_equals(&s, 1e-5), "Failed to extract similar scale to input");
    }

    extract_scale_test(Vector3::new(1.0f32, 2.0f32, 1.0), Vector3::zero());
    extract_scale_test(Vector3::new(-1.0f32, 2.0f32, 1.0), Vector3::zero());
    extract_scale_test(Vector3::new(-1.0f32, 2.0f32, -1.0), Vector3::zero());

    extract_scale_test(Vector3::new(1.0f32, 2.0f32, 0.75), Vector3::unit_x());
    extract_scale_test(Vector3::new(1.0f32, 2.0f32, 0.75), Vector3::unit_y());
    extract_scale_test(Vector3::new(1.0f32, 2.0f32, 0.75), Vector3::unit_z());

    extract_scale_test(Vector3::new(1.0f32, 2.0f32, 0.75), -Vector3::unit_x());
    extract_scale_test(Vector3::new(1.0f32, 2.0f32, 0.75), -Vector3::unit_y());
    extract_scale_test(Vector3::new(1.0f32, 2.0f32, 0.75), -Vector3::unit_z());

    extract_scale_test(Vector3::new(-1.0f32, 2.0f32, 0.75), -Vector3::unit_x());
    extract_scale_test(Vector3::new(1.0f32, -2.0f32, -0.75), -Vector3::unit_y());
    extract_scale_test(Vector3::new(1.0f32, 2.0f32, -0.75), -Vector3::unit_z());

    // Note, for more complex rotations the extraction will not return the same scale
    // These scenarios could potentially be handled by figuring out which of the
    // axis are still in the same RH configuration, but that is a bit beyond the current scope
    // and would be better handled by a full decomposition.
    //extract_scale_test(Vector3::new(1.0f32, 2.0f32, 0.75), Vector3::new(0.5, 0.3, 1.75));
}

#[test]
fn matrix4x4_decompose_test04() {
    let matrix = generate_matrix_number_from_1_to_16();
    let decomp = matrix.decompose();
    assert!(decomp.is_none());
}

// Transform by quaternion test
#[test]
fn matrix4x4_transform_test() {
    let target = generate_matrix_number_from_1_to_16();
    let m = Matrix4x4::create_rotation_x(30.0f32.to_radians())
        * Matrix4x4::create_rotation_y(30.0f32.to_radians())
        * Matrix4x4::create_rotation_z(30.0f32.to_radians());
    let q = Quaternion::new_from_rotation_matrix(m);

    let expected = target * m;
    let actual = target.transform_quaternion(q);
    assert_eq!(expected, actual, "Matrix4x4::transform did not return the expected value.");
}

// A test for CreateRotationX (float)
#[test]
fn matrix4x4_create_rotation_x_test() {
    let radians = 30.0f32.to_radians();

    let mut expected = Matrix4x4::identity();
    expected.m11 = 1.0f32;
    expected.m22 = 0.8660254f32;
    expected.m23 = 0.5f32;
    expected.m32 = -0.5f32;
    expected.m33 = 0.8660254f32;
    expected.m44 = 1.0f32;

    let actual = Matrix4x4::create_rotation_x(radians);
    assert_eq!(expected, actual, "Matrix4x4::create_rotation_x did not return the expected value.");
}

// A test for CreateRotationX (float)
// CreateRotationX of zero degree
#[test]
fn matrix4x4_create_rotation_x_test1() {
    let radians = 0.0f32;

    let expected = Matrix4x4::identity();
    let actual = Matrix4x4::create_rotation_x(radians);
    assert_eq!(expected, actual, "Matrix4x4::create_rotation_x did not return the expected value.");
}

/** */
// A test for CreateRotationX (float, Vector3f)
#[test]
fn matrix4x4_create_rotation_x_center_test() {
    let radians = 30.0f32.to_radians();
    let center = Vector3::new(23.0f32, 42.0f32, 66.0f32);

    let rotate_around_zero = Matrix4x4::create_rotation_x_centered(radians, Vector3::zero());
    let rotate_around_zero_expected = Matrix4x4::create_rotation_x(radians);
    assert_eq!(rotate_around_zero, rotate_around_zero_expected);

    let rotate_around_center = Matrix4x4::create_rotation_x_centered(radians, center);
    let rotate_around_center_expected = 
        Matrix4x4::create_translation(-center) * 
        Matrix4x4::create_rotation_x(radians) * 
        Matrix4x4::create_translation(center);
    assert!(rotate_around_center.almost_equals(&rotate_around_center_expected, 1e-5));
}

// A test for CreateRotationY (float)
#[test]
fn matrix4x4_create_rotation_y_test() {
    let radians = 60.0f32.to_radians();

    let mut expected = Matrix4x4::<f32>::zero();
    expected.m11 = 0.49999997f32;
    expected.m13 = -0.866025448f32;
    expected.m22 = 1.0f32;
    expected.m31 = 0.866025448f32;
    expected.m33 = 0.49999997f32;
    expected.m44 = 1.0f32;

    let actual = Matrix4x4::create_rotation_y(radians);
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_rotation_y did not return the expected value.");
}

// A test for RotationY (float)
// CreateRotationY test for negative angle
#[test]
fn matrix4x4_create_rotation_y_test1() {
    let radians = -300.0f32.to_radians();

    let mut expected = Matrix4x4::<f32>::zero();
    expected.m11 = 0.49999997f32;
    expected.m13 = -0.866025448f32;
    expected.m22 = 1.0f32;
    expected.m31 = 0.866025448f32;
    expected.m33 = 0.49999997f32;
    expected.m44 = 1.0f32;

    let actual = Matrix4x4::create_rotation_y(radians);
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_rotation_y did not return the expected value.");
}

// A test for CreateRotationY (float, Vector3f)
#[test]
fn matrix4x4_create_rotation_y_center_test() {
    let radians = 30.0f32.to_radians();
    let center = Vector3::new(23.0f32, 42.0f32, 66.0);

    let rotate_around_zero = Matrix4x4::create_rotation_y_centered(radians, Vector3::zero());
    let rotate_around_zero_expected = Matrix4x4::create_rotation_y(radians);
    assert_eq!(rotate_around_zero, rotate_around_zero_expected);

    let rotate_around_center = Matrix4x4::create_rotation_y_centered(radians, center);
    let rotate_around_center_expected = 
        Matrix4x4::create_translation(-center) * 
        Matrix4x4::create_rotation_y(radians) *
        Matrix4x4::create_translation(center);
    assert!(rotate_around_center.almost_equals(&rotate_around_center_expected, 1e-5));
}

// A test for CreateFromAxisAngle(Vector3f,float)
#[test]
fn matrix4x4_create_from_axis_angle_test() {
    let radians = -30.0f32.to_radians();

    let expected = Matrix4x4::create_rotation_x(radians);
    let actual = Matrix4x4::create_from_axis_angle(Vector3::unit_x(), radians);
    assert!(expected.almost_equals(&actual, 1e-5));

    let expected = Matrix4x4::create_rotation_y(radians);
    let actual = Matrix4x4::create_from_axis_angle(Vector3::unit_y(), radians);
    assert!(expected.almost_equals(&actual, 1e-5));

    let expected = Matrix4x4::create_rotation_z(radians);
    let actual = Matrix4x4::create_from_axis_angle(Vector3::unit_z(), radians);
    assert!(expected.almost_equals(&actual, 1e-5));

    let expected = Matrix4x4::create_from_quaternion(Quaternion::new_from_axis_angle(Vector3::one().normalize(), radians));
    let actual = Matrix4x4::create_from_axis_angle(Vector3::one().normalize(), radians);
    assert!(expected.almost_equals(&actual, 1e-5));

    let rot_count = 16;
    for i in 0..rot_count {
        let latitude = (2.0 * PI) * (i as f32 / rot_count as f32);
        for j in 0..rot_count {
            let longitude = -FRAC_PI_2 + PI * (j as f32 / rot_count as f32);

            let m = Matrix4x4::create_rotation_z(longitude) * 
                Matrix4x4::create_rotation_y(latitude);
            let axis = Vector3::new(m.m11, m.m12, m.m13);
            for k in 0..rot_count {
                let rot = (2.0 * PI) * (k as f32 / rot_count as f32);
                let expected = Matrix4x4::create_from_quaternion(Quaternion::new_from_axis_angle(axis, rot));
                let actual = Matrix4x4::create_from_axis_angle(axis, rot);
                assert!(expected.almost_equals(&actual, 1e-5));
            }
        }
    }
}


/** */
#[test]
fn matrix4x4_create_from_yaw_pitch_roll_test1() {
    let yaw_angle = 30.0f32.to_radians();
    let pitch_angle = 40.0f32.to_radians();
    let roll_angle = 50.0f32.to_radians();

    let yaw = Matrix4x4::create_from_axis_angle(Vector3::unit_y(), yaw_angle);
    let pitch = Matrix4x4::create_from_axis_angle(Vector3::unit_x(), pitch_angle);
    let roll = Matrix4x4::create_from_axis_angle(Vector3::unit_z(), roll_angle);

    let expected = roll * pitch * yaw;
    let actual = Matrix4x4::create_from_yaw_pitch_roll(yaw_angle, pitch_angle, roll_angle);
    assert!(expected.almost_equals(&actual, 1e-5));
}

// Covers more numeric regions
#[test]
fn matrix4x4_create_from_yaw_pitch_roll_test2() {
    const STEP: usize = 35;

    for yaw_angle in (-720..=720).step_by(STEP) {
        for pitch_angle in (-720..=720).step_by(STEP) {
            for roll_angle in (-720..=720).step_by(STEP) {
                let yaw_rad = (yaw_angle as f32).to_radians();
                let pitch_rad = (pitch_angle as f32).to_radians();
                let roll_rad = (roll_angle as f32).to_radians();

                let yaw = Matrix4x4::create_from_axis_angle(Vector3::unit_y(), yaw_rad);
                let pitch = Matrix4x4::create_from_axis_angle(Vector3::unit_x(), pitch_rad);
                let roll = Matrix4x4::create_from_axis_angle(Vector3::unit_z(), roll_rad);

                let expected = roll * pitch * yaw;
                let actual = Matrix4x4::create_from_yaw_pitch_roll(yaw_rad, pitch_rad, roll_rad);
                assert!(expected.almost_equals(&actual, 1e-5));
            }
        }
    }
}

// Simple shadow test.
#[test]
fn matrix4x4_create_shadow_test01() {
    let light_dir = Vector3::unit_y();
    let plane = Plane::new(Vector3::unit_y(), 0.0f32);

    let expected = Matrix4x4::create_scale_xyz(1.0f32, 0.0f32, 1.0);

    let actual = Matrix4x4::create_shadow(light_dir, plane);
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_shadow did not returned expected value.");
}

#[test]
fn matrix4x4_create_shadow_test02() {
    let planes = [
        Plane::new_from_coordinates(0.0f32, 1.0f32, 0.0f32, 0.0),
        Plane::new_from_coordinates(1.0f32, 2.0f32, 3.0f32, 4.0),
        Plane::new_from_coordinates(5.0f32, 6.0f32, 7.0f32, 8.0),
        Plane::new_from_coordinates(-1.0f32, -2.0f32, -3.0f32, -4.0),
        Plane::new_from_coordinates(-5.0f32, -6.0f32, -7.0f32, -8.0),
    ];

    let points = [
        Vector3::new(1.0f32, 2.0f32, 3.0),
        Vector3::new(5.0f32, 6.0f32, 7.0),
        Vector3::new(8.0f32, 9.0f32, 10.0),
        Vector3::new(-1.0f32, -2.0f32, -3.0),
        Vector3::new(-5.0f32, -6.0f32, -7.0),
        Vector3::new(-8.0f32, -9.0f32, -10.0),
    ];

    for p in planes.iter() {
        let plane = p.normalize();

        let test_directions = [
            Vector3::new(-1.0f32, 1.0f32, 1.0f32),
            Vector3::new(0.0f32, 1.0f32, 1.0f32),
            Vector3::new(1.0f32, 1.0f32, 1.0f32),
            Vector3::new(-1.0f32, 0.0f32, 1.0f32),
            Vector3::new(0.0f32, 0.0f32, 1.0f32),
            Vector3::new(1.0f32, 0.0f32, 1.0f32),
            Vector3::new(-1.0f32,-1.0f32, 1.0f32),
            Vector3::new(0.0f32,-1.0f32, 1.0f32),
            Vector3::new(1.0f32,-1.0f32, 1.0f32),

            Vector3::new(-1.0f32, 1.0f32, 0.0f32),
            Vector3::new(0.0f32, 1.0f32, 0.0f32),
            Vector3::new(1.0f32, 1.0f32, 0.0f32),
            Vector3::new(-1.0f32, 0.0f32, 0.0f32),
            Vector3::new(0.0f32, 0.0f32, 0.0f32),
            Vector3::new(1.0f32, 0.0f32, 0.0f32),
            Vector3::new(-1.0f32,-1.0f32, 0.0f32),
            Vector3::new(0.0f32,-1.0f32, 0.0f32),
            Vector3::new(1.0f32,-1.0f32, 0.0f32),

            Vector3::new(-1.0f32, 1.0f32,-1.0f32),
            Vector3::new(0.0f32, 1.0f32,-1.0f32),
            Vector3::new(1.0f32, 1.0f32,-1.0f32),
            Vector3::new(-1.0f32, 0.0f32,-1.0f32),
            Vector3::new(0.0f32, 0.0f32,-1.0f32),
            Vector3::new(1.0f32, 0.0f32,-1.0f32),
            Vector3::new(-1.0f32,-1.0f32,-1.0f32),
            Vector3::new(0.0f32,-1.0f32,-1.0f32),
            Vector3::new(1.0f32,-1.0f32,-1.0f32),
        ];

        for light_dir_info in test_directions.iter() {
            if light_dir_info.length() < 0.1 { continue; }
            let light_dir = light_dir_info.normalize();
            if plane.dot_normal(light_dir) < 0.1 { continue; }
        
            let m = Matrix4x4::create_shadow(light_dir, plane);
            let pp = plane.normal * (-plane.d);
        
            for point in points.iter() {
                let v4 = point.transform_to_vector4(m);
                let sp = Vector3::new(v4.x, v4.y, v4.z) / v4.w;
        
                // Make sure transformed position is on the plane.
                let v = sp - pp;
                let d = v.dot(plane.normal);
                assert!(equal(d, 0.0f32), "Matrix4x4.CreateShadow did not provide expected value.");
        
                // make sure direction between transformed position and original position are same as light direction.
                if (*point - pp).dot(plane.normal) > 0.0001 {
                    let dir = (*point - sp).normalize();
                    assert!(equal(dir.x, light_dir.x), "Matrix4x4.CreateShadow did not provide expected value.");
                    assert!(equal(dir.y, light_dir.y), "Matrix4x4.CreateShadow did not provide expected value.");
                    assert!(equal(dir.z, light_dir.z), "Matrix4x4.CreateShadow did not provide expected value.");
                }
            }
        }
    }
}

fn create_reflection_test(plane: Plane<f32>, expected: Matrix4x4<f32>) {
    let actual = Matrix4x4::create_reflection(plane);
    assert!(actual.almost_equals(&expected, 1e-5), "Matrix4x4::create_reflection did not return expected value.");
    assert!(actual.is_reflection(), "Matrix4x4::is_reflection did not return expected value.");
}

#[test]
fn matrix4x4_create_reflection_test01() {
    // XY plane.
    create_reflection_test(
        Plane::new(Vector3::unit_z(), 0.0f32),
        Matrix4x4::create_scale_xyz(1.0f32, 1.0f32, -1.0),
    );
    // XZ plane.
    create_reflection_test(
        Plane::new(Vector3::unit_y(), 0.0f32),
        Matrix4x4::create_scale_xyz(1.0f32, -1.0f32, 1.0),
    );
    // YZ plane.
    create_reflection_test(
        Plane::new(Vector3::unit_x(), 0.0f32),
        Matrix4x4::create_scale_xyz(-1.0f32, 1.0f32, 1.0),
    );

    // Complex cases.
    let planes = [
        Plane::new_from_coordinates(0.0f32, 1.0f32, 0.0f32, 0.0),
        Plane::new_from_coordinates(1.0f32, 2.0f32, 3.0f32, 4.0),
        Plane::new_from_coordinates(5.0f32, 6.0f32, 7.0f32, 8.0),
        Plane::new_from_coordinates(-1.0f32, -2.0f32, -3.0f32, -4.0),
        Plane::new_from_coordinates(-5.0f32, -6.0f32, -7.0f32, -8.0),
    ];

    let points = [
        Vector3::new(1.0f32, 2.0f32, 3.0),
        Vector3::new(5.0f32, 6.0f32, 7.0),
        Vector3::new(-1.0f32, -2.0f32, -3.0),
        Vector3::new(-5.0f32, -6.0f32, -7.0),
    ];

    for plane in planes.iter() {
        let plane = plane.normalize();
        let m = Matrix4x4::create_reflection(plane);
        let pp = plane.normal * (-plane.d); // Position on the plane.

        for point in points.iter() {
            let rp = point.transform(m);

            // Manually compute reflection point and compare results.
            let v = *point - pp;
            let d = v.dot(plane.normal);
            let vp = *point - plane.normal * 2.0f32 * d;
            assert!(equal(rp.x, vp.x), "Matrix4x4.Reflection did not provide expected value.");
            assert!(equal(rp.y, vp.y), "Matrix4x4.Reflection did not provide expected value.");
            assert!(equal(rp.z, vp.z), "Matrix4x4.Reflection did not provide expected value.");
        }
    }
}

// A test for CreateRotationZ (float)
#[test]
fn matrix4x4_create_rotation_z_test() {
    let radians = 50.0f32.to_radians();

    let mut expected = Matrix4x4::<f32>::zero();
    expected.m11 = 0.642787635f32;
    expected.m12 = 0.766044438;
    expected.m21 = -0.766044438;
    expected.m22 = 0.642787635;
    expected.m33 = 1.0;
    expected.m44 = 1.0;

    let actual = Matrix4x4::create_rotation_z(radians);
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4.CreateRotationZ did not return the expected value.");
}

// A test for CreateRotationZ (float, Vector3f)
#[test]
fn matrix4x4_create_rotation_z_center_test() {
    let radians = 30.0f32.to_radians();
    let center = Vector3::new(23.0f32, 42.0f32, 66.0);

    let rotate_around_zero = Matrix4x4::create_rotation_z_centered(radians, Vector3::<f32>::zero());
    let rotate_around_zero_expected = Matrix4x4::create_rotation_z(radians);
    assert!(rotate_around_zero.almost_equals(&rotate_around_zero_expected, 1e-5));

    let rotate_around_center = Matrix4x4::create_rotation_z_centered(radians, center);
    let rotate_around_center_expected = Matrix4x4::create_translation(-center)
        * Matrix4x4::create_rotation_z(radians)
        * Matrix4x4::create_translation(center);
    assert!(rotate_around_center.almost_equals(&rotate_around_center_expected, 1e-5));
}

// A test for CreateLookAt (Vector3f, Vector3f, Vector3f)
#[test]
fn matrix4x4_create_look_at_test() {
    let camera_position = Vector3::new(10.0f32, 20.0f32, 30.0);
    let camera_target = Vector3::new(3.0f32, 2.0f32, -4.0);
    let camera_up_vector = Vector3::new(0.0f32, 1.0f32, 0.0);

    let mut expected = Matrix4x4::<f32>::zero();
    expected.m11 = 0.979457;
    expected.m12 = -0.0928267762;
    expected.m13 = 0.179017;

    expected.m21 = 0.0;
    expected.m22 = 0.8877481;
    expected.m23 = 0.460329473;

    expected.m31 = -0.201652914;
    expected.m32 = -0.450872928;
    expected.m33 = 0.8695112;

    expected.m41 = -3.74498272;
    expected.m42 = -3.30050683;
    expected.m43 = -37.0820961;
    expected.m44 = 1.0;

    let actual = Matrix4x4::create_look_at(camera_position, camera_target, camera_up_vector);
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4.CreateLookAt did not return the expected value.");
}

#[test]
fn matrix4x4_create_world_test() {
    let object_position = Vector3::new(10.0f32, 20.0f32, 30.0);
    let object_forward_direction = Vector3::new(3.0f32, 2.0f32, -4.0);
    let object_up_vector = Vector3::new(0.0f32, 1.0f32, 0.0);

    let expected = Matrix4x4::<f32> {
        m11: 0.799999952f32,
        m12: 0f32,
        m13: 0.599999964f32,
        m14: 0f32,
        m21: -0.2228344f32,
        m22: 0.928476632f32,
        m23: 0.297112525f32,
        m24: 0f32,
        m31: -0.557086f32,
        m32: -0.371390671f32,
        m33: 0.742781341f32,
        m34: 0f32,
        m41: 10f32,
        m42: 20f32,
        m43: 30f32,
        m44: 1.0f32,
    };
    let actual = Matrix4x4::create_world(object_position, object_forward_direction, object_up_vector);

    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4.CreateWorld did not return the expected value.");

    assert_eq!(object_position, actual.translation());
    assert!(object_up_vector.normalize().dot(Vector3::new(actual.m21, actual.m22, actual.m23)) > 0.0f32);
    assert!(object_forward_direction.normalize().dot(Vector3::new(-actual.m31, -actual.m32, -actual.m33)) > 0.999f32);
}

#[test]
fn matrix4x4_create_ortho_test() {
    let width = 100.0f32;
    let height = 200.0f32;
    let z_near_plane = 1.5f32;
    let z_far_plane = 1000.0f32;

    let mut expected = Matrix4x4::zero();
    expected.m11 = 0.02f32;
    expected.m22 = 0.01f32;
    expected.m33 = -0.00100150227f32;
    expected.m43 = -0.00150225335f32;
    expected.m44 = 1.0f32;

    let actual = Matrix4x4::create_orthographic(width, height, z_near_plane, z_far_plane);

    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4.CreateOrtho did not return the expected value.");
}

#[test]
fn matrix4x4_create_ortho_off_center_test() {
    let left = 10.0f32;
    let right = 90.0f32;
    let bottom = 20.0f32;
    let top = 180.0f32;
    let z_near_plane = 1.5f32;
    let z_far_plane = 1000.0f32;

    let expected = Matrix4x4 {
        m11: 0.025f32,
        m12: 0.0,
        m13: 0.0,
        m14: 0.0,
        m21: 0.0,
        m22: 0.0125,
        m23: 0.0,
        m24: 0.0,
        m31: 0.0,
        m32: 0.0,
        m33: -0.00100150227,
        m34: 0.0,
        m41: -1.25,
        m42: -1.25,
        m43: -0.00150225335,
        m44: 1.0,
    };

    let actual = Matrix4x4::create_orthographic_off_center(left, right, bottom, top, z_near_plane, z_far_plane);
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_ortho_off_center did not return the expected value.");
}

#[test]
fn matrix4x4_create_perspective_test() {
    let width = 100.0f32;
    let height = 200.0f32;
    let z_near_plane = 1.5f32;
    let z_far_plane = 1000.0f32;

    let expected = Matrix4x4 {
        m11: 0.03f32,
        m12: 0.0,
        m13: 0.0,
        m14: 0.0,
        m21: 0.0,
        m22: 0.015,
        m23: 0.0,
        m24: 0.0,
        m31: 0.0,
        m32: 0.0,
        m33: -1.00150228,
        m34: -1.0,
        m41: 0.0,
        m42: 0.0,
        m43: -1.50225341,
        m44: 0.0,
    };

    let actual = Matrix4x4::create_perspective(width, height, z_near_plane, z_far_plane);
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_perspective did not return the expected value.");
}

// A test for CreatePerspective (float, float, float, float)
// CreatePerspective test where znear = zfar
#[test]
#[should_panic]
fn matrix4x4_create_perspective_test_1() {
    let width = 100.0f32;
    let height = 200.0f32;
    let z_near_plane = 0.0f32;
    let z_far_plane = 0.0f32;

    let p = Matrix4x4::create_perspective(width, height, z_near_plane, z_far_plane);
    print!("{:?}", p);
}

// A test for CreatePerspective (float, float, float, float)
// CreatePerspective test where near plane is negative value
#[test]
#[should_panic]
fn matrix4x4_create_perspective_test_2() {
    let _ = Matrix4x4::create_perspective(10.0f32, 10.0f32, -10.0f32, 10.0);
}

// A test for CreatePerspective (float, float, float, float)
// CreatePerspective test where far plane is negative value
#[test]
#[should_panic]
fn matrix4x4_create_perspective_test_3() {
    let _ = Matrix4x4::create_perspective(10.0f32, 10.0f32, 10.0f32, -10.0);
}

// A test for CreatePerspective (float, float, float, float)
// CreatePerspective test where near plane is beyond far plane
#[test]
#[should_panic]
fn matrix4x4_create_perspective_test_4() {
    let _ = Matrix4x4::create_perspective(10.0f32, 10.0f32, 10.0f32, 1.0);
}

// A test for CreatePerspectiveFieldOfView (float, float, float, float)
#[test]
fn matrix4x4_create_perspective_field_of_view_test() {
    let field_of_view = 30.0f32.to_radians();
    let aspect_ratio = 1280.0f32 / 720.0f32;
    let z_near_plane = 1.5f32;
    let z_far_plane = 1000.0f32;

    let mut expected = Matrix4x4::<f32>::zero();
    expected.m11 = 2.09927845f32;
    expected.m22 = 3.73205066f32;
    expected.m33 = -1.00150228f32;
    expected.m34 = -1.0f32;
    expected.m43 = -1.50225341f32;

    let actual = Matrix4x4::create_perspective_field_of_view(field_of_view, aspect_ratio, z_near_plane, z_far_plane);
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_perspective_field_of_view did not return the expected value.");
}

// A test for CreatePerspectiveFieldOfView (float, float, float, float)
// CreatePerspectiveFieldOfView test where fieldOfView is negative value.
#[test]
#[should_panic]
fn matrix4x4_create_perspective_field_of_view_test1() {
    let _ = Matrix4x4::create_perspective_field_of_view(-1.0f32, 1.0f32, 1.0f32, 10.0);
}

// A test for CreatePerspectiveFieldOfView (float, float, float, float)
// CreatePerspectiveFieldOfView test where fieldOfView is more than pi.
#[test]
#[should_panic]
fn matrix4x4_create_perspective_field_of_view_test2() {
    let _ = Matrix4x4::create_perspective_field_of_view(PI + 0.01, 1.0f32, 1.0f32, 10.0);
}

// A test for CreatePerspectiveFieldOfView (float, float, float, float)
// CreatePerspectiveFieldOfView test where nearPlaneDistance is negative value.
#[test]
#[should_panic]
fn matrix4x4_create_perspective_field_of_view_test3() {
    let _ = Matrix4x4::create_perspective_field_of_view(FRAC_PI_4, 1.0f32, -1.0f32, 10.0);
}

// A test for CreatePerspectiveFieldOfView (float, float, float, float)
// CreatePerspectiveFieldOfView test where farPlaneDistance is negative value.
#[test]
#[should_panic]
fn matrix4x4_create_perspective_field_of_view_test4() {
    let _ = Matrix4x4::create_perspective_field_of_view(FRAC_PI_4, 1.0f32, 1.0f32, -10.0);
}

// A test for CreatePerspectiveFieldOfView (float, float, float, float)
// CreatePerspectiveFieldOfView test where nearPlaneDistance is larger than farPlaneDistance.
#[test]
#[should_panic]
fn matrix4x4_create_perspective_field_of_view_test5() {
    let _ = Matrix4x4::create_perspective_field_of_view(FRAC_PI_4, 1.0f32, 10.0f32, 1.0);
}

// A test for CreatePerspectiveOffCenter (float, float, float, float, float, float)
#[test]
fn matrix4x4_create_perspective_off_center_test() {
    let left = 10.0f32;
    let right = 90.0f32;
    let bottom = 20.0f32;
    let top = 180.0f32;
    let z_near_plane = 1.5f32;
    let z_far_plane = 1000.0f32;

    let mut expected = Matrix4x4::<f32>::zero();
    expected.m11 = 0.0375f32;
    expected.m22 = 0.01875f32;
    expected.m31 = 1.25f32;
    expected.m32 = 1.25f32;
    expected.m33 = -1.00150228f32;
    expected.m34 = -1.0f32;
    expected.m43 = -1.50225341f32;

    let actual = Matrix4x4::create_perspective_off_center(left, right, bottom, top, z_near_plane, z_far_plane);
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4.CreatePerspectiveOffCenter did not return the expected value.");
}

// A test for CreatePerspectiveOffCenter (float, float, float, float, float, float)
// CreatePerspectiveOffCenter test where nearPlaneDistance is negative.
#[test]
#[should_panic]
fn matrix4x4_create_perspective_off_center_test1() {
    let left = 10.0f32; let right = 90.0f32; let bottom = 20.0f32; let top = 180.0f32;
    let _ = Matrix4x4::create_perspective_off_center(left, right, bottom, top, -1f32, 10f32);
}

// A test for CreatePerspectiveOffCenter (float, float, float, float, float, float)
// CreatePerspectiveOffCenter test where farPlaneDistance is negative.
#[test]
#[should_panic]
fn matrix4x4_create_perspective_off_center_test2() {
    let left = 10.0f32; let right = 90.0f32; let bottom = 20.0f32; let top = 180.0f32;
    let _ = Matrix4x4::create_perspective_off_center(left, right, bottom, top, 1f32, -10f32);
}

// A test for CreatePerspectiveOffCenter (float, float, float, float, float, float)
// CreatePerspectiveOffCenter test where test where nearPlaneDistance is larger than farPlaneDistance.
#[test]
#[should_panic]
fn matrix4x4_create_perspective_off_center_test3() {
    let left = 10.0f32; let right = 90.0f32; let bottom = 20.0f32; let top = 180.0f32;
    let _ = Matrix4x4::create_perspective_off_center(left, right, bottom, top, 10f32, 1f32);
}

// A test for Invert (Matrix4x4)
// Non invertible matrix - determinant is zero - singular matrix
#[test]
fn matrix4x4_invert_test1() {
    let mut a = Matrix4x4::<f32>::zero();
    a.m11 = 1.0f32;
    a.m12 = 2.0;
    a.m13 = 3.0;
    a.m14 = 4.0;
    a.m21 = 5.0;
    a.m22 = 6.0;
    a.m23 = 7.0;
    a.m24 = 8.0;
    a.m31 = 9.0;
    a.m32 = 10.0;
    a.m33 = 11.0;
    a.m34 = 12.0;
    a.m41 = 13.0;
    a.m42 = 14.0;
    a.m43 = 15.0;
    a.m44 = 16.0;

    let det_a = a.get_determinant();
    assert!(equal(det_a, 0.0f32), "Matrix4x4.Invert did not return the expected value.");
    assert_eq!(a.invert(), None);
}

#[test]
fn matrix4x4_lerp_test() {
    let mut a = Matrix4x4::<f32>::zero();
    a.m11 = 11.0f32;
    a.m12 = 12.0f32;
    a.m13 = 13.0f32;
    a.m14 = 14.0f32;
    a.m21 = 21.0f32;
    a.m22 = 22.0f32;
    a.m23 = 23.0f32;
    a.m24 = 24.0f32;
    a.m31 = 31.0f32;
    a.m32 = 32.0f32;
    a.m33 = 33.0f32;
    a.m34 = 34.0f32;
    a.m41 = 41.0f32;
    a.m42 = 42.0f32;
    a.m43 = 43.0f32;
    a.m44 = 44.0f32;

    let b = generate_matrix_number_from_1_to_16();
    let t = 0.5;

    let mut expected = Matrix4x4::<f32>::zero();
    expected.m11 = a.m11 + (b.m11 - a.m11) * t;
    expected.m12 = a.m12 + (b.m12 - a.m12) * t;
    expected.m13 = a.m13 + (b.m13 - a.m13) * t;
    expected.m14 = a.m14 + (b.m14 - a.m14) * t;

    expected.m21 = a.m21 + (b.m21 - a.m21) * t;
    expected.m22 = a.m22 + (b.m22 - a.m22) * t;
    expected.m23 = a.m23 + (b.m23 - a.m23) * t;
    expected.m24 = a.m24 + (b.m24 - a.m24) * t;

    expected.m31 = a.m31 + (b.m31 - a.m31) * t;
    expected.m32 = a.m32 + (b.m32 - a.m32) * t;
    expected.m33 = a.m33 + (b.m33 - a.m33) * t;
    expected.m34 = a.m34 + (b.m34 - a.m34) * t;

    expected.m41 = a.m41 + (b.m41 - a.m41) * t;
    expected.m42 = a.m42 + (b.m42 - a.m42) * t;
    expected.m43 = a.m43 + (b.m43 - a.m43) * t;
    expected.m44 = a.m44 + (b.m44 - a.m44) * t;

    let actual = a.lerp(b, t);
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4.Lerp did not return the expected value.");
}

#[test]
fn matrix4x4_unary_negation_test() {
    let a = generate_matrix_number_from_1_to_16();

    let expected = Matrix4x4::<f32> {
        m11: -1.0,
        m12: -2.0,
        m13: -3.0,
        m14: -4.0,
        m21: -5.0,
        m22: -6.0,
        m23: -7.0,
        m24: -8.0,
        m31: -9.0,
        m32: -10.0,
        m33: -11.0,
        m34: -12.0,
        m41: -13.0,
        m42: -14.0,
        m43: -15.0,
        m44: -16.0,
    };

    let actual = -a;
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4.operator - did not return the expected value.");
}

#[test]
fn matrix4x4_subtraction_test() {
    let a = generate_matrix_number_from_1_to_16();
    let b = generate_matrix_number_from_1_to_16();
    let expected = Matrix4x4::<f32>::zero();

    let actual = a - b;
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4.operator - did not return the expected value.");
}

#[test]
fn matrix4x4_multiply_test1() {
    let a = generate_matrix_number_from_1_to_16();
    let b = generate_matrix_number_from_1_to_16();

    let mut expected = Matrix4x4::zero();
    expected.m11 = a.m11 * b.m11 + a.m12 * b.m21 + a.m13 * b.m31 + a.m14 * b.m41;
    expected.m12 = a.m11 * b.m12 + a.m12 * b.m22 + a.m13 * b.m32 + a.m14 * b.m42;
    expected.m13 = a.m11 * b.m13 + a.m12 * b.m23 + a.m13 * b.m33 + a.m14 * b.m43;
    expected.m14 = a.m11 * b.m14 + a.m12 * b.m24 + a.m13 * b.m34 + a.m14 * b.m44;

    expected.m21 = a.m21 * b.m11 + a.m22 * b.m21 + a.m23 * b.m31 + a.m24 * b.m41;
    expected.m22 = a.m21 * b.m12 + a.m22 * b.m22 + a.m23 * b.m32 + a.m24 * b.m42;
    expected.m23 = a.m21 * b.m13 + a.m22 * b.m23 + a.m23 * b.m33 + a.m24 * b.m43;
    expected.m24 = a.m21 * b.m14 + a.m22 * b.m24 + a.m23 * b.m34 + a.m24 * b.m44;

    expected.m31 = a.m31 * b.m11 + a.m32 * b.m21 + a.m33 * b.m31 + a.m34 * b.m41;
    expected.m32 = a.m31 * b.m12 + a.m32 * b.m22 + a.m33 * b.m32 + a.m34 * b.m42;
    expected.m33 = a.m31 * b.m13 + a.m32 * b.m23 + a.m33 * b.m33 + a.m34 * b.m43;
    expected.m34 = a.m31 * b.m14 + a.m32 * b.m24 + a.m33 * b.m34 + a.m34 * b.m44;

    expected.m41 = a.m41 * b.m11 + a.m42 * b.m21 + a.m43 * b.m31 + a.m44 * b.m41;
    expected.m42 = a.m41 * b.m12 + a.m42 * b.m22 + a.m43 * b.m32 + a.m44 * b.m42;
    expected.m43 = a.m41 * b.m13 + a.m42 * b.m23 + a.m43 * b.m33 + a.m44 * b.m43;
    expected.m44 = a.m41 * b.m14 + a.m42 * b.m24 + a.m43 * b.m34 + a.m44 * b.m44;

    let actual = a * b;
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4.operator * did not return the expected value.");
}

#[test]
fn matrix4x4_multiply_test_4() {
    let mut a = Matrix4x4::<f32>::zero();
    a.m11 = 1.0f32;
    a.m12 = 2.0;
    a.m13 = 3.0;
    a.m14 = 4.0;
    a.m21 = 5.0;
    a.m22 = -6.0;
    a.m23 = 7.0;
    a.m24 = -8.0;
    a.m31 = 9.0;
    a.m32 = 10.0;
    a.m33 = 11.0;
    a.m34 = 12.0;
    a.m41 = 13.0;
    a.m42 = -14.0;
    a.m43 = 15.0;
    a.m44 = -16.0;

    let b = Matrix4x4::<f32>::identity();

    let expected = a;
    let actual = a * b;

    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4.operator * did not return the expected value.");
}

#[test]
fn matrix4x4_addition_test() {
    let a = generate_matrix_number_from_1_to_16();
    let b = generate_matrix_number_from_1_to_16();

    let mut expected = Matrix4x4::<f32>::zero();
    expected.m11 = a.m11 + b.m11;
    expected.m12 = a.m12 + b.m12;
    expected.m13 = a.m13 + b.m13;
    expected.m14 = a.m14 + b.m14;
    expected.m21 = a.m21 + b.m21;
    expected.m22 = a.m22 + b.m22;
    expected.m23 = a.m23 + b.m23;
    expected.m24 = a.m24 + b.m24;
    expected.m31 = a.m31 + b.m31;
    expected.m32 = a.m32 + b.m32;
    expected.m33 = a.m33 + b.m33;
    expected.m34 = a.m34 + b.m34;
    expected.m41 = a.m41 + b.m41;
    expected.m42 = a.m42 + b.m42;
    expected.m43 = a.m43 + b.m43;
    expected.m44 = a.m44 + b.m44;

    let actual = a + b;

    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4.operator + did not return the expected value.");
}

#[test]
fn matrix4x4_transpose_test() {
    let a = generate_matrix_number_from_1_to_16();

    let mut expected = Matrix4x4::<f32>::zero();
    expected.m11 = a.m11;
    expected.m12 = a.m21;
    expected.m13 = a.m31;
    expected.m14 = a.m41;
    expected.m21 = a.m12;
    expected.m22 = a.m22;
    expected.m23 = a.m32;
    expected.m24 = a.m42;
    expected.m31 = a.m13;
    expected.m32 = a.m23;
    expected.m33 = a.m33;
    expected.m34 = a.m43;
    expected.m41 = a.m14;
    expected.m42 = a.m24;
    expected.m43 = a.m34;
    expected.m44 = a.m44;

    let actual = a.transpose();
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::transpose did not return the expected value.");
}

#[test]
fn matrix4x4_transpose_test1() {
    let a = Matrix4x4::<f32>::identity();
    let expected = Matrix4x4::<f32>::identity();

    let actual = a.transpose();
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::transpose did not return the expected value.");
}

#[test]
fn matrix4x4_from_quaternion_test1() {
    let axis = Vector3::new(1.0f32, 2.0f32, 3.0).normalize();
    let q = Quaternion::new_from_axis_angle(axis, 30.0f32.to_radians());

    let mut expected = Matrix4x4::<f32>::zero();
    expected.m11 = 0.875595033f32;
    expected.m12 = 0.420031041;
    expected.m13 = -0.2385524;
    expected.m14 = 0.0;

    expected.m21 = -0.38175258;
    expected.m22 = 0.904303849;
    expected.m23 = 0.1910483;
    expected.m24 = 0.0;

    expected.m31 = 0.295970082;
    expected.m32 = -0.07621294;
    expected.m33 = 0.952151954;
    expected.m34 = 0.0;

    expected.m41 = 0.0;
    expected.m42 = 0.0;
    expected.m43 = 0.0;
    expected.m44 = 1.0;

    let actual = Matrix4x4::create_from_quaternion(q);
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::from(Quaternion) did not return the expected value.");
}


// A test for FromQuaternion (Matrix4x4)
// Convert X axis rotation matrix
#[test]
fn matrix4x4_from_quaternion_test2() {
    for angle in (0..720).step_by(10) {
        let quat = Quaternion::new_from_axis_angle(Vector3::unit_x(), angle as f32);

        let expected = Matrix4x4::create_rotation_x(angle as f32);
        let actual = Matrix4x4::create_from_quaternion(quat);
        assert!(expected.almost_equals(&actual, 1e-5), "Quaternion::create_from_quaternion did not return the expected value.");

        // make sure convert back to quaternion is same as we passed quaternion.
        let q2 = Quaternion::new_from_rotation_matrix(actual);
        assert!(equal(quat.x, q2.x) || equal(quat.x, -q2.x));
        assert!(equal(quat.y, q2.y) || equal(quat.y, -q2.y));
        assert!(equal(quat.z, q2.z) || equal(quat.z, -q2.z));
        assert!(equal(quat.w, q2.w) || equal(quat.w, -q2.w));
    }
}

// A test for FromQuaternion (Matrix4x4)
// Convert Y axis rotation matrix
#[test]
fn matrix4x4_from_quaternion_test3() {
    for angle in (0..720).step_by(10) {
        let quat = Quaternion::new_from_axis_angle(Vector3::unit_y(), angle as f32);

        let expected = Matrix4x4::create_rotation_y(angle as f32);
        let actual = Matrix4x4::create_from_quaternion(quat);
        assert!(expected.almost_equals(&actual, 1e-5), "Quaternion::create_from_quaternion did not return the expected value.");

        // make sure convert back to quaternion is same as we passed quaternion.
        let q2 = Quaternion::new_from_rotation_matrix(actual);
        assert!(equal(quat.x, q2.x) || equal(quat.x, -q2.x));
        assert!(equal(quat.y, q2.y) || equal(quat.y, -q2.y));
        assert!(equal(quat.z, q2.z) || equal(quat.z, -q2.z));
        assert!(equal(quat.w, q2.w) || equal(quat.w, -q2.w));
    }
}

// A test for FromQuaternion (Matrix4x4)
// Convert Z axis rotation matrix
#[test]
fn matrix4x4_from_quaternion_test4() {
    for angle in (0..720).step_by(10) {
        let quat = Quaternion::new_from_axis_angle(Vector3::unit_z(), angle as f32);

        let expected = Matrix4x4::create_rotation_z(angle as f32);
        let actual = Matrix4x4::create_from_quaternion(quat);
        assert!(expected.almost_equals(&actual, 1e-5), "Quaternion::create_from_quaternion did not return the expected value.");

        // make sure convert back to quaternion is same as we passed quaternion.
        let q2 = Quaternion::new_from_rotation_matrix(actual);
        assert!(equal(quat.x, q2.x) || equal(quat.x, -q2.x));
        assert!(equal(quat.y, q2.y) || equal(quat.y, -q2.y));
        assert!(equal(quat.z, q2.z) || equal(quat.z, -q2.z));
        assert!(equal(quat.w, q2.w) || equal(quat.w, -q2.w));
    }
}

// A test for FromQuaternion (Matrix4x4)
// Convert XYZ axis rotation matrix
#[test]
fn matrix4x4_from_quaternion_test5() {
    for angle in (0..720).step_by(10) {
        let quat = Quaternion::new_from_axis_angle(Vector3::unit_z(), angle as f32)
            * Quaternion::new_from_axis_angle(Vector3::unit_y(), angle as f32)
            * Quaternion::new_from_axis_angle(Vector3::unit_x(), angle as f32);

        let expected =
            Matrix4x4::create_rotation_x(angle as f32)
            * Matrix4x4::create_rotation_y(angle as f32)
            * Matrix4x4::create_rotation_z(angle as f32);
        let actual = Matrix4x4::create_from_quaternion(quat);
        assert!(expected.almost_equals(&actual, 1e-5), "Quaternion.FromQuaternion did not return the expected value.");

        // make sure convert back to quaternion is same as we passed quaternion.
        let q2 = Quaternion::new_from_rotation_matrix(actual);
        assert!(equal(quat.x, q2.x) || equal(quat.x, -q2.x));
        assert!(equal(quat.y, q2.y) || equal(quat.y, -q2.y));
        assert!(equal(quat.z, q2.z) || equal(quat.z, -q2.z));
        assert!(equal(quat.w, q2.w) || equal(quat.w, -q2.w));
    }
}

// #[test]
// fn matrix4x4_to_string_test() {
//     let mut a = Matrix4x4::zero();
//     a.m11 = 11.0;
//     a.m12 = -12.0;
//     a.m13 = -13.3;
//     a.m14 = 14.4;
//     a.m21 = 21.0;
//     a.m22 = 22.0;
//     a.m23 = 23.0;
//     a.m24 = 24.0;
//     a.m31 = 31.0;
//     a.m32 = 32.0;
//     a.m33 = 33.0;
//     a.m34 = 34.0;
//     a.m41 = 41.0;
//     a.m42 = 42.0;
//     a.m43 = 43.0;
//     a.m44 = 44.0;

//     let expected = format!(
//         "{{ {{M11:{:.1} M12:{:.1} M13:{:.1} M14:{:.1}}} {{M21:{:.1} M22:{:.1} M23:{:.1} M24:{:.1}}} {{M31:{:.1} M32:{:.1} M33:{:.1} M34:{:.1}}} {{M41:{:.1} M42:{:.1} M43:{:.1} M44:{:.1}}} }}",
//         11.0f32, -12.0f32, -13.3, 14.4,
//         21.0f32, 22.0f32, 23.0f32, 24.0,
//         31.0f32, 32.0f32, 33.0f32, 34.0,
//         41.0f32, 42.0f32, 43.0f32, 44.0
//     );

//     let actual = a.to_string();
//     assert_eq!(expected, actual);
// }

#[test]
fn matrix4x4_add_test() {
    let a = generate_matrix_number_from_1_to_16();
    let b = generate_matrix_number_from_1_to_16();

    let expected = Matrix4x4::new(
        a.m11 + b.m11, a.m12 + b.m12, a.m13 + b.m13, a.m14 + b.m14,
        a.m21 + b.m21, a.m22 + b.m22, a.m23 + b.m23, a.m24 + b.m24,
        a.m31 + b.m31, a.m32 + b.m32, a.m33 + b.m33, a.m34 + b.m34,
        a.m41 + b.m41, a.m42 + b.m42, a.m43 + b.m43, a.m44 + b.m44,
    );

    let actual = a.add(b);
    assert!(expected.almost_equals(&actual, 1e-5));
}

#[test]
fn matrix4x4_equals_test() {
    let a = generate_matrix_number_from_1_to_16();
    let b = generate_matrix_number_from_1_to_16();

    // case 1: compare between same values
    let obj: &Matrix4x4<f32> = &b;

    let expected = true;
    let actual = a == *obj;
    assert_eq!(expected, actual);

    // case 2: compare between different values
    let mut c = b;
    c.m11 = 11.0;
    let obj: &Matrix4x4<f32> = &c;
    let expected = false;
    let actual = a == *obj;
    assert_eq!(expected, actual);
}

#[test]
fn matrix4x4_get_hash_code_test() {
    let target = generate_matrix_number_from_1_to_16();
    
    let mut hasher = DefaultHasher::new();
    hash_float(target.m11, &mut hasher);
    hash_float(target.m12, &mut hasher);
    hash_float(target.m13, &mut hasher);
    hash_float(target.m14, &mut hasher);

    hash_float(target.m21, &mut hasher);
    hash_float(target.m22, &mut hasher);
    hash_float(target.m23, &mut hasher);
    hash_float(target.m24, &mut hasher);

    hash_float(target.m31, &mut hasher);
    hash_float(target.m32, &mut hasher);
    hash_float(target.m33, &mut hasher);
    hash_float(target.m34, &mut hasher);

    hash_float(target.m41, &mut hasher);
    hash_float(target.m42, &mut hasher);
    hash_float(target.m43, &mut hasher);
    hash_float(target.m44, &mut hasher);

    let expected = hasher.finish();
    let actual = hash_code(&target);
    assert_eq!(expected, actual);
}

#[test]
fn matrix4x4_multiply_test3() {
    let a = generate_matrix_number_from_1_to_16();
    let b = generate_matrix_number_from_1_to_16();

    let expected = Matrix4x4::<f32> {
        m11: a.m11 * b.m11 + a.m12 * b.m21 + a.m13 * b.m31 + a.m14 * b.m41,
        m12: a.m11 * b.m12 + a.m12 * b.m22 + a.m13 * b.m32 + a.m14 * b.m42,
        m13: a.m11 * b.m13 + a.m12 * b.m23 + a.m13 * b.m33 + a.m14 * b.m43,
        m14: a.m11 * b.m14 + a.m12 * b.m24 + a.m13 * b.m34 + a.m14 * b.m44,
        m21: a.m21 * b.m11 + a.m22 * b.m21 + a.m23 * b.m31 + a.m24 * b.m41,
        m22: a.m21 * b.m12 + a.m22 * b.m22 + a.m23 * b.m32 + a.m24 * b.m42,
        m23: a.m21 * b.m13 + a.m22 * b.m23 + a.m23 * b.m33 + a.m24 * b.m43,
        m24: a.m21 * b.m14 + a.m22 * b.m24 + a.m23 * b.m34 + a.m24 * b.m44,
        m31: a.m31 * b.m11 + a.m32 * b.m21 + a.m33 * b.m31 + a.m34 * b.m41,
        m32: a.m31 * b.m12 + a.m32 * b.m22 + a.m33 * b.m32 + a.m34 * b.m42,
        m33: a.m31 * b.m13 + a.m32 * b.m23 + a.m33 * b.m33 + a.m34 * b.m43,
        m34: a.m31 * b.m14 + a.m32 * b.m24 + a.m33 * b.m34 + a.m34 * b.m44,
        m41: a.m41 * b.m11 + a.m42 * b.m21 + a.m43 * b.m31 + a.m44 * b.m41,
        m42: a.m41 * b.m12 + a.m42 * b.m22 + a.m43 * b.m32 + a.m44 * b.m42,
        m43: a.m41 * b.m13 + a.m42 * b.m23 + a.m43 * b.m33 + a.m44 * b.m43,
        m44: a.m41 * b.m14 + a.m42 * b.m24 + a.m43 * b.m34 + a.m44 * b.m44,
    };
    let actual = a.mul(b);
    assert_eq!(expected, actual);
}

#[test]
fn matrix4x4_multiply_test5() {
    let a = generate_matrix_number_from_1_to_16();
    let expected = Matrix4x4::<f32>::new(
        3.0f32, 6.0f32, 9.0f32, 12.0,
        15.0f32, 18.0f32, 21.0f32, 24.0,
        27.0f32, 30.0f32, 33.0f32, 36.0,
        39.0f32, 42.0f32, 45.0f32, 48.0,
    );
    let actual = a.mul(3.0f32);
    assert_eq!(expected, actual);
}

#[test]
fn matrix4x4_multiply_test6() {
    let a = generate_matrix_number_from_1_to_16();
    let expected = Matrix4x4::new(
        3.0f32, 6.0f32, 9.0f32, 12.0,
        15.0f32, 18.0f32, 21.0f32, 24.0,
        27.0f32, 30.0f32, 33.0f32, 36.0,
        39.0f32, 42.0f32, 45.0f32, 48.0,
    );
    let actual = a * 3.0;
    assert_eq!(expected, actual);
}

#[test]
fn matrix4x4_negate_test() {
    let m = generate_matrix_number_from_1_to_16();
    let expected = Matrix4x4::<f32>::new(
        -1.0f32, -2.0f32, -3.0f32, -4.0,
        -5.0f32, -6.0f32, -7.0f32, -8.0,
        -9.0f32, -10.0f32, -11.0f32, -12.0,
        -13.0f32, -14.0f32, -15.0f32, -16.0,
    );
    let actual = m.neg();
    assert_eq!(expected, actual);
}

#[test]
fn matrix4x4_inequality_test() {
    let a = generate_matrix_number_from_1_to_16();
    let b = generate_matrix_number_from_1_to_16();

    // case 1: compare between same values
    let expected = false;
    let actual = a != b;
    assert_eq!(expected, actual);

    // case 2: compare between different values
    let mut b = b;
    b.m11 = 11.0f32;
    let expected = true;
    let actual = a != b;
    assert_eq!(expected, actual);
}

#[test]
fn matrix4x4_equality_test() {
    let a = generate_matrix_number_from_1_to_16();
    let b = generate_matrix_number_from_1_to_16();

    // case 1: compare between same values
    let expected = true;
    let actual = a == b;
    assert_eq!(expected, actual);

    // case 2: compare between different values
    let mut b = b;
    b.m11 = 11.0f32;
    let expected = false;
    let actual = a == b;
    assert_eq!(expected, actual);
}

#[test]
fn matrix4x4_subtract_test() {
    let a = generate_matrix_number_from_1_to_16();
    let b = generate_matrix_number_from_1_to_16();
    let expected = Matrix4x4::<f32>::zero();

    let actual = a.sub(b);
    assert_eq!(expected, actual);
}

fn create_billboard_fact(place_direction: Vector3<f32>, camera_up_vector: Vector3<f32>, expected_rotation: Matrix4x4<f32>) {
    let camera_position = Vector3::new(3.0f32, 4.0f32, 5.0);
    let object_position = camera_position + place_direction * 10.0f32;
    let expected = expected_rotation * Matrix4x4::create_translation(object_position);
    let actual = Matrix4x4::create_billboard(object_position, camera_position, camera_up_vector, Vector3::new(0.0f32, 0.0f32, -1.0));
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_billboard did not return the expected value.");
}

#[test]
fn matrix4x4_create_billboard_test01() {
    create_billboard_fact(
        Vector3::new(0.0f32, 0.0f32, -1.0),
        Vector3::new(0.0f32, 1.0f32, 0.0),
        Matrix4x4::create_rotation_y(180.0f32.to_radians()),
    );
}

#[test]
fn matrix4x4_create_billboard_test02() {
    create_billboard_fact(
        Vector3::new(0.0f32, 0.0f32, 1.0),
        Vector3::new(0.0f32, 1.0f32, 0.0),
        Matrix4x4::create_rotation_y(0.0f32.to_radians()),
    );
}

#[test]
fn matrix4x4_create_billboard_test03() {
    create_billboard_fact(
        Vector3::new(1.0f32, 0.0f32, 0.0),
        Vector3::new(0.0f32, 1.0f32, 0.0),
        Matrix4x4::create_rotation_y(90.0f32.to_radians()),
    );
}

#[test]
fn matrix4x4_create_billboard_test04() {
    create_billboard_fact(
        Vector3::new(-1.0f32, 0.0f32, 0.0),
        Vector3::new(0.0f32, 1.0f32, 0.0),
        Matrix4x4::create_rotation_y(-90.0f32.to_radians()),
    );
}

#[test]
fn matrix4x4_create_billboard_test05() {
    create_billboard_fact(
        Vector3::new(0.0f32, 1.0f32, 0.0),
        Vector3::new(0.0f32, 0.0f32, 1.0),
        Matrix4x4::create_rotation_x(90.0f32.to_radians())
            * Matrix4x4::create_rotation_z(180.0f32.to_radians()),
    );
}

#[test]
fn matrix4x4_create_billboard_test06() {
    create_billboard_fact(
        Vector3::new(0.0f32, -1.0f32, 0.0),
        Vector3::new(0.0f32, 0.0f32, 1.0),
        Matrix4x4::create_rotation_x(90.0f32.to_radians())
            * Matrix4x4::create_rotation_z(0.0f32.to_radians()),
    );
}

#[test]
fn matrix4x4_create_billboard_test07() {
    create_billboard_fact(
        Vector3::new(1.0f32, 0.0f32, 0.0),
        Vector3::new(0.0f32, 0.0f32, 1.0),
        Matrix4x4::create_rotation_x(90.0f32.to_radians())
            * Matrix4x4::create_rotation_z(90.0f32.to_radians()),
    );
}

#[test]
fn matrix4x4_create_billboard_test08() {
    create_billboard_fact(
        Vector3::new(-1.0f32, 0.0f32, 0.0),
        Vector3::new(0.0f32, 0.0f32, 1.0),
        Matrix4x4::create_rotation_x(90.0f32.to_radians())
            * Matrix4x4::create_rotation_z(-90.0f32.to_radians()),
    );
}

#[test]
fn matrix4x4_create_billboard_test09() {
    create_billboard_fact(
    Vector3::new(0.0f32, 1.0f32, 0.0),
    Vector3::new(-1.0f32, 0.0f32, 0.0),
    Matrix4x4::create_rotation_z(90.0f32.to_radians())
        * Matrix4x4::create_rotation_x(-90.0f32.to_radians()),
    );
}

#[test]
fn matrix4x4_create_billboard_test10() {
    create_billboard_fact(
    Vector3::new(0.0f32, -1.0f32, 0.0),
    Vector3::new(-1.0f32, 0.0f32, 0.0),
    Matrix4x4::create_rotation_z(90.0f32.to_radians())
        * Matrix4x4::create_rotation_x(90.0f32.to_radians()),
    );
}

#[test]
fn matrix4x4_create_billboard_test11() {
    create_billboard_fact(
    Vector3::new(0.0f32, 0.0f32, -1.0),
    Vector3::new(-1.0f32, 0.0f32, 0.0),
    Matrix4x4::create_rotation_z(90.0f32.to_radians())
        * Matrix4x4::create_rotation_x(180.0f32.to_radians()),
    );
}

#[test]
fn matrix4x4_create_billboard_test12() {
    create_billboard_fact(
    Vector3::new(0.0f32, 0.0f32, 1.0),
    Vector3::new(-1.0f32, 0.0f32, 0.0),
    Matrix4x4::create_rotation_z(90.0f32.to_radians())
        * Matrix4x4::create_rotation_x(0.0f32.to_radians()),
    );
}

#[test]
fn matrix4x4_create_billboard_too_close_test1() {
    let object_position = Vector3::new(3.0f32, 4.0f32, 5.0);
    let camera_position = object_position;
    let camera_up_vector = Vector3::new(0.0f32, 1.0f32, 0.0);

    let expected = Matrix4x4::create_rotation_y(180.0f32.to_radians())
        * Matrix4x4::create_translation(object_position);
    let actual = Matrix4x4::create_billboard(
        object_position,
        camera_position,
        camera_up_vector,
        Vector3::new(0.0f32, 0.0f32, 1.0),
    );
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_billboard did not return the expected value.");
}

#[test]
fn matrix4x4_create_billboard_too_close_test2() {
    let object_position = Vector3::new(3.0f32, 4.0f32, 5.0);
    let camera_position = object_position;
    let camera_up_vector = Vector3::new(0.0f32, 1.0f32, 0.0);

    let expected = Matrix4x4::create_rotation_y(-90.0f32.to_radians())
        * Matrix4x4::create_translation(object_position);
    let actual = Matrix4x4::create_billboard(
        object_position,
        camera_position,
        camera_up_vector,
        Vector3::new(1.0f32, 0.0f32, 0.0),
    );
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_billboard did not return the expected value.");
}

fn create_constrained_billboard_fact(place_direction: Vector3<f32>, rotate_axis: Vector3<f32>, expected_rotation: Matrix4x4<f32>) {
    let camera_position = Vector3::new(3.0f32, 4.0f32, 5.0);
    let object_position = camera_position + place_direction * 10.0f32;
    let expected = expected_rotation * Matrix4x4::create_translation(object_position);
    let actual = Matrix4x4::create_constrained_billboard(object_position, camera_position, rotate_axis, Vector3::new(0.0f32, 0.0f32, -1.0), Vector3::new(0.0f32, 0.0f32, -1.0));
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_constrained_billboard did not return the expected value.");

    // When you move camera along rotateAxis, result must be same.
    let mut camera_position = camera_position + rotate_axis * 10.0f32;
    let actual = Matrix4x4::create_constrained_billboard(object_position, camera_position, rotate_axis, Vector3::new(0.0f32, 0.0f32, -1.0), Vector3::new(0.0f32, 0.0f32, -1.0));
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_constrained_billboard did not return the expected value.");

    camera_position = camera_position - rotate_axis * 30.0f32;
    let actual = Matrix4x4::create_constrained_billboard(object_position, camera_position, rotate_axis, Vector3::new(0.0f32, 0.0f32, -1.0), Vector3::new(0.0f32, 0.0f32, -1.0));
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_constrained_billboard did not return the expected value.");
}

#[test]
fn matrix4x4_create_constrained_billboard_test01() {
    // Place object at Forward side of camera on XZ-plane
    // Object placed at Forward of camera. result must be same as 180 degrees rotate along y-axis.
    create_constrained_billboard_fact(Vector3::new(0.0f32, 0.0f32, -1.0), Vector3::new(0.0f32, 1.0f32, 0.0), Matrix4x4::create_rotation_y(180.0f32.to_radians()));
}

#[test]
fn matrix4x4_create_constrained_billboard_test02() {
    // Place object at Backward side of camera on XZ-plane
    // Object placed at Backward of camera. This result must be same as 0 degrees rotate along y-axis.
    create_constrained_billboard_fact(Vector3::new(0.0f32, 0.0f32, 1.0), Vector3::new(0.0f32, 1.0f32, 0.0), Matrix4x4::create_rotation_y(0.0f32.to_radians()));
}

// A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
#[test]
fn matrix4x4_create_constrained_billboard_test03() {
    // Place object at Right side of camera. This result must be same as 90 degrees rotate along y-axis.
    create_constrained_billboard_fact(
    Vector3::new(1.0f32, 0.0f32, 0.0),
    Vector3::new(0.0f32, 1.0f32, 0.0),
    Matrix4x4::create_rotation_y(90.0f32.to_radians()),
    );
}

// A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
#[test]
fn matrix4x4_create_constrained_billboard_test04() {
    // Place object at Left side of camera. This result must be same as -90 degrees rotate along y-axis.
    create_constrained_billboard_fact(
    Vector3::new(-1.0f32, 0.0f32, 0.0),
    Vector3::new(0.0f32, 1.0f32, 0.0),
    Matrix4x4::create_rotation_y(-90.0f32.to_radians()),
    );
}

// A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
#[test]
fn matrix4x4_create_constrained_billboard_test05() {
    // Place object at Up side of camera. result must be same as 180 degrees rotate along z-axis after 90 degrees rotate along x-axis.
    create_constrained_billboard_fact(
    Vector3::new(0.0f32, 1.0f32, 0.0),
    Vector3::new(0.0f32, 0.0f32, 1.0),
    Matrix4x4::create_rotation_x(90.0f32.to_radians())
        * Matrix4x4::create_rotation_z(180.0f32.to_radians()),
    );
}

// A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
#[test]
fn matrix4x4_create_constrained_billboard_test06() {
    // Place object at Down side of camera. result must be same as 0 degrees rotate along z-axis after 90 degrees rotate along x-axis.
    create_constrained_billboard_fact(
    Vector3::new(0.0f32, -1.0f32, 0.0),
    Vector3::new(0.0f32, 0.0f32, 1.0),
    Matrix4x4::create_rotation_x(90.0f32.to_radians()) 
        * Matrix4x4::create_rotation_z(0.0f32.to_radians()),
    );
}

// A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
#[test]
fn matrix4x4_create_constrained_billboard_test07() {
    // Place object at Right side of camera. result must be same as 90 degrees rotate along z-axis after 90 degrees rotate along x-axis.
    create_constrained_billboard_fact(
    Vector3::new(1.0f32, 0.0f32, 0.0),
    Vector3::new(0.0f32, 0.0f32, 1.0),
    Matrix4x4::create_rotation_x(90.0f32.to_radians()) 
        * Matrix4x4::create_rotation_z(90.0f32.to_radians()),
    );
}

// A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
// Place object at Left side of camera on XY-plane
#[test]
fn matrix4x4_create_constrained_billboard_test08() {
    // Place object at Left side of camera. result must be same as -90 degrees rotate along z-axis after 90 degrees rotate along x-axis.
    create_constrained_billboard_fact(
    Vector3::new(-1.0f32, 0.0f32, 0.0),
    Vector3::new(0.0f32, 0.0f32, 1.0),
    Matrix4x4::create_rotation_x(90.0f32.to_radians())
        * Matrix4x4::create_rotation_z(-90.0f32.to_radians()),
    );
}

// A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
// Place object at Up side of camera on YZ-plane
#[test]
fn matrix4x4_create_constrained_billboard_test09() {
    // Place object at Up side of camera. result must be same as -90 degrees rotate along x-axis after 90 degrees rotate along z-axis.
    create_constrained_billboard_fact(
    Vector3::new(0.0f32, 1.0f32, 0.0),
    Vector3::new(-1.0f32, 0.0f32, 0.0),
    Matrix4x4::create_rotation_z(90.0f32.to_radians())
        * Matrix4x4::create_rotation_x(-90.0f32.to_radians()),
    );
}

// A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
// Place object at Down side of camera on YZ-plane
#[test]
fn matrix4x4_create_constrained_billboard_test10() {
    // Place object at Down side of camera. result must be same as 90 degrees rotate along x-axis after 90 degrees rotate along z-axis.
    create_constrained_billboard_fact(
    Vector3::new(0.0f32, -1.0f32, 0.0),
    Vector3::new(-1.0f32, 0.0f32, 0.0),
    Matrix4x4::create_rotation_z(90.0f32.to_radians())
        * Matrix4x4::create_rotation_x(90.0f32.to_radians()),
    );
}

// A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
// Place object at Forward side of camera on YZ-plane
#[test]
fn matrix4x4_create_constrained_billboard_test11() {
    // Place object at Forward side of camera. result must be same as 180 degrees rotate along x-axis after 90 degrees rotate along z-axis.
    create_constrained_billboard_fact(
    Vector3::new(0.0f32, 0.0f32, -1.0),
    Vector3::new(-1.0f32, 0.0f32, 0.0),
    Matrix4x4::create_rotation_z(90.0f32.to_radians())
        * Matrix4x4::create_rotation_x(180.0f32.to_radians()),
    );
}

#[test]
fn matrix4x4_create_constrained_billboard_test12() {
    // Place object at Backward side of camera. result must be same as 0 degrees rotate along x-axis after 90 degrees rotate along z-axis.
    create_constrained_billboard_fact(
    Vector3::new(0.0f32, 0.0f32, 1.0),
    Vector3::new(-1.0f32, 0.0f32, 0.0),
    Matrix4x4::create_rotation_z(90.0f32.to_radians())
        * Matrix4x4::create_rotation_x(0.0f32.to_radians()),
    );
}

#[test]
fn matrix4x4_create_constrained_billboard_too_close_test1() {
    let object_position = Vector3::new(3.0f32, 4.0f32, 5.0);
    let camera_position = object_position;
    let camera_up_vector = Vector3::new(0.0f32, 1.0f32, 0.0);

    // Doesn't pass camera face direction. CreateConstrainedBillboard uses new Vector3f(0f32, 0f32, -1) direction. Result must be same as 180 degrees rotate along y-axis.
    let expected = 
        Matrix4x4::create_rotation_y(180.0f32.to_radians()) 
        * Matrix4x4::create_translation(object_position);
    let actual = Matrix4x4::create_constrained_billboard(object_position, camera_position, camera_up_vector, Vector3::new(0f32, 0f32, 1f32), Vector3::new(0f32, 0f32, -1f32));
    
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_constrained_billboard did not return the expected value.");
}

#[test]
fn matrix4x4_create_constrained_billboard_too_close_test2() {
    let object_position = Vector3::new(3.0f32, 4.0f32, 5.0);
    let camera_position = object_position;
    let camera_up_vector = Vector3::new(0.0f32, 1.0f32, 0.0);

    // Passes Vector3f.Right as camera face direction. Result must be same as -90 degrees rotate along y-axis.
    let expected = 
        Matrix4x4::create_rotation_y(-90.0f32.to_radians()) 
            * Matrix4x4::create_translation(object_position);
    let actual = Matrix4x4::create_constrained_billboard(
        object_position, 
        camera_position, 
        camera_up_vector, 
        Vector3::new(1f32, 0f32, 0f32), 
        Vector3::new(0f32, 0f32, -1f32));

    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_constrained_billboard did not return the expected value.");
}

#[test]
fn matrix4x4_create_constrained_billboard_along_axis_test1() {
    // Place camera at up side of object.
    let object_position = Vector3::new(3.0f32, 4.0f32, 5.0);
    let rotate_axis = Vector3::new(0.0f32, 1.0f32, 0.0);
    let camera_position = object_position + rotate_axis * 10.0f32;

    // In this case, CreateConstrainedBillboard picks new Vector3f(0f32, 0f32, -1) as object forward vector.
    let expected = Matrix4x4::create_rotation_y(180.0f32.to_radians())
        * Matrix4x4::create_translation(object_position);
    let actual = Matrix4x4::create_constrained_billboard(
        object_position,
        camera_position,
        rotate_axis,
        Vector3::new(0.0f32, 0.0f32, -1.0),
        Vector3::new(0.0f32, 0.0f32, -1.0),
    );
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_constrained_billboard did not return the expected value.");
}

// A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
// Angle between rotateAxis and camera to object vector is too small. And user doesn't passed objectForwardVector parameter.
#[test]
fn matrix4x4_create_constrained_billboard_along_axis_test2() {
    // Place camera at up side of object.
    let object_position = Vector3::new(3.0f32, 4.0f32, 5.0);
    let rotate_axis = Vector3::new(0.0f32, 0.0f32, -1.0);
    let camera_position = object_position + rotate_axis * 10.0f32;

    // In this case, CreateConstrainedBillboard picks new Vector3f(1, 0f32, 0) as object forward vector.
    let expected = Matrix4x4::create_rotation_x(-90.0f32.to_radians())
        * Matrix4x4::create_rotation_z(-90.0f32.to_radians())
        * Matrix4x4::create_translation(object_position);
    let actual = Matrix4x4::create_constrained_billboard(
        object_position,
        camera_position,
        rotate_axis,
        Vector3::new(0.0f32, 0.0f32, -1.0),
        Vector3::new(1.0f32, 0.0f32, 0.0),
    );
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_constrained_billboard did not return the expected value.");
}


#[test]
fn matrix4x4_create_constrained_billboard_along_axis_test3() {
    // Place camera at up side of object.
    let object_position = Vector3::new(3.0f32, 4.0f32, 5.0);
    let rotate_axis = Vector3::new(0.0f32, 1.0f32, 0.0);
    let camera_position = object_position + rotate_axis * 10.0f32;

    // User passes correct object_forward_vector.
    let expected = Matrix4x4::create_rotation_y(180.0f32.to_radians())
        * Matrix4x4::create_translation(object_position);
    let actual = Matrix4x4::create_constrained_billboard(
        object_position,
        camera_position,
        rotate_axis,
        Vector3::new(0.0f32, 0.0f32, -1.0),
        Vector3::new(0.0f32, 0.0f32, -1.0)
    );
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_constrained_billboard did not return the expected value.");
}

#[test]
fn matrix4x4_create_constrained_billboard_along_axis_test4() {
    // Place camera at up side of object.
    let object_position = Vector3::new(3.0f32, 4.0f32, 5.0);
    let rotate_axis = Vector3::new(0.0f32, 1.0f32, 0.0);
    let camera_position = object_position + rotate_axis * 10.0f32;

    // User passes incorrect object_forward_vector.
    let expected = Matrix4x4::create_rotation_y(180.0f32.to_radians())
        * Matrix4x4::create_translation(object_position);
    let actual = Matrix4x4::create_constrained_billboard(
        object_position,
        camera_position,
        rotate_axis,
        Vector3::new(0.0f32, 0.0f32, -1.0),
        Vector3::new(0.0f32, 1.0f32, 0.0),
    );
    assert!(expected.almost_equals(&actual, 1e-5), "Matrix4x4::create_constrained_billboard did not return the expected value.");
}

#[test]
fn matrix4x4_create_constrained_billboard_along_axis_test5() {
    // Place camera at up side of object.
    let object_position = Vector3::new(3.0f32, 4.0f32, 5.0);
    let rotate_axis = Vector3::new(0.0f32, 0.0f32, -1.0);
    let camera_position = object_position + rotate_axis * 10.0f32;

    // In this case, create_constrained_billboard picks Vector3::new(1.0f32, 0.0f32, 0.0) as object forward vector.
    let expected = Matrix4x4::create_rotation_x(-90.0f32.to_radians())
        * Matrix4x4::create_rotation_z(-90.0f32.to_radians())
        * Matrix4x4::create_translation(object_position);
    let actual = Matrix4x4::create_constrained_billboard(
        object_position,
        camera_position,
        rotate_axis,
        Vector3::new(0.0f32, 0.0f32, -1.0),
        Vector3::new(0.0f32, 0.0f32, -1.0),
    );
    assert!(expected.almost_equals(&actual, 1e-5));
}

// A test for CreateScale (Vector3f)
#[test]
fn matrix4x4_create_scale_test1() {
    let scales = Vector3::new(2.0f32, 3.0f32, 4.0);
    let expected = Matrix4x4::new(
        2.0f32, 0.0f32, 0.0f32, 0.0,
        0.0f32, 3.0f32, 0.0f32, 0.0,
        0.0f32, 0.0f32, 4.0f32, 0.0,
        0.0f32, 0.0f32, 0.0f32, 1.0,
    );
    let actual = Matrix4x4::create_scale(scales);
    assert_eq!(expected, actual);
}

// A test for CreateScale (Vector3f, Vector3f)
#[test]
fn matrix4x4_create_scale_center_test1() {
    let scale = Vector3::new(3.0f32, 4.0f32, 5.0);
    let center = Vector3::new(23.0f32, 42.0f32, 666.0);

    let scale_around_zero = Matrix4x4::create_scale_centered(scale, Vector3::zero());
    let scale_around_zero_expected = Matrix4x4::create_scale(scale);
    assert_eq!(scale_around_zero, scale_around_zero_expected);

    let scale_around_center = Matrix4x4::create_scale_centered(scale, center);
    let scale_around_center_expected =
        Matrix4x4::create_translation(-center) * 
        Matrix4x4::create_scale(scale) * 
        Matrix4x4::create_translation(center);
    assert_eq!(scale_around_center, scale_around_center_expected);
}

// A test for CreateScale (float)
#[test]
fn matrix4x4_create_scale_test2() {
    let scale = 2.0f32;
    let expected = Matrix4x4::new(
        2.0f32, 0.0f32, 0.0f32, 0.0,
        0.0f32, 2.0f32, 0.0f32, 0.0,
        0.0f32, 0.0f32, 2.0f32, 0.0,
        0.0f32, 0.0f32, 0.0f32, 1.0,
    );
    let actual = Matrix4x4::create_scale_uniform(scale);
    assert_eq!(expected, actual);
}

#[test]
fn matrix4x4_create_scale_center_test2() {
    let scale = 5.0f32;
    let center = Vector3::new(23.0f32, 42.0f32, 666.0);

    let scale_around_zero = Matrix4x4::create_scale_uniform_centered(scale, Vector3::zero());
    let scale_around_zero_expected = Matrix4x4::create_scale_uniform(scale);
    assert_eq!(scale_around_zero, scale_around_zero_expected);

    let scale_around_center = Matrix4x4::create_scale_uniform_centered(scale, center);
    let scale_around_center_expected =
        Matrix4x4::create_translation(-center) * 
        Matrix4x4::create_scale_uniform(scale) * 
        Matrix4x4::create_translation(center);
        assert_eq!(scale_around_center, scale_around_center_expected);
}

#[test]
fn matrix4x4_create_scale_test3() {
    let x_scale = 2.0f32;
    let y_scale = 3.0f32;
    let z_scale = 4.0f32;
    let expected = Matrix4x4::new(
        x_scale, 0.0f32, 0.0f32, 0.0,
        0.0f32, y_scale, 0.0f32, 0.0,
        0.0f32, 0.0f32, z_scale, 0.0,
        0.0f32, 0.0f32, 0.0f32, 1.0,
    );
    let actual = Matrix4x4::create_scale_xyz(x_scale, y_scale, z_scale);
    assert_eq!(expected, actual);
}

#[test]
fn matrix4x4_create_scale_center_test3() {
    let scale = Vector3::new(3.0f32, 4.0f32, 5.0);
    let center = Vector3::new(23.0f32, 42.0f32, 666.0);

    let scale_around_zero = Matrix4x4::create_scale_xyz_center(scale.x, scale.y, scale.z, Vector3::zero());
    let scale_around_zero_expected = Matrix4x4::create_scale_xyz(scale.x, scale.y, scale.z);
    assert_eq!(scale_around_zero, scale_around_zero_expected);

    let scale_around_center = Matrix4x4::create_scale_xyz_center(scale.x, scale.y, scale.z, center);
    let scale_around_center_expected =
        Matrix4x4::create_translation(-center) * 
        Matrix4x4::create_scale_xyz(scale.x, scale.y, scale.z) * 
        Matrix4x4::create_translation(center);
    assert_eq!(scale_around_center, scale_around_center_expected);
}

#[test]
fn matrix4x4_create_translation_test1() {
    let position = Vector3::new(2.0f32, 3.0f32, 4.0);
    let expected = Matrix4x4::new(
        1.0f32, 0.0f32, 0.0f32, 0.0,
        0.0f32, 1.0f32, 0.0f32, 0.0,
        0.0f32, 0.0f32, 1.0f32, 0.0,
        2.0f32, 3.0f32, 4.0f32, 1.0,
    );

    let actual = Matrix4x4::create_translation(position);
    assert_eq!(expected, actual);
}

#[test]
fn matrix4x4_create_translation_test2() {
    let x_position = 2.0f32;
    let y_position = 3.0f32;
    let z_position = 4.0f32;

    let expected = Matrix4x4::new(
        1.0f32, 0.0f32, 0.0f32, 0.0,
        0.0f32, 1.0f32, 0.0f32, 0.0,
        0.0f32, 0.0f32, 1.0f32, 0.0,
        2.0f32, 3.0f32, 4.0f32, 1.0,
    );

    let actual = Matrix4x4::create_translation_xyz(x_position, y_position, z_position);
    assert_eq!(expected, actual);
}

#[test]
fn matrix4x4_translation_test() {
    let mut a = generate_test_matrix();
    let b = a;

    // Transformed vector that has same semantics of property must be same.
    let val = Vector3::new(a.m41, a.m42, a.m43);
    assert_eq!(val, a.translation());

    // Set value and get value must be same.
    let val = Vector3::new(1.0f32, 2.0f32, 3.0);
    a = a.set_translation(val);
    assert_eq!(val, a.translation());

    // Make sure it only modifies expected value of matrix.
    assert!(a.m11 == b.m11 && a.m12 == b.m12 && a.m13 == b.m13 && a.m14 == b.m14
        && a.m21 == b.m21 && a.m22 == b.m22 && a.m23 == b.m23 && a.m24 == b.m24
        && a.m31 == b.m31 && a.m32 == b.m32 && a.m33 == b.m33 && a.m34 == b.m34
        && a.m41 != b.m41 && a.m42 != b.m42 && a.m43 != b.m43 && a.m44 == b.m44);
}

#[test]
fn matrix4x4_equals_test1() {
    let a = generate_matrix_number_from_1_to_16();
    let mut b = generate_matrix_number_from_1_to_16();

    // case 1: compare between same values
    let expected = true;
    let actual = a.eq(&b);
    assert_eq!(expected, actual);

    // case 2: compare between different values
    b.m11 = 11.0;
    let expected = false;
    let actual = a.eq(&b);
    assert_eq!(expected, actual);
}

#[test]
fn matrix4x4_is_identity_test() {
    assert!(Matrix4x4::<f32>::identity().is_identity());
    assert!(Matrix4x4::<f32>::new(1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32).is_identity());
    assert!(!Matrix4x4::<f32>::new(0f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32).is_identity());
    assert!(!Matrix4x4::<f32>::new(1f32, 1f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32).is_identity());
    assert!(!Matrix4x4::<f32>::new(1f32, 0f32, 1f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32).is_identity());
    assert!(!Matrix4x4::<f32>::new(1f32, 0f32, 0f32, 1f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32).is_identity());
    assert!(!Matrix4x4::<f32>::new(1f32, 0f32, 0f32, 0f32, 1f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32).is_identity());
    assert!(!Matrix4x4::<f32>::new(1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32).is_identity());
    assert!(!Matrix4x4::<f32>::new(1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 1f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32).is_identity());
    assert!(!Matrix4x4::<f32>::new(1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 1f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32).is_identity());
    assert!(!Matrix4x4::<f32>::new(1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 1f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32).is_identity());
    assert!(!Matrix4x4::<f32>::new(1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 1f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32).is_identity());
    assert!(!Matrix4x4::<f32>::new(1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 1f32).is_identity());
    assert!(!Matrix4x4::<f32>::new(1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 1f32, 0f32, 0f32, 0f32, 1f32).is_identity());
    assert!(!Matrix4x4::<f32>::new(1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 1f32, 0f32, 0f32, 1f32).is_identity());
    assert!(!Matrix4x4::<f32>::new(1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 1f32, 0f32, 1f32).is_identity());
    assert!(!Matrix4x4::<f32>::new(1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 1f32, 1f32).is_identity());
    assert!(!Matrix4x4::<f32>::new(1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 0f32).is_identity());
}

#[test]
fn matrix4x4_equals_nan_test()
{
    let a = Matrix4x4::<f32>::new(f32::NAN, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32);
    let b = Matrix4x4::<f32>::new(0f32, f32::NAN, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32);
    let c = Matrix4x4::<f32>::new(0f32, 0f32, f32::NAN, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32);
    let d = Matrix4x4::<f32>::new(0f32, 0f32, 0f32, f32::NAN, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32);
    let e = Matrix4x4::<f32>::new(0f32, 0f32, 0f32, 0f32, f32::NAN, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32);
    let f = Matrix4x4::<f32>::new(0f32, 0f32, 0f32, 0f32, 0f32, f32::NAN, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32);
    let g = Matrix4x4::<f32>::new(0f32, 0f32, 0f32, 0f32, 0f32, 0f32, f32::NAN, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32);
    let h = Matrix4x4::<f32>::new(0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, f32::NAN, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32);
    let i = Matrix4x4::<f32>::new(0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, f32::NAN, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32);
    let j = Matrix4x4::<f32>::new(0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, f32::NAN, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32);
    let k = Matrix4x4::<f32>::new(0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, f32::NAN, 0f32, 0f32, 0f32, 0f32, 0f32);
    let l = Matrix4x4::<f32>::new(0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, f32::NAN, 0f32, 0f32, 0f32, 0f32);
    let m = Matrix4x4::<f32>::new(0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, f32::NAN, 0f32, 0f32, 0f32);
    let n = Matrix4x4::<f32>::new(0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, f32::NAN, 0f32, 0f32);
    let o = Matrix4x4::<f32>::new(0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, f32::NAN, 0f32);
    let p = Matrix4x4::<f32>::new(0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, f32::NAN);

    assert!(!(a == Matrix4x4::<f32>::zero()));
    assert!(!(b == Matrix4x4::<f32>::zero()));
    assert!(!(c == Matrix4x4::<f32>::zero()));
    assert!(!(d == Matrix4x4::<f32>::zero()));
    assert!(!(e == Matrix4x4::<f32>::zero()));
    assert!(!(f == Matrix4x4::<f32>::zero()));
    assert!(!(g == Matrix4x4::<f32>::zero()));
    assert!(!(h == Matrix4x4::<f32>::zero()));
    assert!(!(i == Matrix4x4::<f32>::zero()));
    assert!(!(j == Matrix4x4::<f32>::zero()));
    assert!(!(k == Matrix4x4::<f32>::zero()));
    assert!(!(l == Matrix4x4::<f32>::zero()));
    assert!(!(m == Matrix4x4::<f32>::zero()));
    assert!(!(n == Matrix4x4::<f32>::zero()));
    assert!(!(o == Matrix4x4::<f32>::zero()));
    assert!(!(p == Matrix4x4::<f32>::zero()));

    assert!((a != Matrix4x4::<f32>::zero()));
    assert!((b != Matrix4x4::<f32>::zero()));
    assert!((c != Matrix4x4::<f32>::zero()));
    assert!((d != Matrix4x4::<f32>::zero()));
    assert!((e != Matrix4x4::<f32>::zero()));
    assert!((f != Matrix4x4::<f32>::zero()));
    assert!((g != Matrix4x4::<f32>::zero()));
    assert!((h != Matrix4x4::<f32>::zero()));
    assert!((i != Matrix4x4::<f32>::zero()));
    assert!((j != Matrix4x4::<f32>::zero()));
    assert!((k != Matrix4x4::<f32>::zero()));
    assert!((l != Matrix4x4::<f32>::zero()));
    assert!((m != Matrix4x4::<f32>::zero()));
    assert!((n != Matrix4x4::<f32>::zero()));
    assert!((o != Matrix4x4::<f32>::zero()));
    assert!((p != Matrix4x4::<f32>::zero()));
  
    assert!(!(a.eq(&Matrix4x4::<f32>::zero())));
    assert!(!(b.eq(&Matrix4x4::<f32>::zero())));
    assert!(!(c.eq(&Matrix4x4::<f32>::zero())));
    assert!(!(d.eq(&Matrix4x4::<f32>::zero())));
    assert!(!(e.eq(&Matrix4x4::<f32>::zero())));
    assert!(!(f.eq(&Matrix4x4::<f32>::zero())));
    assert!(!(g.eq(&Matrix4x4::<f32>::zero())));
    assert!(!(h.eq(&Matrix4x4::<f32>::zero())));
    assert!(!(i.eq(&Matrix4x4::<f32>::zero())));
    assert!(!(j.eq(&Matrix4x4::<f32>::zero())));
    assert!(!(k.eq(&Matrix4x4::<f32>::zero())));
    assert!(!(l.eq(&Matrix4x4::<f32>::zero())));
    assert!(!(m.eq(&Matrix4x4::<f32>::zero())));
    assert!(!(n.eq(&Matrix4x4::<f32>::zero())));
    assert!(!(o.eq(&Matrix4x4::<f32>::zero())));
    assert!(!(p.eq(&Matrix4x4::<f32>::zero())));

    assert!(!(a.is_identity()));
    assert!(!(b.is_identity()));
    assert!(!(c.is_identity()));
    assert!(!(d.is_identity()));
    assert!(!(e.is_identity()));
    assert!(!(f.is_identity()));
    assert!(!(g.is_identity()));
    assert!(!(h.is_identity()));
    assert!(!(i.is_identity()));
    assert!(!(j.is_identity()));
    assert!(!(k.is_identity()));
    assert!(!(l.is_identity()));
    assert!(!(m.is_identity()));
    assert!(!(n.is_identity()));
    assert!(!(o.is_identity()));
    assert!(!(p.is_identity()));

    // Counterintuitive result - IEEE rules for NaN comparison are weird!
    assert!(!(a.eq(&a)));
    assert!(!(b.eq(&b)));
    assert!(!(c.eq(&c)));
    assert!(!(d.eq(&d)));
    assert!(!(e.eq(&e)));
    assert!(!(f.eq(&f)));
    assert!(!(g.eq(&g)));
    assert!(!(h.eq(&h)));
    assert!(!(i.eq(&i)));
    assert!(!(j.eq(&j)));
    assert!(!(k.eq(&k)));
    assert!(!(l.eq(&l)));
    assert!(!(m.eq(&m)));
    assert!(!(n.eq(&n)));
    assert!(!(o.eq(&o)));
    assert!(!(p.eq(&p)));
}

#[test]
fn perspective_far_plane_at_infinity_test() {
    let near_plane_distance = 0.125f32;
    let m = Matrix4x4::create_perspective(1.0f32, 1.0, near_plane_distance, f32::INFINITY);
    assert!(equal(-1.0f32, m.m33));
    assert!(equal(-near_plane_distance, m.m43));
}

#[test]
fn perspective_field_of_view_far_plane_at_infinity_test() {
    let near_plane_distance = 0.125f32;
    let m = Matrix4x4::create_perspective_field_of_view(
        60.0f32.to_radians(),
        1.5f32,
        near_plane_distance,
        f32::INFINITY,
    );
    assert!(equal(-1.0, m.m33));
    assert!(equal(-near_plane_distance, m.m43));
}

#[test]
fn perspective_off_center_far_plane_at_infinity_test() {
    let near_plane_distance = 0.125f32;
    let m = Matrix4x4::create_perspective_off_center(
        0.0f32,
        1.0,
        1.0,
        0.0,
        near_plane_distance,
        f32::INFINITY,
    );
    assert!(equal(-1.0, m.m33));
    assert!(equal(-near_plane_distance, m.m43));
}
