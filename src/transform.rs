use scalar::{
    Scalar,
    ScalarFloat,
};
use matrix::{
    Matrix3,
    Matrix4,
};
use vector::{
    Vector2,
    Vector3,
};
use point::{
    Point2,
    Point3,
};
use structure::{
    One,
    InvertibleSquareMatrix,
};

use std::fmt;


/// A trait for implementing two-dimensional affine transformations.
pub trait AffineTransformation2D<P, V, S> where Self: Sized {
    /// The output associated type for points. This allows us to use both pointers and values
    /// on the inputs.
    type OutPoint;
    /// The output associated type for vectors. This allows us to use both pointers and values
    /// on the inputs.
    type OutVector;

    /// The identity transformation for this type.
    fn identity() -> Self;

    /// Compute the inverse of an affine transformation.
    fn inverse(&self) -> Option<Self>;

    /// Apply the affine transformation to a vector.
    fn apply_vector(&self, vector: V) -> Self::OutVector;

    /// Apply the affine transformation to a point.
    fn apply_point(&self, point: P) -> Self::OutPoint;

    /// Apply the inverse of the affine transformation to a vector.
    fn apply_inverse_vector(&self, vector: V) -> Option<Self::OutVector> {
        self.inverse()
            .and_then(|matrix_inverse| Some(matrix_inverse.apply_vector(vector)))
    }

    /// Convert a specific two-dimensional affine transformation into a generic 
    /// two-dimensional affine transformation.
    fn to_transform2d(&self) -> Transform2D<S>;
}

/// A trait for implementing three-dimensional affine transformations.
pub trait AffineTransformation3D<P, V, S> where Self: Sized {
    /// The output associated type for points. This allows us to use both pointers and values
    /// on the inputs.
    type OutPoint;
    /// The output associated type for vectors. This allows us to use both pointers and values
    /// on the inputs.
    type OutVector;

    /// The identity transformation for this type.
    fn identity() -> Self;

    /// Compute the inverse of an affine transformation.
    fn inverse(&self) -> Option<Self>;

    /// Apply the affine transformation to a vector.
    fn apply_vector(&self, vector: V) -> Self::OutVector;

    /// Apply the affine transformation to a point.
    fn apply_point(&self, point: P) -> Self::OutPoint;

    /// Apply the inverse of the affine transformation to a vector.
    fn apply_inverse_vector(&self, vector: V) -> Option<Self::OutVector> {
        self.inverse()
            .and_then(|matrix_inverse| Some(matrix_inverse.apply_vector(vector)))
    }

    /// Convert a specific three-dimensional affine transformation into a generic 
    /// three-dimensional affine transformation.
    fn to_transform3d(&self) -> Transform3D<S>;
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform2D<S> {
    matrix: Matrix3<S>,
}

impl<S> Transform2D<S> where S: Scalar {
    /// Convert a 3x3 matrix to a two-dimensional affine transformation. This function
    /// is primarily for internal use in implementing type conversions for affine 
    /// transformations.
    #[inline]
    pub(crate) fn matrix_to_transform2d(matrix: Matrix3<S>) -> Transform2D<S> {
        // TODO: Make this function const when const fn stabilizes for traits other than
        // Sized. See issue #57563: <https://github.com/rust-lang/rust/issues/57563>
        Transform2D {
            matrix: matrix,
        }
    }
}

impl<S> AsRef<Matrix3<S>> for Transform2D<S> {
    #[inline]
    fn as_ref(&self) -> &Matrix3<S> {
        &self.matrix
    }
}

impl<S> fmt::Display for Transform2D<S> where S: Scalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(&self, f)
    }
}

impl<S> From<Transform2D<S>> for Matrix3<S> where S: Copy {
    fn from(transformation: Transform2D<S>) -> Matrix3<S> {
        transformation.matrix
    }
}

impl<S> From<&Transform2D<S>> for Matrix3<S> where S: Copy {
    fn from(transformation: &Transform2D<S>) -> Matrix3<S> {
        transformation.matrix
    }
}

impl<S> AffineTransformation2D<Point2<S>, Vector2<S>, S> for Transform2D<S> where S: ScalarFloat {
    type OutPoint = Point2<S>;
    type OutVector = Vector2<S>;

    #[inline]
    fn identity() -> Transform2D<S> {
        Transform2D { 
            matrix: Matrix3::one(),
        }
    }

    #[inline]
    fn inverse(&self) -> Option<Transform2D<S>> {
        if let Some(matrix) = self.matrix.inverse() {
            Some(Transform2D {
                matrix: matrix
            })
        } else {
            None
        }
    }

