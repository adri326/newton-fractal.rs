use num::complex::Complex;
use std::fmt;
use super::complex_simd::Complex8;

#[derive(Clone, Debug)]
pub struct Polynomial {
    params: Vec<Complex<f64>>,
}

impl Polynomial {
    pub fn new(arr: &[Complex<f64>]) -> Self {
        Self {
            params: arr.iter().copied().collect()
        }
    }

    pub fn constant(x: impl Into<Complex<f64>>) -> Self {
        Self {
            params: vec![x.into()]
        }
    }

    pub fn from_roots(arr: &[Complex<f64>]) -> Self {
        let mut res: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); arr.len() + 1];
        res[0] = Complex::new(1.0, 0.0);

        let mut acc: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); arr.len() + 1];

        for root in arr {
            unsafe {
                // Equivalent to: memcpy(acc, res, sizeof(Complex<f64>) * arr.len())
                // Safe because res and acc have as capability `ref` and as length `arr.len()`
                // and because f64 and thus Complex<f64> have Copy

                // res -> acc
                std::ptr::copy_nonoverlapping(res.as_ptr(), acc.as_mut_ptr(), arr.len() + 1);
                // acc -> res + 1
                std::ptr::copy_nonoverlapping(acc.as_ptr(), res.as_mut_ptr().add(1), arr.len());
            }

            res[0] = Complex::new(0.0, 0.0);

            // res -= acc * root
            for (i, x) in res.iter_mut().enumerate() {
                *x -= acc[i] * root;
            }
        }

        Polynomial {
            params: res,
        }
    }

    pub fn len(&self) -> usize {
        self.params.len()
    }

    pub fn dim(&self) -> usize {
        let mut res: usize = 0;
        for (i, x) in self.params.iter().enumerate() {
            if x.re != 0.0 || x.im != 0.0 {
                res = i;
            }
        }
        res
    }

    pub fn shrink(&mut self) -> usize {
        let dim = self.dim();
        self.params.truncate(dim);
        self.params.shrink_to_fit();
        dim
    }

    pub fn eval(&self, z: Complex<f64>) -> Complex<f64> {
        let mut acc = Complex::new(1.0, 0.0);
        let mut res = Complex::new(0.0, 0.0);

        for &x in self.params.iter() {
            res += acc * x;
            acc *= z;
        }

        res
    }

    pub fn eval8(&self, z: Complex8) -> Complex8 {
        let mut acc = Complex8::from_complex(Complex::new(1.0, 0.0));
        let mut res = Complex8::from_complex(Complex::new(0.0, 0.0));

        for &x in self.params.iter() {
            res += acc * x;
            acc *= z;
        }

        res
    }

    pub fn diff(&self) -> Polynomial {
        if self.params.len() <= 1 {
            return Polynomial {
                params: vec![]
            };
        }

        let mut res = Polynomial {
            params: Vec::with_capacity(self.params.len() - 1),
        };

        let mut iter = self.params.iter().enumerate();
        iter.next(); // skip constant term
        for (i, x) in iter {
            res.params.push(i as f64 * x);
        }

        res
    }
}

impl std::ops::AddAssign<&Polynomial> for Polynomial {
    fn add_assign(self: &mut Polynomial, q: &Polynomial) {
        if q.params.len() >= self.params.len() {
            self.params.reserve(q.params.len() - self.params.len());
            for i in 0..self.params.len() {
                self.params[i] += q.params[i];
            }
            if q.params.len() > self.params.len() {
                for i in self.params.len()..q.params.len() {
                    self.params.push(q.params[i]);
                }
            }
        } else {
            for i in 0..q.params.len() {
                self.params[i] += q.params[i];
            }
        }
    }
}

impl std::ops::Add<&Polynomial> for Polynomial {
    type Output = Polynomial;

    fn add(mut self, q: &Polynomial) -> Self {
        self += q;

        self
    }
}

impl std::ops::Add<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn add(self, q: &Polynomial) -> Polynomial {
        let mut res = self.clone();
        res += q;
        res
    }
}

impl std::ops::Mul<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn mul(self, q: &Polynomial) -> Polynomial {
        let mut res = Polynomial {
            params: vec![Complex::new(0.0, 0.0); self.params.len() + q.params.len() - 1],
        };

        for (i, x) in self.params.iter().enumerate() {
            for (j, y) in q.params.iter().enumerate() {
                res.params[i + j] += x * y;
            }
        }

        res.shrink();
        res
    }
}

impl std::ops::MulAssign<&Polynomial> for Polynomial {
    fn mul_assign(&mut self, q: &Polynomial) {
        let res = &*self * q;

        self.params = res.params;
    }
}

impl std::cmp::PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        if self.dim() != other.dim() {
            return false;
        }

        for (i, x) in self.params.iter().copied().enumerate() {
            if other.params[i] != x {
                return false;
            }
        }

        true
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, z) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, " + ({})*z^{}", z, i)?;
            } else {
                write!(f, "({})", z)?;
            }
        }
        Ok(())
    }
}

#[test]
fn test_polynomial_from_roots() {
    assert_eq!(Polynomial::from_roots(&[
        Complex::new(0.0, 0.0)
    ]), Polynomial::new(&[
        Complex::new(0.0, 0.0),
        Complex::new(1.0, 0.0),
    ]));

    assert_eq!(Polynomial::from_roots(&[
        Complex::new(1.5, 0.6)
    ]), Polynomial::new(&[
        Complex::new(-1.5, -0.6),
        Complex::new(1.0, 0.0),
    ]));

    assert_eq!(Polynomial::from_roots(&[
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 1.0),
    ]), Polynomial::new(&[
        Complex::new(0.0, 1.0),
        Complex::new(-1.0, -1.0),
        Complex::new(1.0, 0.0),
    ]));
}
