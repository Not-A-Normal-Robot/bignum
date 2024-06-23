mod converters {
    use std::num::ParseFloatError;
    use crate::bignum::BigNum;

    impl BigNum {
        pub fn from_exponent(exponent: f64, is_negative: bool) -> BigNum {
            return BigNum {
                is_negative,
                exponent
            };
        }

        pub fn from_f64(value: f64) -> BigNum {
            return BigNum {
                is_negative: value.is_sign_negative(),
                exponent: value.abs().log10()
            };
        }

        pub fn from_f32(value: f32) -> BigNum {
            return Self::from_f64(value as f64);
        }

        pub fn from_string(value: &str) -> Result<BigNum, String> {
            if value.is_empty() {
                return Err("Value is empty".into());
            }

            match value.to_lowercase().as_str() {
                "inf"       => return Ok(BigNum::INFINITY),
                "infinity"  => return Ok(BigNum::INFINITY),
                "-inf"      => return Ok(BigNum::NEG_INFINITY),
                "-infinity" => return Ok(BigNum::NEG_INFINITY),
                "nan"       => return Ok(BigNum::NAN),
                _ => {}
            }


            let first_char: char = value.as_bytes()[0].into();

            let is_negative = match first_char {
                '-' => true,
                '+' => false,
                char => {
                    if char.is_numeric() || char.to_ascii_lowercase() == 'e' {
                        false
                    } else {
                        return Err("Invalid first character".into());
                    }
                }
            };

            let has_sign_char = match first_char {
                '-' => true,
                '+' => true,
                _ => false
            };

            let is_char_e: fn(char) -> bool = |c| c.to_ascii_lowercase() == 'e';
            
            let exponent = {
                // check if string has an 'e' or an 'E', if it does, get the position
                if let Some(e_position) = value.find(is_char_e) {

                    // lhs: coefficient, before 'e', like the '1.23' in '1.23e456'
                    // rhs: exponent, after 'e', like the '456' in '1.23e456'
                    let start = if has_sign_char { 1 } else { 0 };
                    let lhs = &value[start..e_position];
                    let rhs = &value[(e_position + 1)..];

                    if lhs.is_empty() {
                        // logarithm notation (like 'e1234', '-e11', 'e-11')
                        let parse_result: Result<f64, ParseFloatError> = rhs.parse();

                        if let Err(e) = parse_result {
                            return Err(e.to_string());
                        }

                        parse_result.unwrap()
                    } else {
                        // scientific notation (like '1.23e456', '-1e12', '1e-12')
                        let coefficient: Result<f64, ParseFloatError> = lhs.parse();

                        if let Err(e) = coefficient {
                            return Err(e.to_string());
                        }

                        let coefficient = coefficient.unwrap();


                        let exponent: Result<f64, ParseFloatError> = rhs.parse();

                        if let Err(e) = exponent {
                            return Err(e.to_string());
                        }

                        let exponent = exponent.unwrap();

                        // log_10(coefficient * 10^exponent) simplifies to this
                        coefficient.log10() + exponent
                    }
                } else {
                    // Regular number (like '123.456', '-123.456')
                    let parse_result: Result<f64, ParseFloatError> = value.parse();

                    if let Err(e) = parse_result {
                        return Err(e.to_string());
                    }

                    parse_result.unwrap().abs().log10()
                }
            };

            return Ok(BigNum {
                is_negative,
                exponent
            });
        }

        pub fn to_f64(&self) -> Option<f64> {
            if self.exponent > f64::MAX.log10() {
                return None;
            }

            let abs_value = 10_f64.powf(self.exponent);

            return Some(if self.is_negative { -abs_value } else { abs_value });
        }

        pub fn to_string_num(&self, precision: usize) -> Option<String> {
            if self.exponent > f64::MAX.log10() {
                return None;
            }

            return Some(
                format!(
                    "{}{:.*}",
                    if self.is_negative { "-" } else { "" },
                    precision,
                    10_f64.powf(self.exponent)
                )
            );
        }

        pub fn to_string_sci(&self, precision: usize) -> String {
            if self.exponent.is_nan() {
                return "NaN".into();
            } else if self.is_zero() {
                return format!("{:.*}", precision, 0);
            } else if self.is_infinite() {
                return format!("{}inf", if self.is_negative { "-" } else { "" });
            } else if self.exponent < 9.0 && self.exponent > -3.0 {
                return self.to_string_num(precision).unwrap();
            }

            let coefficient = 10_f64.powf(self.exponent.fract());
            let exponent = self.exponent.floor();

            if self.exponent.abs() < 1e9 {
                // single exponential, e.g. '1.23e456'
                return format!(
                    "{}{:.*}e{:.0}",
                    if self.is_negative { "-" } else { "" },
                    precision,
                    coefficient,
                    exponent
                );
            } else {
                // double exponential, e.g. '3e1.23e45'
                return format!(
                    "{}{:.0}e{}",
                    if self.is_negative { "-" } else { "" },
                    coefficient,
                    {
                        let exponent_exponent = exponent.abs().log10().floor();
                        let exponent_coefficient = exponent / 10_f64.powf(exponent_exponent);

                        format!("{:.*}e{:.0}", precision, exponent_coefficient, exponent_exponent)
                    }
                );
            }
        }

        pub fn to_string_log(&self, precision: usize) -> String {
            if self.exponent.is_nan() {
                return "NaN".into();
            } else if self.is_zero() {
                return format!("{:.*}", precision, 0);
            } else if self.is_infinite() {
                return format!("{}inf", if self.is_negative { "-" } else { "" });
            }

            if self.exponent.abs() < 1e9 {
                // Single exponential, e.g. 'e123.456'
                return format!(
                    "{}e{:.*}",
                    if self.is_negative { "-" } else { "" },
                    precision,
                    self.exponent
                );
            } else {
                // Double exponential, e.g. 'e1e123.456'
                return format!(
                    "{}e{}",
                    if self.is_negative { "-" } else { "" },
                    {
                        let exponent_exponent = self.exponent.abs().log10().floor();
                        let exponent_coefficient = self.exponent / 10_f64.powf(exponent_exponent);

                        format!("{:.*}e{:.0}", precision, exponent_coefficient, exponent_exponent)
                    }
                );
            }
        }
    }

    impl From<f64> for BigNum {
        fn from(value: f64) -> Self {
            return BigNum::from_f64(value);
        }
    }

    impl From<f32> for BigNum {
        fn from(value: f32) -> Self {
            return BigNum::from_f32(value);
        }
    }

    impl From<&str> for BigNum {
        fn from(value: &str) -> Self {
            return BigNum::from_string(value).unwrap();
        }
    }
}