    #[inline]
    fn apply_vector(&self, vector: Vector2<S>) -> Vector2<S> {
        (self.matrix * vector.expand(S::zero())).contract()
    }

    #[inline]
    fn apply_point(&self, point: Point2<S>) -> Point2<S> {
        Point2::from_homogeneous(self.matrix * point.to_homogeneous())
    }

    #[inline]
    fn to_transform2d(&self) -> Transform2D<S> {
        *self
    }
}

impl<S> AffineTransformation2D<Point2<S>, &Vector2<S>, S> for Transform2D<S> where S: ScalarFloat {
    type OutPoint = Point2<S>;
    type OutVector = Vector2<S>;

    #[inline]
    fn identity() -> Transform2D<S> {
        Transform2D { 
            matrix: Matrix3::one(),
        }
    }

    #[inline]
    fn inverse(&self) -> Option<Transform2D<S>> {
        if let Some(matrix) = self.matrix.inverse() {
            Some(Transform2D {
                matrix: matrix
            })
        } else {
            None
        }
    }

    #[inline]
    fn apply_vector(&self, vector: &Vector2<S>) -> Vector2<S> {
        (self.matrix * vector.expand(S::zero())).contract()
    }

    #[inline]
    fn apply_point(&self, point: Point2<S>) -> Point2<S> {
        Point2::from_homogeneous(self.matrix * point.to_homogeneous())
    }

    #[inline]
    fn to_transform2d(&self) -> Transform2D<S> {
        *self
    }
}

impl<S> AffineTransformation2D<&Point2<S>, Vector2<S>, S> for Transform2D<S> where S: ScalarFloat {
    type OutPoint = Point2<S>;
    type OutVector = Vector2<S>;

    #[inline]
    fn identity() -> Transform2D<S> {
        Transform2D { 
            matrix: Matrix3::one(),
        }
    }

    #[inline]
    fn inverse(&self) -> Option<Transform2D<S>> {
        if let Some(matrix) = self.matrix.inverse() {
            Some(Transform2D {
                matrix: matrix
            })
        } else {
            None
        }
    }

    #[inline]
    fn apply_vector(&self, vector: Vector2<S>) -> Vector2<S> {
        (self.matrix * vector.expand(S::zero())).contract()
    }

    #[inline]
    fn apply_point(&self, point: &Point2<S>) -> Point2<S> {
        Point2::from_homogeneous(self.matrix * point.to_homogeneous())
    }

    #[inline]
    fn to_transform2d(&self) -> Transform2D<S> {
        *self
    }
}

impl<'a, 'b, S> AffineTransformation2D<&'a Point2<S>, &'b Vector2<S>, S> for Transform2D<S> where S: ScalarFloat {
    type OutPoint = Point2<S>;
    type OutVector = Vector2<S>;

    #[inline]
    fn identity() -> Transform2D<S> {
        Transform2D { 
            matrix: Matrix3::one(),
        }
    }

    #[inline]
    fn inverse(&self) -> Option<Transform2D<S>> {
        if let Some(matrix) = self.matrix.inverse() {
            Some(Transform2D {
                matrix: matrix
            })
        } else {
            None
        }
    }

    #[inline]
    fn apply_vector(&self, vector: &'b Vector2<S>) -> Vector2<S> {
        (self.matrix * vector.expand(S::zero())).contract()
    }

    #[inline]
    fn apply_point(&self, point: &'a Point2<S>) -> Point2<S> {
        Point2::from_homogeneous(self.matrix * point.to_homogeneous())
    }

    #[inline]
    fn to_transform2d(&self) -> Transform2D<S> {
        *self
    }
}



#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform3D<S> {
    matrix: Matrix4<S>,
}

impl<S> Transform3D<S> where S: Scalar {
    /// Convert a 4x4 matrix to a three-dimensional affine transformation. This function
    /// is primarily for internal use in implementing type conversions for affine 
    /// transformations.
    #[inline]
    pub(crate) fn matrix_to_transform3d(matrix: Matrix4<S>) -> Transform3D<S> {
        // TODO: Make this function const when const fn stabilizes for traits other than
        // Sized. See issue #57563: <https://github.com/rust-lang/rust/issues/57563>.
        Transform3D {
            matrix: matrix,
        }
    }
}

impl<S> AsRef<Matrix4<S>> for Transform3D<S> {
    #[inline]
    fn as_ref(&self) -> &Matrix4<S> {
        &self.matrix
    }
}

impl<S> fmt::Display for Transform3D<S> where S: Scalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(&self, f)
    }
}

