mod ops {
    use std::{f64::NAN, ops};
    use crate::bignum::BigNum;

    impl ops::Add for BigNum {
        type Output = BigNum;

        fn add(self, other: Self) -> Self::Output {
            if self.is_nan() || other.is_nan() {
                return BigNum::NAN;
            }

            if self.is_infinite() || other.is_infinite() {
                // Infinity - Infinity = NaN
                if self.is_infinite() && other.is_infinite() && self.is_negative != other.is_negative {
                    return BigNum::NAN;
                }

                // Infinity + x = Infinity
                return if self.is_infinite() {
                    self
                } else {
                    other
                };
            }

            // Different signs = subtraction
            if self.is_negative != other.is_negative {
                if self.is_negative {
                    // self is negative, other is positive
                    // we normalize so both are positive
                    return other - (- self);
                } else {
                    // self is positive, other is negative
                    // we normalize so both are positive
                    return self - (- other);
                }
            }

            // Below this, signs are equal
            
            let (smaller, larger) = self.order(&other);

            if larger.exponent - smaller.exponent > 20.0 || smaller.is_zero() {
                // If the difference in exponents is too large, the smaller number is negligible
                return larger;
            }

            // we can assume a >= b without loss of generality
            // log10(10^a + 10^b) = a + log10(1 + 10^(b - a)), for all real a, b
            return BigNum {
                exponent: larger.exponent + (1.0 + 10_f64.powf(smaller.exponent - larger.exponent)).log10(),
                is_negative: larger.is_negative
            }
        }
    }
    impl ops::Add<f64> for BigNum {
        type Output = BigNum;

        fn add(self, other: f64) -> Self::Output {
            return self + BigNum::from_f64(other);
        }
    }
    impl ops::Add<BigNum> for f64 {
        type Output = BigNum;

        fn add(self, other: BigNum) -> Self::Output {
            return BigNum::from_f64(self) + other;
        }
    }

    impl ops::AddAssign for BigNum {
        fn add_assign(&mut self, other: Self) {
            *self = *self + other;
        }
    }
    impl ops::AddAssign<f64> for BigNum {
        fn add_assign(&mut self, other: f64) {
            *self = *self + BigNum::from_f64(other);
        }
    }
    impl ops::AddAssign<BigNum> for f64 {
        fn add_assign(&mut self, other: BigNum) {
            *self = *self + other.to_f64().unwrap_or(f64::MAX);
        }
    }

    impl ops::Neg for BigNum {
        type Output = BigNum;

        fn neg(self) -> Self::Output {
            return BigNum {
                exponent: self.exponent,
                is_negative: !self.is_negative
            }
        }
    }

    impl ops::Sub for BigNum {
        type Output = BigNum;

        fn sub(self, rhs: Self) -> Self::Output {
            if self.is_nan() || rhs.is_nan() {
                return BigNum::NAN;
            }

            if self.is_infinite() || rhs.is_infinite() {
                // Infinity - Infinity = NaN
                if self.is_infinite() && rhs.is_infinite() && self.is_negative == rhs.is_negative {
                    return BigNum::NAN;
                }
                // Below this, signs are different

                return BigNum {
                    // -Infinity - Infinity = -Infinity
                    // Infinity - -Infinity = Infinity
                    is_negative: self.is_negative,
                    exponent: f64::INFINITY
                }
            }

            // Different signs = addition
            if self.is_negative != rhs.is_negative {
                if self.is_negative {
                    // self is negative, rhs is positive
                    // we normalize so both are positive
                    return -(-self + rhs);
                } else {
                    // self is positive, rhs is negative
                    // we normalize so both are positive
                    return self + -rhs;
                }
            }
            // Below this, signs are equal

            if self.is_zero() {
                // 0 - x = -x
                return -rhs;
            } else if rhs.is_zero() {
                // x - 0 = x
                return self;
            }

            // if the difference between exponents is too large, the smaller number is negligible
            if self.exponent - rhs.exponent > 20.0 {
                // +++ - ~0 = +++
                return self;
            } else if rhs.exponent - self.exponent > 20.0 {
                // ~0 - +++ = ---
                return -rhs;
            }

            // log10(10^a - 10^b) = b + log10​(10^(a - b) − 1), for a > b
            return BigNum {
                exponent: rhs.exponent + (10_f64.powf(self.exponent - rhs.exponent) - 1.0).abs().log10(),
                is_negative: self.is_negative != (self.exponent < rhs.exponent)
            }
        }
    }
    impl ops::Sub<f64> for BigNum {
        type Output = BigNum;

