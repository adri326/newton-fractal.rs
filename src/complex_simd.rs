use core_simd::f64x8;
use std::mem::ManuallyDrop;
use num::complex::Complex;

#[derive(Clone, Copy, Debug)]
pub struct Complex8 {
    pub re: f64x8,
    pub im: f64x8,
}

impl Complex8 {
    pub fn from_complex(c: Complex<f64>) -> Self {
        Self {
            re: f64x8::splat(c.re),
            im: f64x8::splat(c.im),
        }
    }

    pub fn norm(&self) -> f64x8 {
        let re: f64x8 = self.re * self.re;
        self.im * self.im + re
    }
}

// Complex8 + Complex8

impl std::ops::AddAssign<&Complex8> for Complex8 {
    #[inline]
    fn add_assign(&mut self, other: &Complex8) {
        self.re += other.re;
        self.im += other.im;
    }
}

impl std::ops::AddAssign<Complex8> for Complex8 {
    #[inline]
    fn add_assign(&mut self, other: Complex8) {
        self.re += other.re;
        self.im += other.im;
    }
}

impl std::ops::Add<Complex8> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn add(self, other: Complex8) -> Complex8 {
        Complex8 {
            re: self.re + other.re,
            im: self.im + other.im
        }
    }
}

impl std::ops::Add<&Complex8> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn add(self, other: &Complex8) -> Complex8 {
        Complex8 {
            re: self.re + other.re,
            im: self.im + other.im
        }
    }
}

// Complex8 + Complex

impl std::ops::AddAssign<&Complex<f64>> for Complex8 {
    #[inline]
    fn add_assign(&mut self, other: &Complex<f64>) {
        self.re += other.re;
        self.im += other.im;
    }
}

impl std::ops::AddAssign<Complex<f64>> for Complex8 {
    #[inline]
    fn add_assign(&mut self, other: Complex<f64>) {
        self.re += other.re;
        self.im += other.im;
    }
}

impl std::ops::Add<Complex<f64>> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn add(self, other: Complex<f64>) -> Complex8 {
        Complex8 {
            re: self.re + other.re,
            im: self.im + other.im
        }
    }
}

impl std::ops::Add<&Complex<f64>> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn add(self, other: &Complex<f64>) -> Complex8 {
        Complex8 {
            re: self.re + other.re,
            im: self.im + other.im
        }
    }
}

// Complex8 - Complex8

impl std::ops::SubAssign<&Complex8> for Complex8 {
    #[inline]
    fn sub_assign(&mut self, other: &Complex8) {
        self.re -= other.re;
        self.im -= other.im;
    }
}

impl std::ops::SubAssign<Complex8> for Complex8 {
    #[inline]
    fn sub_assign(&mut self, other: Complex8) {
        self.re -= other.re;
        self.im -= other.im;
    }
}

impl std::ops::Sub<Complex8> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn sub(self, other: Complex8) -> Complex8 {
        Complex8 {
            re: self.re - other.re,
            im: self.im - other.im
        }
    }
}

impl std::ops::Sub<&Complex8> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn sub(self, other: &Complex8) -> Complex8 {
        Complex8 {
            re: self.re - other.re,
            im: self.im - other.im
        }
    }
}

// Complex8 - Complex

impl std::ops::SubAssign<&Complex<f64>> for Complex8 {
    #[inline]
    fn sub_assign(&mut self, other: &Complex<f64>) {
        self.re -= other.re;
        self.im -= other.im;
    }
}

impl std::ops::SubAssign<Complex<f64>> for Complex8 {
    #[inline]
    fn sub_assign(&mut self, other: Complex<f64>) {
        self.re -= other.re;
        self.im -= other.im;
    }
}

impl std::ops::Sub<Complex<f64>> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn sub(self, other: Complex<f64>) -> Complex8 {
        Complex8 {
            re: self.re - other.re,
            im: self.im - other.im
        }
    }
}

impl std::ops::Sub<&Complex<f64>> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn sub(self, other: &Complex<f64>) -> Complex8 {
        Complex8 {
            re: self.re - other.re,
            im: self.im - other.im
        }
    }
}

// Complex8 * Complex8

impl std::ops::MulAssign<Complex8> for Complex8 {
    #[inline]
    fn mul_assign(&mut self, other: Complex8) {
        // im = -xi * yi
        let im: f64x8 = self.im * other.im;
        // comp = xr * yi
        let comp: f64x8 = self.re * other.im;
        // xi = xi * yr + comp = xi & yr + xr * yi
        self.im = self.im * other.re + comp;
        // xr = xr * yr - im = xr * yr - xi * yi
        self.re = self.re * other.re - im;
    }
}

impl std::ops::MulAssign<&Complex8> for Complex8 {
    #[inline]
    fn mul_assign(&mut self, other: &Complex8) {
        *self *= *other;
    }
}

impl std::ops::Mul<Complex8> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn mul(mut self, other: Complex8) -> Complex8 {
        self *= other;
        self
    }
}

impl std::ops::Mul<&Complex8> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn mul(mut self, other: &Complex8) -> Complex8 {
        self *= other;
        self
    }
}

// Complex8 * Complex

impl std::ops::MulAssign<Complex<f64>> for Complex8 {
    #[inline]
    fn mul_assign(&mut self, other: Complex<f64>) {
        let other: Complex8 = Complex8::from_complex(other);
        *self *= other;
    }
}

impl std::ops::MulAssign<&Complex<f64>> for Complex8 {
    #[inline]
    fn mul_assign(&mut self, other: &Complex<f64>) {
        let other: Complex8 = Complex8::from_complex(*other);
        *self *= other;
    }
}

impl std::ops::Mul<Complex<f64>> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn mul(mut self, z: Complex<f64>) -> Complex8 {
        self *= z;
        self
    }
}