impl<S> From<Transform3D<S>> for Matrix4<S> where S: Copy {
    fn from(transformation: Transform3D<S>) -> Matrix4<S> {
        transformation.matrix
    }
}

impl<S> From<&Transform3D<S>> for Matrix4<S> where S: Copy {
    fn from(transformation: &Transform3D<S>) -> Matrix4<S> {
        transformation.matrix
    }
}

impl<S> AffineTransformation3D<Point3<S>, Vector3<S>, S> for Transform3D<S> where S: ScalarFloat {
    type OutPoint = Point3<S>;
    type OutVector = Vector3<S>;

    #[inline]
    fn identity() -> Transform3D<S> {
        Transform3D { 
            matrix: Matrix4::one(),
        }
    }

    #[inline]
    fn inverse(&self) -> Option<Transform3D<S>> {
        if let Some(matrix) = self.matrix.inverse() {
            Some(Transform3D {
                matrix: matrix
            })
        } else {
            None
        }
    }

    #[inline]
    fn apply_vector(&self, vector: Vector3<S>) -> Vector3<S> {
        (self.matrix * vector.extend(S::zero())).contract()
    }

    #[inline]
    fn apply_point(&self, point: Point3<S>) -> Point3<S> {
        Point3::from_homogeneous(self.matrix * point.to_homogeneous())
    }

    #[inline]
    fn to_transform3d(&self) -> Transform3D<S> {
        *self
    }
}

impl<S> AffineTransformation3D<Point3<S>, &Vector3<S>, S> for Transform3D<S> where S: ScalarFloat {
    type OutPoint = Point3<S>;
    type OutVector = Vector3<S>;

    #[inline]
    fn identity() -> Transform3D<S> {
        Transform3D { 
            matrix: Matrix4::one(),
        }
    }

    #[inline]
    fn inverse(&self) -> Option<Transform3D<S>> {
        if let Some(matrix) = self.matrix.inverse() {
            Some(Transform3D {
                matrix: matrix
            })
        } else {
            None
        }
    }

    #[inline]
    fn apply_vector(&self, vector: &Vector3<S>) -> Vector3<S> {
        (self.matrix * vector.extend(S::zero())).contract()
    }

    #[inline]
    fn apply_point(&self, point: Point3<S>) -> Point3<S> {
        Point3::from_homogeneous(self.matrix * point.to_homogeneous())
    }

    #[inline]
    fn to_transform3d(&self) -> Transform3D<S> {
        *self
    }
}

impl<S> AffineTransformation3D<&Point3<S>, Vector3<S>, S> for Transform3D<S> where S: ScalarFloat {
    type OutPoint = Point3<S>;
    type OutVector = Vector3<S>;

    #[inline]
    fn identity() -> Transform3D<S> {
        Transform3D { 
            matrix: Matrix4::one(),
        }
    }

    #[inline]
    fn inverse(&self) -> Option<Transform3D<S>> {
        if let Some(matrix) = self.matrix.inverse() {
            Some(Transform3D {
                matrix: matrix
            })
        } else {
            None
        }
    }

    #[inline]
    fn apply_vector(&self, vector: Vector3<S>) -> Vector3<S> {
        (self.matrix * vector.extend(S::zero())).contract()
    }

    #[inline]
    fn apply_point(&self, point: &Point3<S>) -> Point3<S> {
        Point3::from_homogeneous(self.matrix * point.to_homogeneous())
    }

    #[inline]
    fn to_transform3d(&self) -> Transform3D<S> {
        *self
    }
}

impl<'a, 'b, S> AffineTransformation3D<&'a Point3<S>, &'b Vector3<S>, S> for Transform3D<S> where S: ScalarFloat {
    type OutPoint = Point3<S>;
    type OutVector = Vector3<S>;

    #[inline]
    fn identity() -> Transform3D<S> {
        Transform3D { 
            matrix: Matrix4::one(),
        }
    }

    #[inline]
    fn inverse(&self) -> Option<Transform3D<S>> {
        if let Some(matrix) = self.matrix.inverse() {
            Some(Transform3D {
                matrix: matrix
            })
        } else {
            None
        }
    }

    #[inline]
    fn apply_vector(&self, vector: &'b Vector3<S>) -> Vector3<S> {
        (self.matrix * vector.extend(S::zero())).contract()
    }

    #[inline]
    fn apply_point(&self, point: &'a Point3<S>) -> Point3<S> {
        Point3::from_homogeneous(self.matrix * point.to_homogeneous())
    }

    #[inline]
    fn to_transform3d(&self) -> Transform3D<S> {
        *self
    }
}