        fn sub(self, other: f64) -> Self::Output {
            return self - BigNum::from_f64(other);
        }
    }
    impl ops::Sub<BigNum> for f64 {
        type Output = BigNum;

        fn sub(self, other: BigNum) -> Self::Output {
            return BigNum::from_f64(self) - other;
        }
    }

    impl ops::SubAssign for BigNum {
        fn sub_assign(&mut self, other: Self) {
            *self = *self - other;
        }
    }
    impl ops::SubAssign<f64> for BigNum {
        fn sub_assign(&mut self, other: f64) {
            *self = *self - BigNum::from_f64(other);
        }
    }
    impl ops::SubAssign<BigNum> for f64 {
        fn sub_assign(&mut self, other: BigNum) {
            *self = *self - other.to_f64().unwrap_or(f64::MAX);
        }
    }

    impl ops::Mul for BigNum {
        type Output = BigNum;

        fn mul(self, rhs: Self) -> Self::Output {
            if self.is_nan() || rhs.is_nan() {
                return BigNum::NAN;
            }

            if self.is_infinite() || rhs.is_infinite() {
                // Infinity * any = Infinity
                return BigNum {
                    is_negative: self.is_negative != rhs.is_negative,
                    exponent: f64::INFINITY
                }
            }

            // log10(10^a * 10^b) = a + b, for all real a, b
            return BigNum {
                exponent: self.exponent + rhs.exponent,
                is_negative: self.is_negative != rhs.is_negative
            }
        }
    }
    impl ops::Mul<f64> for BigNum {
        type Output = BigNum;

        fn mul(self, other: f64) -> Self::Output {
            return self * BigNum::from_f64(other);
        }
    }
    impl ops::Mul<BigNum> for f64 {
        type Output = BigNum;

        fn mul(self, other: BigNum) -> Self::Output {
            return BigNum::from_f64(self) * other;
        }
    }

    impl ops::MulAssign for BigNum {
        fn mul_assign(&mut self, other: Self) {
            *self = *self * other;
        }
    }
    impl ops::MulAssign<f64> for BigNum {
        fn mul_assign(&mut self, other: f64) {
            *self = *self * BigNum::from_f64(other);
        }
    }
    impl ops::MulAssign<BigNum> for f64 {
        fn mul_assign(&mut self, other: BigNum) {
            *self = *self * other.to_f64().unwrap_or(f64::MAX);
        }
    }

    impl ops::Div for BigNum {
        type Output = BigNum;

        fn div(self, rhs: Self) -> Self::Output {
            if self.is_nan() || rhs.is_nan() {
                return BigNum::NAN;
            }

            if self.is_infinite() || rhs.is_infinite() {
                // Infinity / Infinity = NaN
                if self.is_infinite() && rhs.is_infinite() {
                    return BigNum::NAN;
                }

                // Infinity / finite = Infinity
                if self.is_infinite() {
                    return BigNum {
                        is_negative: self.is_negative != rhs.is_negative,
                        exponent: f64::INFINITY
                    }
                }

                // finite / Infinity = 0
                return BigNum::ZERO;
            }

            // log10(10^a / 10^b) = a - b, for all real a, b
            return BigNum {
                exponent: self.exponent - rhs.exponent,
                is_negative: self.is_negative != rhs.is_negative
            }
        }
    }
    impl ops::Div<f64> for BigNum {
        type Output = BigNum;

        fn div(self, other: f64) -> Self::Output {
            return self / BigNum::from_f64(other);
        }
    }
    impl ops::Div<BigNum> for f64 {
        type Output = BigNum;

        fn div(self, other: BigNum) -> Self::Output {
            return BigNum::from_f64(self) / other;
        }
    }

    impl ops::DivAssign for BigNum {
        fn div_assign(&mut self, other: Self) {
            *self = *self / other;
        }
    }
    impl ops::DivAssign<f64> for BigNum {
        fn div_assign(&mut self, other: f64) {
            *self = *self / BigNum::from_f64(other);
        }
    }
    impl ops::DivAssign<BigNum> for f64 {
        fn div_assign(&mut self, other: BigNum) {
            *self = *self / other.to_f64().unwrap_or(f64::MAX);
        }
    }