impl std::ops::Mul<&Complex<f64>> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn mul(mut self, z: &Complex<f64>) -> Complex8 {
        self *= z;
        self
    }
}

// Complex8 * f64

impl std::ops::MulAssign<f64> for Complex8 {
    #[inline]
    fn mul_assign(&mut self, other: f64) {
        self.re *= other;
        self.im *= other;
    }
}

impl std::ops::Mul<f64> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn mul(mut self, z: f64) -> Complex8 {
        self *= z;
        self
    }
}

// Complex8 / Complex8

impl std::ops::DivAssign<Complex8> for Complex8 {
    #[inline]
    fn div_assign(&mut self, mut other: Complex8) {
        // im = xi * yi
        let im: f64x8 = self.im * other.im;
        // comp = -xr * yi
        let comp: f64x8 = -(self.re * other.im);
        // xi = xi * yr + comp = xi * yr - xr * yi
        self.im = self.im * other.re + comp;
        // xr = xr * yr + im = xr * yr + xi * yi
        self.re = self.re * other.re + im;
        // yr * yr
        let re2: f64x8 = other.re * other.re;
        // norm = yi * yi + re2 = yi * yi + yr * yr
        let norm: f64x8 = other.im * other.im + re2;
        // xr /= norm = (xr * yr + xi * yi) / (yr * yr + yi * yi)
        self.re /= norm;
        // xi /= norm = (xi * yr - xr * yi) / (yr * yr + yi * yi)
        self.im /= norm;
    }
}

impl std::ops::DivAssign<&Complex8> for Complex8 {
    #[inline]
    fn div_assign(&mut self, other: &Complex8) {
        *self *= *other;
    }
}

impl std::ops::Div<Complex8> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn div(mut self, other: Complex8) -> Complex8 {
        self *= other;
        self
    }
}

impl std::ops::Div<&Complex8> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn div(mut self, other: &Complex8) -> Complex8 {
        self *= other;
        self
    }
}

// Complex8 / Complex

impl std::ops::DivAssign<Complex<f64>> for Complex8 {
    #[inline]
    fn div_assign(&mut self, other: Complex<f64>) {
        let other: Complex8 = Complex8::from_complex(other);
        *self /= other;
    }
}

impl std::ops::DivAssign<&Complex<f64>> for Complex8 {
    #[inline]
    fn div_assign(&mut self, other: &Complex<f64>) {
        let other: Complex8 = Complex8::from_complex(*other);
        *self /= other;
    }
}

impl std::ops::Div<Complex<f64>> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn div(mut self, z: Complex<f64>) -> Complex8 {
        self /= z;
        self
    }
}

impl std::ops::Div<&Complex<f64>> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn div(mut self, z: &Complex<f64>) -> Complex8 {
        self /= z;
        self
    }
}

// Complex8 / f64

impl std::ops::DivAssign<f64> for Complex8 {
    #[inline]
    fn div_assign(&mut self, other: f64) {
        self.re /= other;
        self.im /= other;
    }
}

impl std::ops::Div<f64> for Complex8 {
    type Output = Complex8;

    #[inline]
    fn div(mut self, z: f64) -> Complex8 {
        self /= z;
        self
    }
}

impl std::convert::From<[Complex<f64>; 8]> for Complex8 {
    fn from(arr: [Complex<f64>; 8]) -> Complex8 {
        let arr = ManuallyDrop::new(arr); // Prevent arr from being dropped...
        let f64_ptr = arr.as_ptr() as *const f64;
        let mut half1: [f64; 8] = [0.0; 8];
        let mut half2: [f64; 8] = [0.0; 8];

        // Copy into half1 and half2
        unsafe {
            std::ptr::copy_nonoverlapping(f64_ptr, half1.as_mut_ptr(), 8);
            std::ptr::copy_nonoverlapping(f64_ptr.add(8), half2.as_mut_ptr(), 8);
        }

        let half1v = f64x8::from_array(half1);
        let half2v = f64x8::from_array(half2);

        // Magic!
        let (re, im) = half1v.deinterleave(half2v);

        let _ = ManuallyDrop::into_inner(arr); // Drop arr

        Complex8 {
            re,
            im,
        }
    }
}

impl std::convert::From<Complex8> for [Complex<f64>; 8] {
    fn from(complex: Complex8) -> [Complex<f64>; 8] {
        let (low, high) = complex.re.interleave(complex.im);
        let low = low.to_array();
        let high = high.to_array();

        let mut res = [Complex::new(0.0, 0.0); 8];
        let f64_ptr = res.as_mut_ptr() as *mut f64;

        unsafe {
            std::ptr::copy_nonoverlapping(low.as_ptr(), f64_ptr, 8);
            std::ptr::copy_nonoverlapping(high.as_ptr(), f64_ptr.add(8), 8);
        }

        std::mem::forget(complex);

        res
    }
}

#[test]
fn test_from_complex8() {
    let arr = [
        Complex::<f64>::new(1.0, 2.0),
        Complex::<f64>::new(3.0, 4.0),
        Complex::<f64>::new(5.0, 6.0),
        Complex::<f64>::new(7.0, 8.0),
        Complex::<f64>::new(9.0, 10.0),
        Complex::<f64>::new(11.0, 12.0),
        Complex::<f64>::new(13.0, 14.0),
        Complex::<f64>::new(15.0, 16.0),
    ];
    let combined = Complex8::from(arr.clone());
    assert_eq!(combined.re, f64x8::from_array([1.0, 3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0]));
    assert_eq!(combined.im, f64x8::from_array([2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0]));
    let arr2: [Complex<f64>; 8] = combined.into();
    assert_eq!(arr2, arr);
}
