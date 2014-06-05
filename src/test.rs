#[test]
fn boolean_test () {
    assert_eq!(true, false);
}

#[test]
fn int_test () {
    assert_eq!(10, 10.00);
}

#[test]
fn float_test () {
    assert_eq!(10.0, 10f64);
}

struct point3d {
    x: f64,
    y: f64,
    z: f64
}

#[test]
fn point3d_test () {
    let point = point3d {x: 3.333, y:41.12, z:32322.343};
}