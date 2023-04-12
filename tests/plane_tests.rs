use num_traits::Float;
use vim_math3d::{self, Matrix4x4, Transformable3D, Quaternion, Plane, Vector3, Vector4};

fn equal<T: Float>(a: T, b: T) -> bool { (a - b).abs() < T::from(1e-5).unwrap() }

#[test]
fn plane_equals_test1() {
    let a = Plane::new_from_coordinates(1.0f32, 2.0f32, 3.0, 4.0);
    let b = Plane::new_from_coordinates(1.0f32, 2.0f32, 3.0, 4.0);

    assert_eq!(a, b);

    let c = Plane::new(Vector3::new(10.0f32, b.normal.y, b.normal.z), b.d);
    assert_ne!(b, c);
}

#[test]
fn plane_equality_test() {
    let a = Plane::new_from_coordinates(1.0f32, 2.0, 3.0, 4.0);
    let b = Plane::new_from_coordinates(1.0f32, 2.0, 3.0, 4.0);

    assert_eq!(a, b);

    let b = Plane::new(Vector3::new(10.0f32, b.normal.y, b.normal.z), b.d);
    assert_ne!(a, b);
}

#[test]
fn plane_constructor_test1() {
    let a = 1.0;
    let b = 2.0;
    let c = 3.0;
    let d = 4.0;
    let target = Plane::new_from_coordinates(a, b, c, d);

    assert_eq!(target.normal.x, a);
    assert_eq!(target.normal.y, b);
    assert_eq!(target.normal.z, c);
    assert_eq!(target.d, d);
}

#[test]
fn plane_create_from_vertices_test() {
    let point1 = Vector3::new(0.0f32, 1.0, 1.0);
    let point2 = Vector3::new(0.0f32, 0.0, 1.0);
    let point3 = Vector3::new(1.0f32, 0.0, 1.0);

    let target = Plane::new_from_vertices(point1, point2, point3);
    let expected = Plane::new(Vector3::new(0.0f32, 0.0, 1.0), -1.0f32);
    assert_eq!(target, expected);
}

// #[test]
// fn plane_create_from_vertices_test2() {
//     let point1 = Vector3::new(0.0f32, 0.0, 1.0);
//     let point2 = Vector3::new(1.0f32, 0.0, 0.0);
//     let point3 = Vector3::new(1.0f32, 1.0, 0.0);

//     let target = Plane::new_from_vertices(point1, point2, point3);
//     let inv_root_2 = 1.0f32 / f32::sqrt(2f32);

//     let expected = Plane::new(Vector3::new(inv_root_2, 0.0f32, inv_root_2), -inv_root_2);
//     assert!(equal(target.normal.x, expected.normal.x));
//     assert!(equal(target.normal.y, expected.normal.y));
//     assert!(equal(target.normal.z, expected.normal.z));
//     assert!(equal(target.d, expected.d));
// }

#[test]
fn plane_constructor_test3() {
    let normal = Vector3::new(1.0f32, 2.0, 3.0);
    let d = 4.0f32;

    let target = Plane::new(normal, d);
    assert_eq!(target.normal, normal);
    assert_eq!(target.d, d);
}

#[test]
fn plane_constructor_test() {
    let value = Vector4::new(1.0f32, 2.0, 3.0, 4.0);
    let target = Plane::new_from_vector(value);

    assert_eq!(target.normal.x, value.x);
    assert_eq!(target.normal.y, value.y);
    assert_eq!(target.normal.z, value.z);
    assert_eq!(target.d, value.w);
}

/** */
#[test]
fn plane_dot_test() {
    let target = Plane::new(Vector3::new(2.0f32, 3.0, 4.0), 5.0);
    let value = Vector4::new(5.0f32, 4.0, 3.0, 2.0);

    let expected = 10.0f32 + 12.0 + 12.0 + 10.0;
    let actual = target.dot_vector(value);
    assert!(equal(expected, actual));
}

#[test]
fn plane_dot_coordinate_test() {
    let target = Plane::new(Vector3::new(2.0f32, 3.0, 4.0), 5.0);
    let value = Vector3::new(5.0f32, 4.0, 3.0);

    let expected = 10.0f32 + 12.0 + 12.0 + 5.0;
    let actual = Plane::dot_coordinate(&target, value);
    assert!(equal(expected, actual));
}

#[test]
fn plane_dot_normal_test() {
    let target = Plane::new(Vector3::new(2.0f32, 3.0, 4.0), 5.0);
    let value = Vector3::new(5.0f32, 4.0, 3.0);

    let expected = 10.0f32 + 12.0 + 12.0;
    let actual = target.dot_normal(value);
    assert!(equal(expected, actual));
}

