use num_traits::Float;
use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};
use vim_math3d::{self, Vector2, Vector3, Vector4, Matrix4x4, Transformable3D, Quaternion};

fn equal<T: Float>(a: T, b: T) -> bool { (a - b).abs() < T::from(1e-5).unwrap() }
fn equal_tolerance<T: Float>(a: T, b: T, t: T) -> bool { (a - b).abs() < t }

fn hash_code<T: Float>(v: &Vector4<T>) -> u64 {
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    hasher.finish()
}

fn generate_matrix_number_from_1_to_16() -> Matrix4x4<f32> {
    Matrix4x4::<f32>::new(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    )
}

fn generate_test_matrix() -> Matrix4x4<f32> {
    let m = Matrix4x4::create_rotation_x(30.0f32.to_radians())
        * Matrix4x4::create_rotation_y(30.0f32.to_radians())
        * Matrix4x4::create_rotation_z(30.0f32.to_radians());
    m * m.set_translation(Vector3::new(111.0f32, 222.0, 333.0))
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
    let mtx = Matrix4x4::create_scale_xyz(23.0f32, 42.0, -666.0);
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
        Matrix4x4::create_from_yaw_pitch_roll(3.0f32, 4.0, 5.0) *
        Matrix4x4::create_scale_xyz(23.0f32, 42.0, -666.0) *
        Matrix4x4::create_translation_xyz(17.0f32, 53.0, 89.0);
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
    decompose_test(10.0f32, 20.0, 30.0, Vector3::new(10.0f32, 20.0, 30.0), Vector3::new(2.0, 3.0, 4.0));
    let step: usize = 35;
    for yaw_angle in (-720..=720).step_by(step) {
        for pitch_angle in (-720..=720).step_by(step) {
            for roll_angle in (-720..=720).step_by(step) {
                decompose_test(yaw_angle as f32, pitch_angle as f32, roll_angle as f32, Vector3::new(10.0f32, 20.0, 30.0), Vector3::new(2.0f32, 3.0, 4.0));
            }
        }
    }
}

// Various scaled matrix decompose test.
#[test]
fn matrix4x4_decompose_test02() {
    decompose_test(10.0f32, 20.0f32, 30.0, Vector3::new(10.0f32, 20.0, 30.0), Vector3::new(2.0f32, 3.0, 4.0));

    // Various scales.
    decompose_test(0.0f32, 0.0, 0.0, Vector3::zero(), Vector3::new(1.0f32, 2.0, 3.0));
    decompose_test(0.0f32, 0.0, 0.0, Vector3::zero(), Vector3::new(1.0f32, 3.0, 2.0));
    decompose_test(0.0f32, 0.0, 0.0, Vector3::zero(), Vector3::new(2.0f32, 1.0, 3.0));
    decompose_test(0.0f32, 0.0, 0.0, Vector3::zero(), Vector3::new(2.0f32, 3.0, 1.0));
    decompose_test(0.0f32, 0.0, 0.0, Vector3::zero(), Vector3::new(3.0f32, 1.0, 2.0));
    decompose_test(0.0f32, 0.0, 0.0, Vector3::zero(), Vector3::new(3.0f32, 2.0, 1.0));

    decompose_test(0.0f32, 0.0, 0.0, Vector3::zero(), Vector3::new(-2.0f32, 1.0, 1.0));

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