    impl BigNum {
        pub fn recip(&self) -> Self {
            return BigNum {
                exponent: -self.exponent,
                is_negative: self.is_negative
            }
        }
    }

    impl ops::Rem for BigNum {
        type Output = BigNum;

        // Algorithm copied from logarithmica_numerus_lite.js by Aarex: https://github.com/aarextiaokhiao/magna_numerus.js/blob/master/logarithmica_numerus_lite.js#L308
        fn rem(self, rhs: Self) -> Self::Output {
            if self.is_infinite() && rhs.is_infinite() {
                return BigNum {
                    is_negative: self.is_negative != rhs.is_negative,
                    exponent: f64::NEG_INFINITY
                }
            }

            let exp_diff = self.exponent - rhs.exponent;
            if exp_diff < 0.0 {
                // a % b = a, if a < b
                return self;
            } else if exp_diff >= 15.0 || exp_diff == 0.0 {
                // a % b = 0, if a >= b
                // a % b will return 0 if a >> b
                return BigNum {
                    is_negative: self.is_negative,
                    exponent: f64::NEG_INFINITY
                };
            }

            let modulo = 10_f64.powf(exp_diff);
            let modulo_floor = modulo.floor();

            if modulo == modulo_floor {
                return BigNum {
                    is_negative: self.is_negative != rhs.is_negative,
                    exponent: f64::NEG_INFINITY
                };
            } else {
                return BigNum {
                    is_negative: self.is_negative != rhs.is_negative,
                    exponent: rhs.exponent + (modulo - modulo_floor).log10()
                };
            }
        }
    }
    impl ops::Rem<f64> for BigNum {
        type Output = BigNum;

        fn rem(self, other: f64) -> Self::Output {
            return self % BigNum::from_f64(other);
        }
    }
    impl ops::Rem<BigNum> for f64 {
        type Output = BigNum;

        fn rem(self, other: BigNum) -> Self::Output {
            return BigNum::from_f64(self) % other;
        }
    }

    impl ops::RemAssign for BigNum {
        fn rem_assign(&mut self, other: Self) {
            *self = *self % other;
        }
    }
    impl ops::RemAssign<f64> for BigNum {
        fn rem_assign(&mut self, other: f64) {
            *self = *self % BigNum::from_f64(other);
        }
    }
    impl ops::RemAssign<BigNum> for f64 {
        fn rem_assign(&mut self, other: BigNum) {
            *self = *self % other.to_f64().unwrap_or(f64::MAX);
        }
    }

    // Exponentiation
    impl BigNum {
        pub fn powf(&self, exp: f64) -> Self {
            if self.is_nan() || exp.is_nan() {
                return BigNum::NAN;
            }

            if self.is_zero() {
                if exp.is_sign_positive() {
                    return BigNum::ZERO;
                } else if exp.is_sign_negative() {
                    return BigNum::NAN;
                } else {
                    return BigNum::ONE;
                }
            }

            if self.is_infinite() {
                if exp.is_sign_positive() {
                    return BigNum {
                        is_negative: self.is_negative && exp % 2.0 != 0.0,
                        exponent: f64::INFINITY
                    }
                } else if exp.is_sign_negative() {
                    return BigNum::ZERO;
                } else {
                    return BigNum::NAN;
                }
            }

            if exp == 0.0 {
                return BigNum::ONE;
            }

            if exp == 1.0 {
                return *self;
            }

            return BigNum {
                is_negative: self.is_negative && exp % 2.0 != 0.0,
                exponent: self.exponent * exp
            };
        }

        pub fn powi(&self, exp: i32) -> Self {
            if self.is_nan() {
                return BigNum::NAN;
            }

            if self.is_zero() {
                if exp > 0 {
                    return BigNum::ZERO;
                } else if exp < 0 {
                    return BigNum::NAN;
                } else {
                    return BigNum::ONE;
                }
            }

            if self.is_infinite() {
                if exp > 0 {
                    return BigNum {
                        is_negative: self.is_negative && exp % 2 != 0,
                        exponent: f64::INFINITY
                    }
                } else if exp < 0 {
                    return BigNum::ZERO;
                } else {
                    return BigNum::NAN;
                }
            }

            if exp == 0 {
                return BigNum::ONE;
            }

            if exp == 1 {
                return *self;
            }

            return BigNum {
                is_negative: self.is_negative && exp % 2 != 0,
                exponent: self.exponent * exp as f64
            };
        }

