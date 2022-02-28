use crate::base::{
    Magnitude,
    Scalar,
    ScalarSigned,
    ScalarFloat,
};
use crate::angle::{
    Angle,
    Radians,
};
use core::ops;


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Complex<S> {
    pub re: S,
    pub im: S,
}

impl<S> Complex<S> {
    #[inline]
    pub const fn new(re: S, im: S) -> Self {
        Self {
            re: re,
            im: im,
        }   
    }

    #[inline]
    pub fn shape(&self) -> (usize, usize) {
        (2, 1)
    }

    /// Get a pointer to the underlying array.
    #[inline]
    pub fn as_ptr(&self) -> *const S {
        &self.re
    }

    /// Get a mutable pointer to the underlying array.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut S {
        &mut self.re
    }

    /// Get a slice of the underlying elements of the data type.
    #[inline]
    pub fn as_slice(&self) -> &[S] {
        <Self as AsRef<[S; 2]>>::as_ref(self)
    }
}

impl<S> Complex<S>
where
    S: Scalar
{
    #[inline]
    pub fn i() -> Self {
        Self::new(S::zero(), S::one())
    }

    #[inline]
    pub fn from_real(value: S) -> Self {
        Self::new(value, S::zero())
    }

    #[inline]
    pub fn from_imaginary(value: S) -> Self {
        Self::new(S::zero(), value)
    }

    #[inline]
    pub fn zero() -> Self {
        Self::new(S::zero(), S::zero())
    }

    #[inline]
    pub fn identity() -> Self {
        Self::new(S::one(), S::zero())
    }

    #[inline]
    pub fn magnitude_squared(self) -> S {
        self.re.clone() * self.re.clone() + self.im.clone() * self.im.clone()
    }
}

impl<S> Complex<S>
where
    S: ScalarSigned
{
    #[inline]
    pub fn conjugate(self) -> Self {
        Self::new(self.re.clone(), -self.im.clone())
    }

    #[inline]
    pub fn scale(self, scale: S) -> Self {
        Self::new(
            self.re.clone() * scale.clone(),
            self.im.clone() * scale
        )
    }

    #[inline]
    pub fn unscale(self, scale: S) -> Self {
        let one_over_scale = S::one() / scale;

        Self::new(
            self.re.clone() * one_over_scale.clone(), 
            self.im.clone() * one_over_scale
        )
    }

    #[inline]
    pub fn inverse(self) -> Self {
        let magnitude_squared = self.magnitude_squared();
        Self::new(
             self.re.clone() / magnitude_squared.clone(),
            -self.im.clone() / magnitude_squared
        )
    }

    #[inline]
    pub fn powi(self, power: i32) -> Self {
        unimplemented!()
    }

    #[inline]
    pub fn powu(self, power: u32) -> Self {
        unimplemented!()
    }
}

impl<S> Complex<S>
where
    S: ScalarFloat
{
    #[inline]
    pub fn magnitude(self) -> S {
        self.magnitude_squared().sqrt()
    }

    #[inline]
    pub fn arg(self) -> S {
        self.im.atan2(self.re)
    }

    #[inline]
    pub fn from_polar_decomposition<A: Into<Radians<S>>>(radius: S, angle: A) -> Self {
        let _angle: Radians<S> = angle.into();
        Self::new(radius * _angle.cos(), radius * _angle.sin())
    }

    #[inline]
    pub fn from_angle<A: Into<Radians<S>>>(angle: A) -> Self {
        Self::from_polar_decomposition(S::one(), angle)
    }

    #[inline]
    pub fn polar_decomposition(self) -> (S, Radians<S>) {
        (self.magnitude(), Radians(self.arg()))
    }

    #[inline]
    pub fn exp(self) -> Self {
        let exp_re = self.re.exp();
        let (sin_im, cos_im) = self.im.sin_cos();

        Self::new(exp_re * cos_im, exp_re * sin_im)
    }

    /// Calculate the principal value of the natural logarithm of a complex number.
    #[inline]
    pub fn ln(self) -> Self {
        let magnitude_self = self.magnitude();
        let arg_self = self.arg();

        Self::new(magnitude_self.ln(), arg_self)
    }

    /// Calculate the principal value of the square root of a complex number.
    #[inline]
    pub fn sqrt(self) -> Self {
        let two = S::one() + S::one();
        let magnitude = self.magnitude();
        let angle = self.arg();
        let (sin_angle_over_two, cos_angle_over_two) = (angle / two).sin_cos();

        Self::new(magnitude * cos_angle_over_two, magnitude * sin_angle_over_two)
    }
}

