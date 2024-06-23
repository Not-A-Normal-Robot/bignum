pub mod bignum {
    pub mod converters;
    pub mod comparisons;
    pub mod operations;
    pub mod constants;
    pub mod misc;

    mod tests;

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub struct BigNum {
        pub is_negative: bool,
        pub exponent: f64
    }
}