        pub fn powb(&self, exp: BigNum) -> Self {
            if self.is_nan() || exp.is_nan() {
                return BigNum::NAN;
            }

            if self.is_zero() {
                if exp.is_sign_positive() {
                    return BigNum::ZERO;
                } else if exp.is_sign_negative() {
                    return BigNum::NAN;
                } else {
                    return BigNum::ONE;
                }
            }

            if self.is_infinite() {
                if exp.is_sign_positive() {
                    return BigNum {
                        is_negative: self.is_negative && exp % 2.0 != BigNum::ZERO,
                        exponent: f64::INFINITY
                    }
                } else if exp.is_sign_negative() {
                    return BigNum::ZERO;
                } else {
                    return BigNum::NAN;
                }
            }

            return BigNum {
                is_negative: self.is_negative && exp % 2.0 != BigNum::ZERO,
                exponent: (self.exponent * exp).to_f64().unwrap_or(f64::INFINITY)
            };
        }
    }

    // Common exponential functions
    impl BigNum {
        pub fn sqr(&self) -> Self {
            return BigNum {
                is_negative: false,
                exponent: self.exponent * 2.0
            }
        }

        pub fn cube(&self) -> Self {
            return BigNum {
                is_negative: self.is_negative,
                exponent: self.exponent * 3.0
            }
        }

        pub fn sqrt(&self) -> Self {
            if self.is_nan() || self.is_negative {
                return BigNum::NAN;
            }

            return BigNum {
                is_negative: false,
                exponent: self.exponent / 2.0
            }
        }

        pub fn cbrt(&self) -> Self {
            if self.is_nan() {
                return BigNum::NAN;
            }

            return BigNum {
                is_negative: self.is_negative,
                exponent: self.exponent / 3.0
            }
        }

        pub fn exp(&self) -> Self {
            let e = BigNum::from_f64(std::f64::consts::E);
            return e.powb(*self);
        }

        pub fn exp2(&self) -> Self {
            let two = BigNum {
                is_negative: false,
                exponent: std::f64::consts::LOG10_2
            };
            return two.powb(*self);
        }

        pub fn log10(&self) -> Self {
            if self.is_negative {
                return BigNum::NAN;
            }

            return self.abs_log10();
        }

        pub fn abs_log10(&self) -> Self {
            return BigNum {
                is_negative: false,
                exponent: self.exponent.log10()
            };
        }

        pub fn log(&self, base: f64) -> Self {
            if self.is_negative {
                return BigNum::NAN;
            }

            return self.abs_log(base);
        }

        pub fn abs_log(&self, base: f64) -> Self {
            return BigNum {
                is_negative: false,
                exponent: self.exponent.log(base)
            };
        }

        pub fn log2(&self) -> Self {
            if self.is_negative {
                return BigNum::NAN;
            }

            return self.abs_log2();
        }

        pub fn abs_log2(&self) -> Self {
            return BigNum {
                is_negative: false,
                exponent: self.exponent.log2()
            };
        }

        pub fn ln(&self) -> Self {
            if self.is_negative {
                return BigNum::NAN;
            }

            return self.abs_ln();
        }

        pub fn abs_ln(&self) -> Self {
            return BigNum {
                is_negative: false,
                exponent: self.exponent.ln()
            };
        }

        pub fn hypot(&self, other: &Self) -> Self {
            return (self.sqr() + other.sqr()).sqrt();
        }
    }

    // Trig functions
    impl BigNum {
        pub fn sin(&self) -> f64 {
            let phase = *self % BigNum::TAU;
            let phase = phase.to_f64().unwrap_or(NAN);
            return phase.sin();
        }

        pub fn cos(&self) -> f64 {
            let phase = *self % BigNum::TAU;
            let phase = phase.to_f64().unwrap_or(NAN);
            return phase.cos();
        }

        pub fn sin_cos(&self) -> (f64, f64) {
            let phase = *self % BigNum::TAU;
            let phase = phase.to_f64().unwrap_or(NAN);
            return phase.sin_cos();
        }

