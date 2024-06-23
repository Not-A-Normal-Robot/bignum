mod constants {
    use crate::bignum::BigNum;

    impl BigNum {
        pub const NEG_INFINITY: BigNum = BigNum {
            is_negative: true,
            exponent: f64::INFINITY
        };
        
        pub const MIN: BigNum = BigNum {
            is_negative: true,
            exponent: f64::MAX
        };

        pub const NEG_ONE: BigNum = BigNum {
            is_negative: true,
            exponent: 0.0
        };

        pub const MAX_NEGATIVE: BigNum = BigNum {
            is_negative: true,
            exponent: f64::MIN_POSITIVE
        };

        pub const ZERO: BigNum = BigNum {
            is_negative: false,
            exponent: f64::NEG_INFINITY
        };

        pub const MIN_POSITIVE: BigNum = BigNum {
            is_negative: false,
            exponent: f64::MIN_POSITIVE
        };

        pub const ONE: BigNum = BigNum {
            is_negative: false,
            exponent: 0.0
        };

        pub const MAX: BigNum = BigNum {
            is_negative: false,
            exponent: f64::MAX
        };

        pub const INFINITY: BigNum = BigNum {
            is_negative: false,
            exponent: f64::INFINITY
        };

        pub const NAN: BigNum = BigNum {
            is_negative: false,
            exponent: f64::NAN
        };

        pub const PI: BigNum = BigNum {
            is_negative: false,
            exponent: 0.49714987269413385 // log10(pi)
        };

        pub const TAU: BigNum = BigNum {
            is_negative: false,
            exponent: 0.798179868358115 // log10(tau)
        };

        pub const E: BigNum = BigNum {
            is_negative: false,
            exponent: 0.4342944819032518 // log10(e)
        };
    }
}