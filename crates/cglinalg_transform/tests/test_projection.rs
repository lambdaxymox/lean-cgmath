extern crate cglinalg_transform;


use cglinalg_core::{
    Matrix4x4,
    Angle,
    Degrees,
    Radians,
    Point3,
    Vector3,
};
use cglinalg_transform::{
    Orthographic3,
    OrthographicFov3,
    Perspective3,
    PerspectiveFov3,
};
use approx::{
    assert_relative_eq,
};


#[rustfmt::skip]
#[test]
fn test_perspective_projection_matrix() {
    let left = -4_f64;
    let right = 4_f64;
    let bottom = -2_f64;
    let top = 3_f64;
    let near = 1_f64;
    let far = 100_f64;
    let expected = Matrix4x4::new(
        1_f64 / 4_f64, 0_f64,          0_f64,             0_f64,
        0_f64,         2_f64 / 5_f64,  0_f64,             0_f64,
        0_f64,         1_f64 / 5_f64, -101_f64 / 99_f64, -1_f64,
        0_f64,         0_f64,         -200_f64 / 99_f64,  0_f64
    );
    let result = Matrix4x4::from_perspective(left, right, bottom, top, near, far);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_perspective_projection_transformation() {
    let left = -4_f64;
    let right = 4_f64;
    let bottom = -2_f64;
    let top = 3_f64;
    let near = 1_f64;
    let far = 100_f64;
    let expected = Matrix4x4::new(
        1_f64 / 4_f64, 0_f64,          0_f64,             0_f64,
        0_f64,         2_f64 / 5_f64,  0_f64,             0_f64,
        0_f64,         1_f64 / 5_f64, -101_f64 / 99_f64, -1_f64,
        0_f64,         0_f64,         -200_f64 / 99_f64,  0_f64
    );
    let result = Perspective3::new(left, right, bottom, top, near, far);

    assert_eq!(result.matrix(), &expected);
}

#[rustfmt::skip]
#[test]
fn test_perspective_projection_fov_matrix() {
    let vfov = Degrees(72_f32);
    let aspect = 800_f32 / 600_f32;
    let near = 0.1_f32;
    let far = 100_f32;
    let expected = Matrix4x4::new(
        1.0322863_f32, 0_f32,         0_f32,         0_f32, 
        0_f32,         1.3763818_f32, 0_f32,         0_f32, 
        0_f32,         0_f32,        -1.002002_f32, -1_f32, 
        0_f32,         0_f32,        -0.2002002_f32, 0_f32
    );
    let result = Matrix4x4::from_perspective_fov(vfov, aspect, near, far);

    assert_relative_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_perspective_projection_fov_transformation() {
    let vfov = Degrees(72_f32);
    let aspect = 800_f32 / 600_f32;
    let near = 0.1_f32;
    let far = 100_f32;
    let expected = Matrix4x4::new(
        1.0322863_f32, 0_f32,         0_f32,         0_f32, 
        0_f32,         1.3763818_f32, 0_f32,         0_f32, 
        0_f32,         0_f32,        -1.002002_f32, -1_f32, 
        0_f32,         0_f32,        -0.2002002_f32, 0_f32
    );
    let result = PerspectiveFov3::new(vfov, aspect, near, far);

    assert_relative_eq!(result.matrix(), &expected, epsilon = 1e-10);
}

#[test]
fn test_perspective_projection_unproject_point1() {
    let vfov = Degrees(72_f64);
    let aspect = 800_f64 / 600_f64;
    let near = 0.1_f64;
    let far = 100_f64;
    let point = Point3::new(-2_f64, 2_f64, -50_f64);
    let projection = PerspectiveFov3::new(vfov, aspect, near, far);
    let expected = point;
    let projected_point = projection.project_point(&expected);
    let result = projection.unproject_point(&projected_point);

    assert_relative_eq!(result, expected, epsilon = 1e-8);
}

#[test]
fn test_perspective_projection_unproject_vector1() {
    let vfov = Degrees(72_f64);
    let aspect = 800_f64 / 600_f64;
    let near = 0.1_f64;
    let far = 100_f64;
    let vector = Vector3::new(-2_f64, 2_f64, -50_f64);
    let projection = PerspectiveFov3::new(vfov, aspect, near, far);
    let expected = vector;
    let projected_vector = projection.project_vector(&expected);
    let result = projection.unproject_vector(&projected_vector);

    assert_relative_eq!(result, expected, epsilon = 1e-8);
}

#[test]
fn test_perspective_projection_unproject_point2() {
    let left = -4_f64;
    let right = 4_f64;
    let bottom = -2_f64;
    let top = 2_f64;
    let near = 1_f64;
    let far = 100_f64;
    let projection = Perspective3::new(left, right, bottom, top, near, far);
    let expected = Point3::new(-2_f64, 2_f64, -50_f64);
    let projected_point = projection.project_point(&expected);
    let result = projection.unproject_point(&projected_point);

    assert_relative_eq!(result, expected, epsilon = 1e-8);
}

#[test]
fn test_perspective_projection_unproject_vector2() {
    let left = -4_f64;
    let right = 4_f64;
    let bottom = -2_f64;
    let top = 2_f64;
    let near = 1_f64;
    let far = 100_f64;
    let projection = Perspective3::new(left, right, bottom, top, near, far);
    let expected = Vector3::new(-2_f64, 2_f64, -50_f64);
    let projected_vector = projection.project_vector(&expected);
    let result = projection.unproject_vector(&projected_vector);

    assert_relative_eq!(result, expected, epsilon = 1e-8);
}

#[rustfmt::skip]
#[test]
fn test_orthographic_projection_matrix() {
    let left = -4_f64;
    let right = 4_f64;
    let bottom = -2_f64;
    let top = 2_f64;
    let near = 1_f64;
    let far = 100_f64;
    let expected = Matrix4x4::new(
        1_f64 / 4_f64, 0_f64,          0_f64,            0_f64,
        0_f64,         1_f64 / 2_f64,  0_f64,            0_f64,
        0_f64,         0_f64,         -2_f64 / 99_f64,   0_f64,
        0_f64,         0_f64,         -101_f64 / 99_f64, 1_f64
    );
    let result = Matrix4x4::from_orthographic(left, right, bottom, top, near, far);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_orthographic_projection_transformation() {
    let left = -4_f64;
    let right = 4_f64;
    let bottom = -2_f64;
    let top = 2_f64;
    let near = 1_f64;
    let far = 100_f64;
    let expected = Matrix4x4::new(
        1_f64 / 4_f64, 0_f64,          0_f64,            0_f64,
        0_f64,         1_f64 / 2_f64,  0_f64,            0_f64,
        0_f64,         0_f64,         -2_f64 / 99_f64,   0_f64,
        0_f64,         0_f64,         -101_f64 / 99_f64, 1_f64
    );
    let result = Orthographic3::new(left, right, bottom, top, near, far);

    assert_eq!(result.matrix(), &expected);
}

#[test]
fn test_orthographic_projection_unproject_point() {
    let left = -4_f64;
    let right = 4_f64;
    let bottom = -2_f64;
    let top = 2_f64;
    let near = 1_f64;
    let far = 100_f64;
    let projection = Orthographic3::new(left, right, bottom, top, near, far);
    let expected = Point3::new(1_f64, 1_f64, 50_f64);
    let projected_point = projection.project_point(&expected);
    let result = projection.unproject_point(&projected_point);

    assert_eq!(result, expected);
}

#[test]
fn test_orthographic_projection_unproject_vector() {
    let left = -4_f64;
    let right = 4_f64;
    let bottom = -2_f64;
    let top = 2_f64;
    let near = 1_f64;
    let far = 100_f64;
    let projection = Orthographic3::new(left, right, bottom, top, near, far);
    let expected = Vector3::new(1_f64, 1_f64, 50_f64);
    let projected_vector = projection.project_vector(&expected);
    let result = projection.unproject_vector(&projected_vector);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_orthographic_fov_projection_matrix() {
    let aspect = 2_f64;
    // 9.1478425198 Degrees.
    let vfov = Degrees::from(Radians::atan2(8_f64, 100_f64) * 2_f64);
    let near = 1_f64;
    let far = 100_f64;
    let expected = Matrix4x4::new(
        1_f64 / 4_f64,  0_f64,          0_f64,            0_f64,
        0_f64,          1_f64 / 2_f64,  0_f64,            0_f64,
        0_f64,          0_f64,         -2_f64 / 99_f64,   0_f64,
        0_f64,          0_f64,         -101_f64 / 99_f64, 1_f64
    );
    let result = Matrix4x4::from_orthographic_fov(vfov, aspect, near, far);

    assert_relative_eq!(result, expected, epsilon = 1e-10);
}

#[rustfmt::skip]
#[test]
fn test_orthographic_fov_projecton_transformation() {
    let aspect = 2_f64;
    // 9.1478425198 Degrees.
    let vfov = Degrees::from(Radians::atan2(8_f64, 100_f64) * 2_f64);
    let near = 1_f64;
    let far = 100_f64;
    let expected = Matrix4x4::new(
        1_f64 / 4_f64,  0_f64,          0_f64,            0_f64,
        0_f64,          1_f64 / 2_f64,  0_f64,            0_f64,
        0_f64,          0_f64,         -2_f64 / 99_f64,   0_f64,
        0_f64,          0_f64,         -101_f64 / 99_f64, 1_f64
    );
    let result = OrthographicFov3::new(vfov, aspect, near, far);

    assert_relative_eq!(result.matrix(), &expected, epsilon = 1e-10);
}

#[test]
fn test_orthographic_fov_projection_unproject_point() {
    let aspect = 2_f64;
    // 9.1478425198 Degrees.
    let vfov = Degrees::from(Radians::atan2(8_f64, 100_f64) * 2_f64);
    let near = 1_f64;
    let far = 100_f64;
    let projection = OrthographicFov3::new(vfov, aspect, near, far);
    let expected = Point3::new(1_f64, 1_f64, 50_f64);
    let projected_point = projection.project_point(&expected);
    let result = projection.unproject_point(&projected_point);

    assert_eq!(result, expected);
}

#[test]
fn test_orthographic_fov_projection_unproject_vector() {
    let aspect = 2_f64;
    // 9.1478425198 Degrees.
    let vfov = Degrees::from(Radians::atan2(8_f64, 100_f64) * 2_f64);
    let near = 1_f64;
    let far = 100_f64;
    let projection = OrthographicFov3::new(vfov, aspect, near, far);
    let expected = Vector3::new(1_f64, 1_f64, 50_f64);
    let projected_vector = projection.project_vector(&expected);
    let result = projection.unproject_vector(&projected_vector);

    assert_eq!(result, expected);
}