        pub fn tan(&self) -> f64 {
            let phase = *self % BigNum::TAU;
            let phase = phase.to_f64().unwrap_or(NAN);
            return phase.tan();
        }

        pub fn asin(&self) -> f64 {
            let phase = self.to_f64().unwrap_or(NAN);
            return phase.asin();
        }

        pub fn acos(&self) -> f64 {
            let phase = self.to_f64().unwrap_or(NAN);
            return phase.acos();
        }

        pub fn atan(&self) -> f64 {
            let phase = self.to_f64().unwrap_or(NAN);
            return phase.atan();
        }

        pub fn atan2(&self, other: &Self) -> f64 {
            let phase = self.to_f64().unwrap_or(NAN);
            let other = other.to_f64().unwrap_or(NAN);
            return phase.atan2(other);
        }

        pub fn sinh(&self) -> f64 {
            let phase = self.to_f64().unwrap_or(NAN);
            return phase.sinh();
        }

        pub fn cosh(&self) -> f64 {
            let phase = self.to_f64().unwrap_or(NAN);
            return phase.cosh();
        }

        pub fn tanh(&self) -> f64 {
            let phase = self.to_f64().unwrap_or(NAN);
            return phase.tanh();
        }

        pub fn asinh(&self) -> f64 {
            let phase = self.to_f64().unwrap_or(NAN);
            return phase.asinh();
        }

        pub fn acosh(&self) -> f64 {
            let phase = self.to_f64().unwrap_or(NAN);
            return phase.acosh();
        }

        pub fn atanh(&self) -> f64 {
            let phase = self.to_f64().unwrap_or(NAN);
            return phase.atanh();
        }

        pub fn sinc(&self) -> f64 {
            let phase = self.to_f64().unwrap_or(NAN);
            return phase.sin() / phase;
        }

        pub fn cosc(&self) -> f64 {
            let phase = self.to_f64().unwrap_or(NAN);
            return phase.cos() / phase;
        }

        pub fn tanc(&self) -> f64 {
            let phase = self.to_f64().unwrap_or(NAN);
            return phase.tan() / phase;
        }
    }

    // Integer operations
    impl BigNum {
        pub fn floor(&self) -> Self {
            if self.exponent > 300.0 {
                return *self;
            }

            return self.to_f64().unwrap().floor().into();
        }

        pub fn ceil(&self) -> Self {
            if self.exponent > 300.0 {
                return *self;
            }

            return self.to_f64().unwrap().ceil().into();
        }

        pub fn round(&self) -> Self {
            if self.exponent > 300.0 {
                return *self;
            }

            return self.to_f64().unwrap().round().into();
        }

        pub fn trunc(&self) -> Self {
            if self.exponent > 300.0 {
                return *self;
            }

            return self.to_f64().unwrap().trunc().into();
        }

        pub fn fract(&self) -> Self {
            if self.exponent > 300.0 {
                return BigNum::ZERO;
            }

            return self.to_f64().unwrap().fract().into();
        }
    }

    // Equality
    impl BigNum {
        pub fn almost_equal(&self, other: &BigNum, tolerance: &BigNum) -> bool {
            return (*self - *other).abs() < *tolerance;
        }
    }

    // Sign operations
    impl BigNum {
        pub fn abs(&self) -> BigNum {
            return BigNum {
                is_negative: false,
                exponent: self.exponent
            };
        }

        pub fn signum(&self) -> BigNum {
            if self.is_nan() {
                return BigNum::NAN;
            }

            return BigNum {
                is_negative: self.is_negative,
                exponent: 0.0
            };
        }
    }

    // Order operations
    impl BigNum {
        pub fn clamp(&self, min: &BigNum, max: &BigNum) -> Option<BigNum> {
            if self.is_nan() {
                return None;
            }

            if self < min {
                return Some(*min);
            } else if self > max {
                return Some(*max);
            }

            return Some(*self);
        }

        /// Returns the smaller and the larger of the two numbers, respectively.
        pub fn order(&self, other: &BigNum) -> (BigNum, BigNum) {
            if self < other {
                return (*self, *other);
            }

            return (*other, *self);
        }

        pub fn min(&self, other: &BigNum) -> BigNum {
            return self.order(other).0;
        }

        pub fn max(&self, other: &BigNum) -> BigNum {
            return self.order(other).1;
        }
    }
}