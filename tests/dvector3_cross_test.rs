use vim_math3d::Vector3;

// A test for Cross (DVector3, DVector3)
#[test]
fn dvector3_cross_test() {
    let a = Vector3::<f64>::new(1.0, 0.0, 0.0);
    let b =  Vector3::<f64>::new(0.0, 1.0, 0.0);

    let expected =  Vector3::<f64>::new(0.0, 0.0, 1.0);
    let actual = a.cross(b);
    assert!(
        expected.x == actual.x && 
        expected.y == actual.y && 
        expected.z == actual.z, 
        "Vector3f.Cross did not return the expected value."
    );
}