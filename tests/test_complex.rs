extern crate cglinalg;


#[cfg(test)]
mod storage_tests {
    use cglinalg::{
        Complex,
    };


    #[test]
    fn test_as_ref() {
        let z = Complex::new(1_i32, 2_i32);
        let z_ref: &[i32; 2] = z.as_ref();

        assert_eq!(z_ref, &[1_i32, 2_i32]);
    }

    #[test]
    fn test_indices_match_components() {
        let z = Complex::new(1_i32, 2_i32);

        // assert_eq!(z[0], z.re);
        // assert_eq!(z[1], z.im);
        assert!(false);
    }
}

#[cfg(test)]
mod arithmetic_tests {
    use cglinalg::{
        Complex,
    };


    #[test]
    fn test_addition_complex_complex() {
        let z1 = Complex::new(1_i32, 3_i32);
        let z2 = Complex::new(5_i32, 17_i32);
        let expected = Complex::new(6_i32, 20_i32);
        let result = z1 + z2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_addition_complex_zero() {
        let zero: Complex<i32> = Complex::zero();
        let z = Complex::new(74_i32, 12_i32);

        assert_eq!(z + zero, z);
    }

    #[test]
    fn test_addition_zero_complex() {
        let zero: Complex<i32> = Complex::zero();
        let z = Complex::new(74_i32, 12_i32);

        assert_eq!(zero + z, z);
    }

    #[test]
    fn test_addition_zero_zero() {
        let zero: Complex<i32> = Complex::zero();

        assert_eq!(zero + zero, zero);
    }

    #[test]
    fn test_addition_scalar_complex() {
        let c = 8_i32;
        let z = Complex::new(9_i32, 4_i32);
        let expected = Complex::new(17_i32, 4_i32);
        let result = c + z;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_addition_complex_scalar() {
        let c = 8_i32;
        let z = Complex::new(9_i32, 4_i32);
        let expected = Complex::new(17_i32, 4_i32);
        let result = z + c;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction_complex_complex() {
        let z1 = Complex::new(45_i32, 7_i32);
        let z2 = Complex::new(74_i32, 10_i32);
        let expected = Complex::new(-29_i32, -3_i32);
        let result = z1 - z2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction_complex_zero() {
        let zero: Complex<i32> = Complex::zero();
        let z = Complex::new(74_i32, 12_i32);

        assert_eq!(z - zero, z);
    }

    #[test]
    fn test_subtraction_zero_complex() {
        let zero: Complex<i32> = Complex::zero();
        let z = Complex::new(74_i32, 12_i32);

        assert_eq!(zero - z, -z);
    }

    #[test]
    fn test_subtraction_zero_zero() {
        let zero: Complex<i32> = Complex::zero();

        assert_eq!(zero - zero, zero);
    }

    #[test]
    fn test_subtraction_scalar_complex() {
        let c = 7_i32;
        let z = Complex::new(1_i32, 3_i32);
        let expected = Complex::new(6_i32, 3_i32);
        let result = c - z;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction_complex_scalar() {
        let c = 7_i32;
        let z = Complex::new(1_i32, 3_i32);
        let expected = Complex::new(-6_i32, 3_i32);
        let result = z - c;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiplication_unit_re_complex() {
        let one: Complex<i32> = Complex::unit_re();
        let z = Complex::new(3_i32, 4_i32);

        assert_eq!(one * z, z);
    }

    #[test]
    fn test_multiplication_complex_unit_re() {
        let one: Complex<i32> = Complex::unit_re();
        let z = Complex::new(3_i32, 4_i32);

        assert_eq!(z * one, z);
    }

    #[test]
    fn test_unit_im_times_unit_im() {
        let i: Complex<i32> = Complex::unit_im();
        let one: Complex<i32> = Complex::unit_re();

        assert_eq!(i * i, -one);
    }

    #[test]
    fn test_multiplication_zero_zero() {
        let zero: Complex<i32> = Complex::zero();

        assert_eq!(zero * zero, zero);
    }

    #[test]
    fn test_multiplication_zero_complex() {
        let zero: Complex<i32> = Complex::zero();
        let z = Complex::new(2_i32, 3_i32);

        assert_eq!(zero * z, zero);
    }

    #[test]
    fn test_multiplication_complex_zero() {
        let zero: Complex<i32> = Complex::zero();
        let z = Complex::new(2_i32, 3_i32);

        assert_eq!(z * zero, zero);
    }

    #[test]
    fn test_multiplication_complex_complex() {
        let z1 = Complex::new(2_i32, 3_i32);
        let z2 = Complex::new(5_i32, 8_i32);
        let expected = Complex::new(-14_i32, 31_i32);
        let result = z1 * z2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiplication_scalar_complex() {
        let c = 3_i32;
        let z = Complex::new(2_i32, 5_i32);
        let expected = Complex::new(6_i32, 15_i32);
        let result = c * z;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiplication_complex_scalar() {
        let c = 3_i32;
        let z = Complex::new(2_i32, 5_i32);
        let expected = Complex::new(6_i32, 15_i32);
        let result = z * c;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_division_complex_complex() {
        let z1 = Complex::new(1_f64, 3_f64);
        let z2 = Complex::new(4_f64, 8_f64);
        let expected = Complex::new(7_f64 / 20_f64, 1_f64 / 20_f64);
        let result = z1 / z2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_division_complex_scalar() {
        let c = 5_f64;
        let z = Complex::new(3_f64, 7_f64);
        let expected = Complex::new(3_f64 / 5_f64, 7_f64 / 5_f64);
        let result = z / c;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_division_scalar_complex() {
        let c = 7_f64;
        let z = Complex::new(24_f64, 69_f64);
        let expected = Complex::new(56_f64 / 1779_f64, -161_f64 / 1779_f64);
        let result = c / z;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_division_unit_re_complex() {
        let one: Complex<f64> = Complex::unit_re();
        let z = Complex::new(57_f64, 92_f64);
        let expected = Complex::new(57_f64 / 11713_f64, -92_f64 / 11713_f64);
        let result = one / z;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_division_complex_unit_re() {
        let one: Complex<f64> = Complex::unit_re();
        let z = Complex::new(57_f64, 92_f64);
        let expected = z;
        let result = z / one;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_division_zero_complex() {
        let zero: Complex<i32> = Complex::zero();
        let z = Complex::new(1_i32, 2_i32);

        assert_eq!(zero / z, zero);
    }

    #[test]
    #[should_panic]
    fn test_division_complex_zero() {
        let zero: Complex<i32> = Complex::zero();
        let z = Complex::new(1_i32, 2_i32);

        assert_eq!(z / zero, z / zero);
    }
}

#[cfg(test)]
mod magnitude_tests {
    use cglinalg::{
        Complex,
    };


    #[test]
    fn test_unit_re_should_have_unit_norm() {
        let one: Complex<f64> = Complex::unit_re();

        assert_eq!(one.magnitude(), 1_f64);
    }

    #[test]
    fn test_unit_im_should_have_unit_norm() {
        let i: Complex<f64> = Complex::unit_im();

        assert_eq!(i.magnitude(), 1_f64);
    }
}

#[cfg(test)]
mod conjugate_tests {
    use cglinalg::{
        Complex,
    };


    #[test]
    fn test_conjugate() {
        let z = Complex::new(1_i32, 2_i32);
        let expected = Complex::new(1_i32, -2_i32);
        let result = z.conjugate();

        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod arg_tests {
    use cglinalg::{
        Complex,
    };


    #[test]
    fn test_arg_unit_im() {
        let i: Complex<f64> = Complex::unit_im();
        let expected = core::f64::consts::FRAC_PI_2;
        let result = i.arg();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_arg_unit_re() {
        let one: Complex<f64> = Complex::unit_re();
        let expected = 0_f64;
        let result = one.arg();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_arg_minus_unit_im() {
        let minus_i: Complex<f64> = -Complex::unit_im();
        let expected = -core::f64::consts::FRAC_PI_2;
        let result = minus_i.arg();
    }

    #[test]
    fn test_arg_minus_unit_re() {
        let minus_one: Complex<f64> = -Complex::unit_re();
        let expected = -core::f64::consts::PI;
        let result = minus_one.arg();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_arg_complex1() {
        let z = Complex::new(1_f64, 1_f64);
        let expected = core::f64::consts::FRAC_PI_4;
        let result = z.arg();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_arg_complex2() {
        let z = Complex::new(1_f64, -1_f64);
        let expected = -core::f64::consts::FRAC_PI_4;
        let result = z.arg();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_arg_complex3() {
        let z = Complex::new(-1_f64, 1_f64);
        let expected = 3_f64 * core::f64::consts::FRAC_PI_4;
        let result = z.arg();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_arg_complex4() {
        let z = Complex::new(-1_f64, -1_f64);
        let expected = -3_f64 * core::f64::consts::FRAC_PI_4;
        let result = z.arg();

        assert_eq!(result, expected);
    }
}
/*
#[cfg(test)]
mod inverse_tests {
    use cglinalg::{
        Complex,
    };


    #[test]
    fn test_inverse_zero() {
        let zero: Complex<f64> = Complex::zero();

        assert!(zero.inverse().is_none());
    }

    #[test]
    fn test_inverse_unit_re() {
        let one: Complex<f64> = Complex::unit_re();

        assert_eq!(one.inverse(), one);
    }

    #[test]
    fn test_inverse_unit_im() {
        let i: Complex<f64> = Complex::unit_im();

        assert_eq!(i.inverse().unwrap(), -i);
    }

    #[test]
    fn test_inverse_real() {
        let z = Complex::from_real(2_f64);
        let expected = Complex::from_real(1_f64 / 2_f64);
        let result = z.inverse().unwrap();

        assert_eq!(result, expected);
    }
}
*/