#[test]
fn plane_normalize_test() {
    let target = Plane::new_from_coordinates(1.0f32, 2.0, 3.0, 4.0);

    let f: f32 = target.normal.length_squared();
    let inv_f: f32 = 1.0 / f.sqrt();
    let expected = Plane::new(target.normal * inv_f, target.d * inv_f);

    let actual = target.normalize();
    assert!(equal(expected.normal.x, actual.normal.x));
    assert!(equal(expected.normal.y, actual.normal.y));
    assert!(equal(expected.normal.z, actual.normal.z));
    assert!(equal(expected.d, actual.d));

    let actual = actual.normalize();
    assert!(equal(expected.normal.x, actual.normal.x));
    assert!(equal(expected.normal.y, actual.normal.y));
    assert!(equal(expected.normal.z, actual.normal.z));
    assert!(equal(expected.d, actual.d));
}

#[test]
fn plane_transform_test1() {
    let target = Plane::new_from_coordinates(1.0f32, 2.0, 3.0, 4.0);
    let target = target.normalize();

    let mut m = Matrix4x4::create_rotation_x(30f32.to_radians()) * 
        Matrix4x4::create_rotation_y(30f32.to_radians()) * 
        Matrix4x4::create_rotation_z(30f32.to_radians());
    m.m41 = 10.0f32;
    m.m42 = 20.0f32;
    m.m43 = 30.0f32;

    let itm = m.invert();
    assert!(itm.is_some());
    let itm = itm.unwrap().transpose();
    let x = target.normal.x;
    let y = target.normal.y;
    let z = target.normal.z;
    let w = target.d;
    let normal = Vector3::new(
        x * itm.m11 + y * itm.m21 + z * itm.m31 + w * itm.m41,
        x * itm.m12 + y * itm.m22 + z * itm.m32 + w * itm.m42,
        x * itm.m13 + y * itm.m23 + z * itm.m33 + w * itm.m43);
    let d = x * itm.m14 + y * itm.m24 + z * itm.m34 + w * itm.m44;
    let expected = Plane::new(normal, d);
    let actual = target.transform(m);
    assert!(equal(expected.normal.x, actual.normal.x));
    assert!(equal(expected.normal.y, actual.normal.y));
    assert!(equal(expected.normal.z, actual.normal.z));
    assert!(equal(expected.d, actual.d));
}

#[test]
fn plane_transform_test2() {
    let target = Plane::new_from_coordinates(1.0f32, 2.0, 3.0, 4.0);
    let target = target.normalize();

    let m = Matrix4x4::create_rotation_x(30f32.to_radians()) * 
        Matrix4x4::create_rotation_y(30f32.to_radians()) * 
        Matrix4x4::create_rotation_z(30f32.to_radians());
    let q = Quaternion::create_from_rotation_matrix(m);

    let x = target.normal.x;
    let y = target.normal.y;
    let z = target.normal.z;
    let w = target.d;
    let normal = Vector3::new(
        x * m.m11 + y * m.m21 + z * m.m31 + w * m.m41,
        x * m.m12 + y * m.m22 + z * m.m32 + w * m.m42,
        x * m.m13 + y * m.m23 + z * m.m33 + w * m.m43);
    let d = x * m.m14 + y * m.m24 + z * m.m34 + w * m.m44;

    let expected = Plane::new(normal, d);
    let actual = target.transform_quaternion(q);
    assert!(equal(expected.normal.x, actual.normal.x));
    assert!(equal(expected.normal.y, actual.normal.y));
    assert!(equal(expected.normal.z, actual.normal.z));
    assert!(equal(expected.d, actual.d));
}

#[test]
fn plane_equals_nan_test() {
    let a = Plane::new(Vector3::new(f32::NAN, 0.0, 0.0), 0.0);
    let b = Plane::new(Vector3::new(0.0, f32::NAN, 0.0), 0.0);
    let c = Plane::new(Vector3::new(0.0, 0.0, f32::NAN), 0.0);
    let d = Plane::new(Vector3::new(0.0, 0.0, 0.0), f32::NAN);

    let plane_zero = Plane::new(Vector3::new(0.0f32, 0.0, 0.0), 0.0f32);

    assert!(a != plane_zero);
    assert!(b != plane_zero);
    assert!(c != plane_zero);
    assert!(d != plane_zero);

    assert_ne!(a, plane_zero);
    assert_ne!(b, plane_zero);
    assert_ne!(c, plane_zero);
    assert_ne!(d, plane_zero);

    assert!(a.ne(&plane_zero));
    assert!(b.ne(&plane_zero));
    assert!(c.ne(&plane_zero));
    assert!(d.ne(&plane_zero));

    assert_ne!(a, a);
    assert_ne!(b, b);
    assert_ne!(c, c);
    assert_ne!(d, d);
}
