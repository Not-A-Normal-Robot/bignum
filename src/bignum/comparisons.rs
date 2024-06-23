mod cmp {
    use std::cmp::Ordering;
    use crate::bignum::BigNum;

    impl std::cmp::PartialOrd for BigNum {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            // NaNs are uncomparable
            if self.is_nan() || other.is_nan() {
                return None;
            }
    
            // Infinities of the same sign are uncomparable
            if self.is_infinite() && other.is_infinite() && self.is_negative == other.is_negative {
                return None;
            }
    
            // Handle different signs
            if self.is_negative && !other.is_negative {
                // - < +
                return Some(Ordering::Less);
            } else if !self.is_negative && other.is_negative {
                // + > -
                return Some(Ordering::Greater);
            }
            // Below this, signs are equal
    
            if self.exponent > other.exponent {
                if self.is_negative {
                    // -- < -
                    return Some(Ordering::Less);
                }
                // ++ > +
                return Some(Ordering::Greater);
            } else if self.exponent < other.exponent {
                if self.is_negative {
                    // - > --
                    return Some(Ordering::Greater);
                }
                // + < ++
                return Some(Ordering::Less);
            }
            // + = +, - = -
            return Some(Ordering::Equal);
        }
    }
}