impl<S> AsRef<[S; 2]> for Complex<S> {
    #[inline]
    fn as_ref(&self) -> &[S; 2] {
        unsafe { 
            &*(self as *const Complex<S> as *const [S; 2])
        }
    }
}

impl<S> AsRef<(S, S)> for Complex<S> {
    #[inline]
    fn as_ref(&self) -> &(S, S) {
        unsafe { 
            &*(self as *const Complex<S> as *const (S, S))
        }
    }
}

impl<S> AsMut<[S; 2]> for Complex<S> {
    #[inline]
    fn as_mut(&mut self) -> &mut [S; 2] {
        unsafe { 
            &mut *(self as *mut Complex<S> as *mut [S; 2])
        }
    }
}

impl<S> AsMut<(S, S)> for Complex<S> {
    #[inline]
    fn as_mut(&mut self) -> &mut (S, S) {
        unsafe { 
            &mut *(self as *mut Complex<S> as *mut (S, S))
        }
    }
}

impl<S> ops::Index<usize> for Complex<S> 
where 
    S: Scalar 
{
    type Output = S;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let v: &[S; 2] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::Range<usize>> for Complex<S> 
where 
    S: Scalar 
{
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::Range<usize>) -> &Self::Output {
        let v: &[S; 2] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::RangeTo<usize>> for Complex<S> 
where 
    S: Scalar 
{
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::RangeTo<usize>) -> &Self::Output {
        let v: &[S; 2] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::RangeFrom<usize>> for Complex<S> 
where 
    S: Scalar 
{
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::RangeFrom<usize>) -> &Self::Output {
        let v: &[S; 2] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::RangeFull> for Complex<S> 
where 
    S: Scalar 
{
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::RangeFull) -> &Self::Output {
        let v: &[S; 2] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::IndexMut<usize> for Complex<S> 
where 
    S: Scalar 
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut S {
        let v: &mut [S; 2] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::Range<usize>> for Complex<S> 
where 
    S: Scalar 
{
    #[inline]
    fn index_mut(&mut self, index: ops::Range<usize>) -> &mut [S] {
        let v: &mut [S; 2] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::RangeTo<usize>> for Complex<S> 
where 
    S: Scalar 
{
    #[inline]
    fn index_mut(&mut self, index: ops::RangeTo<usize>) -> &mut [S] {
        let v: &mut [S; 2] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::RangeFrom<usize>> for Complex<S> 
where 
    S: Scalar 
{
    #[inline]
    fn index_mut(&mut self, index: ops::RangeFrom<usize>) -> &mut [S] {
        let v: &mut [S; 2] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::RangeFull> for Complex<S> 
where 
    S: Scalar 
{
    #[inline]
    fn index_mut(&mut self, index: ops::RangeFull) -> &mut [S] {
        let v: &mut [S; 2] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::Add<Complex<S>> for Complex<S>
where
    S: Scalar
{
    type Output = Complex<S>;

    #[inline]
    fn add(self, other: Complex<S>) -> Self::Output {
        Self::Output::new(self.re + other.re, self.im + other.im)
    }
}

impl<S> ops::Add<&Complex<S>> for Complex<S>
where
    S: Scalar
{
    type Output = Complex<S>;

    #[inline]
    fn add(self, other: &Complex<S>) -> Self::Output {
        Self::Output::new(self.re + other.re, self.im + other.im)
    }
}

impl<S> ops::Add<Complex<S>> for &Complex<S>
where
    S: Scalar
{
    type Output = Complex<S>;

    #[inline]
    fn add(self, other: Complex<S>) -> Self::Output {
        Self::Output::new(self.re + other.re, self.im + other.im)
    }
}

impl<'a, 'b, S> ops::Add<&'b Complex<S>> for &'a Complex<S>
where
    S: Scalar
{
    type Output = Complex<S>;

    #[inline]
    fn add(self, other: &'b Complex<S>) -> Self::Output {
        Self::Output::new(self.re + other.re, self.im + other.im)
    }
}

impl<S> ops::Add<S> for Complex<S>
where
    S: Scalar
{
    type Output = Complex<S>;

    #[inline]
    fn add(self, other: S) -> Self::Output {
        Self::Output::new(self.re + other, self.im)
    }
}

impl<S> ops::Add<&S> for Complex<S>
where
    S: Scalar
{
    type Output = Complex<S>;

    #[inline]
    fn add(self, other: &S) -> Self::Output {
        Self::Output::new(self.re + *other, self.im)
    }
}

impl<S> ops::Add<S> for &Complex<S>
where
    S: Scalar
{
    type Output = Complex<S>;

    #[inline]
    fn add(self, other: S) -> Self::Output {
        Self::Output::new(self.re + other, self.im)
    }
}

impl<'a, 'b, S> ops::Add<&'b S> for &'a Complex<S>
where
    S: Scalar
{
    type Output = Complex<S>;

    #[inline]
    fn add(self, other: &'b S) -> Self::Output {
        Self::Output::new(self.re + *other, self.im)
    }
}

impl<S> ops::Sub<Complex<S>> for Complex<S>
where
    S: Scalar
{
    type Output = Complex<S>;

    #[inline]
    fn sub(self, other: Complex<S>) -> Self::Output {
        Self::Output::new(self.re - other.re, self.im - other.im)
    }
}

impl<S> ops::Sub<&Complex<S>> for Complex<S>
where
    S: Scalar
{
    type Output = Complex<S>;

    #[inline]
    fn sub(self, other: &Complex<S>) -> Self::Output {
        Self::Output::new(self.re - other.re, self.im - other.im)
    }
}

impl<S> ops::Sub<Complex<S>> for &Complex<S>
where
    S: Scalar
{
    type Output = Complex<S>;

    #[inline]
    fn sub(self, other: Complex<S>) -> Self::Output {
        Self::Output::new(self.re - other.re, self.im - other.im)
    }
}

impl<'a, 'b, S> ops::Sub<&'b Complex<S>> for &'a Complex<S>
where
    S: Scalar
{
    type Output = Complex<S>;

    #[inline]
    fn sub(self, other: &'b Complex<S>) -> Self::Output {
        Self::Output::new(self.re - other.re, self.im - other.im)
    }
}

impl<S> ops::Sub<S> for Complex<S>
where
    S: Scalar
{
    type Output = Complex<S>;

    #[inline]
    fn sub(self, other: S) -> Self::Output {
        Self::Output::new(self.re - other, self.im)
    }
}

impl<S> ops::Sub<&S> for Complex<S>
where
    S: Scalar
{
    type Output = Complex<S>;

    #[inline]
    fn sub(self, other: &S) -> Self::Output {
        Self::Output::new(self.re - *other, self.im)
    }
}

impl<S> ops::Sub<S> for &Complex<S>
where
    S: Scalar
{
    type Output = Complex<S>;

    #[inline]
    fn sub(self, other: S) -> Self::Output {
        Self::Output::new(self.re - other, self.im)
    }
}

impl<'a, 'b, S> ops::Sub<&'b S> for &'a Complex<S>
where
    S: Scalar
{
    type Output = Complex<S>;

    #[inline]
    fn sub(self, other: &'b S) -> Self::Output {
        Self::Output::new(self.re - *other, self.im)
    }
}
