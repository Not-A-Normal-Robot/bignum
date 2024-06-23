mod misc {
    use std::fmt;

    use crate::bignum::BigNum;

    impl BigNum {
        pub fn is_nan(&self) -> bool {
            return self.exponent.is_nan();
        }

        pub fn is_infinite(&self) -> bool {
            return self.exponent.is_infinite() && self.exponent.is_sign_positive();
        }

        pub fn is_finite(&self) -> bool {
            return !self.is_infinite() && !self.is_nan();
        }

        pub fn is_sign_positive(&self) -> bool {
            return !self.is_negative;
        }

        pub fn is_sign_negative(&self) -> bool {
            return self.is_negative;
        }

        // log(0) = -Infinity
        pub fn is_zero(&self) -> bool {
            return self.exponent.is_infinite() && self.exponent.is_sign_negative();
        }

        pub fn classify(&self) -> std::num::FpCategory {
            if self.is_nan() {
                return std::num::FpCategory::Nan;
            }

            if self.is_infinite() {
                return std::num::FpCategory::Infinite;
            }

            if self.is_zero() {
                return std::num::FpCategory::Zero;
            }

            return self.exponent.classify();
        }
    }

    // Implementations for Display
    impl fmt::Display for BigNum {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}",
                if self.exponent < 100.0 {
                    self.to_string_sci(2)
                } else if self.exponent < 1e6 {
                    self.to_string_sci(0)
                } else {
                    self.to_string_log(3)
                }
            )
        }
    }
}