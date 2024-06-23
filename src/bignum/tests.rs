#[cfg(test)]
mod tests {
    use crate::bignum::BigNum;

    fn almost_equal(a: f64, b: f64, tolerance: f64) -> bool {
        if a.is_infinite() && b.is_infinite() {
            return a == b;
        }

        return (a - b).abs() < tolerance;
    }

    #[test]
    fn test_from_exponent() {
        let decimal = BigNum::from_exponent(2.0, false);

        assert_eq!(decimal.is_negative, false);
        assert_eq!(decimal.exponent, 2.0);
    }

    #[test]
    fn test_from_f64() {
        let decimal = BigNum::from_f64(100.0);

        assert_eq!(decimal.is_negative, false);
        assert!(almost_equal(decimal.exponent, 2.0, 1e-9));
    }

    #[test]
    fn test_from_string() {
        use std::f64::INFINITY as INF;
        #[allow(overflowing_literals)]
        let tests = vec![
            ("Zero",            "0.0",          -INF,   false),
            ("Negative Zero",   "-0.0",         -INF,   true),
            ("Normal",          "100.0",         2.0,   false),
            ("Negative",        "-100.0",        2.0,   true),
            ("Small",           "0.01",         -2.0,   false),
            ("Scientific",      "1e2",           2.0,   false),
            ("Sci. Small",      "1e-2",         -2.0,   false),
            ("Sci. Negative",   "-1e2",          2.0,   true),
            ("Sci. Huge",       "1e500",       500.0,   false),
            ("Sci. Huge Neg.",  "-1e500",      500.0,   true),
            ("Sci. Tiny",       "1e-500",     -500.0,   false),
            ("Sci. Massive",    "1e1e200",     1e200,   false),
            ("Sci. Msv. Neg.",  "-1e1e200",    1e200,   true),
            ("Sci. Miniscule",  "1e-1e200",   -1e200,   false),
            ("Sci. Mini Neg.",  "-1e-1e200",  -1e200,   true),
            ("Logarithm",       "e10",          10.0,   false),
            ("Log Negative",    "-e10",         10.0,   true),
            ("Log Small",       "e-10",        -10.0,   false),
            ("Log Huge",        "e500",        500.0,   false),
            ("Log Huge Neg.",   "-e500",       500.0,   true),
            ("Log Tiny",        "e-500",      -500.0,   false),
            ("Log Massive",     "e1e200",      1e200,   false),
            ("Log Msv. Neg.",   "-e1e200",     1e200,   true),
            ("Log Miniscule",   "e-1e200",    -1e200,   false),
            ("Log Mini. Neg.",  "-e-1e200",   -1e200,   true),
            ("Log Decimal",     "e123.456",    123.456, false)
        ];

        for (name, input, expected_exponent, expected_is_negative) in tests {
            let decimal = BigNum::from_string(input).unwrap();

            assert_eq!(decimal.is_negative, expected_is_negative, "{}: sign check", name);
            assert!(almost_equal(decimal.exponent, expected_exponent, 1e-9),
                "{}: exponent check (got {}, expected {})", name, decimal.exponent, expected_exponent);
        }
    }

    #[test]
    fn test_from_string_with_invalid_input() {
        let t2 = BigNum::from_string("1e");
        assert!(t2.is_err());

        let t3 = BigNum::from_string("1e-");
        assert!(t3.is_err());

        let t4 = BigNum::from_string("1e+");
        assert!(t4.is_err());

        let t5 = BigNum::from_string("e");
        assert!(t5.is_err());

        let t6 = BigNum::from_string("-");
        assert!(t6.is_err());
    }

    #[test]
    fn test_to_f64() {
        let t1 = BigNum::from_exponent(2.0, false);
        assert!(almost_equal(t1.to_f64().unwrap(), 100.0, 1e-9));

        let t2 = BigNum::from_exponent(-2.0, false);
        assert!(almost_equal(t2.to_f64().unwrap(), 0.01, 1e-9));

        let t3 = BigNum::from_exponent(2.0, true);
        assert!(almost_equal(t3.to_f64().unwrap(), -100.0, 1e-9));
    }

    #[test]
    fn test_string_conversions() {
        let tests = vec![
            ("Zero",            "0"),
            ("Sci. Huge",       "1.00e500"),
            ("Sci. Huge Neg.",  "-1.00e500"),
            ("Sci. Tiny",       "1.00e-500"),
            ("Sci. Massive",    "1e1.00e200"),
            ("Sci. Msv. Neg.",  "-1e1.00e200"),
            ("Sci. Miniscule",  "1e-1.00e200"),
            ("Sci. Mini Neg.",  "-1e-1.00e200"),
            ("Logarithm",       "e10.000"),
            ("Log Negative",    "-e10.000"),
            ("Log Small",       "e-10.000"),
            ("Log Huge",        "e500.000"),
            ("Log Huge Neg.",   "-e500.000"),
            ("Log Tiny",        "e-500.000"),
            ("Log Massive",     "e1.000e200"),
            ("Log Msv. Neg.",   "-e1.000e200"),
            ("Log Miniscule",   "e-1.000e200"),
            ("Log Mini. Neg.",  "-e-1.000e200"),
            ("Log Decimal",     "e123.456")
        ];

        for (name, string) in tests {
            let decimal = BigNum::from_string(string).unwrap();

            let result = if name.to_lowercase().contains("log") {
                decimal.to_string_log(3)
            } else {
                decimal.to_string_sci(2)
            };

            assert_eq!(result, string, "{}: string conversion check", name);
        }
    }

    #[test]
    fn test_comparisons() {
        use std::cmp::Ordering;

        let tests = vec![
            ("Signed Zero",         "-0",       "0",        Some(Ordering::Less)),
            ("Equal Zero",          "0",        "0",        Some(Ordering::Equal)),
            ("Different Signs 1",   "-100",     "100",      Some(Ordering::Less)),
            ("Different Signs 2",   "100",      "-100",     Some(Ordering::Greater)),
            ("Infinities 1",        "inf",      "inf",      None),
            ("Infinities 2",        "inf",      "-inf",     Some(Ordering::Greater)),
            ("Infinities 3",        "-inf",     "inf",      Some(Ordering::Less)),
            ("Infinities 4",        "-inf",     "-inf",     None),
            ("Non-numbers 1",       "nan",      "nan",      None),
            ("Non-numbers 2",       "nan",      "inf",      None),
            ("Non-numbers 3",       "0",        "nan",      None),
            ("Non-numbers 4",       "-inf",     "nan",      None),
            ("Non-numbers 5",       "e500",     "nan",      None)
        ];

        for (name, a, b, expected) in tests {
            let a = BigNum::from_string(a).unwrap();
            let b = BigNum::from_string(b).unwrap();

            assert_eq!(a.partial_cmp(&b), expected, "Comparison check: {}", name);
        }
    }

    #[test]
    fn test_addition() {
        let tests = vec![
            // Name,    A,          B,          Expected,       Tolerance
            ("Zero",    "0",        "0",        "0",            "e-9"),
            ("Two",     "1",        "1",        "2",            "e-9"),
            ("Meme",    "9",        "10",       "19",           "e-9"),
            ("Neg 1",   "-1",       "1",        "0",            "e-9"),
            ("Neg 2",   "1",        "-1",       "0",            "e-9"),
            ("Neg 3",   "-1",       "-1",       "-2",           "e-9"),
            ("Neg 4",   "-1",       "-9",       "-10",          "e-9"),
            ("Neg 5",   "-9",       "-1",       "-10",          "e-9"),
            ("Neg 6",   "21",       "-25",       "-4",           "e-9"),
            ("Neg 7",   "42",       "-62",       "-20",          "e-9"),
            ("Neg 8",   "e6",       "-e7",      "-9e6",         "e2"),
            ("Tiny 1",  "e-4",      "e-5",      "1.1e-4",       "e-12"),
            ("Tiny 2",  "e-5",      "e-4",      "1.1e-4",       "e-12"),
            ("Tiny 3",  "2e-4",     "3e-5",     "2.3e-4",       "e-12"),
            ("Tiny 4",  "6e-4",     "5e-4",     "1.1e-3",       "e-12"),
            ("Tiny 5",  "e-500",    "e-600",    "e-500",        "e-510"),
            ("Huge 1",  "e308",     "e309",     "1.1e309",      "e300"),
            ("Huge 2",  "e500",     "e530",     "e530",         "e500"),
            ("Huge 3",  "e1e12",    "e9e15",    "e9e15",        "e9e15"),
            ("Huge 4",  "e1e200",   "e2e200",   "e2e200",       "e1e200"),
            ("Huge 5",  "e1e308",   "e1e308",   "e1e308",       "e1e308")
        ];

        for (name, a, b, expected, tolerance) in tests {
            let a = BigNum::from_string(a).unwrap();
            let b = BigNum::from_string(b).unwrap();
            let expected = BigNum::from_string(expected).unwrap();
            let tolerance = BigNum::from_string(tolerance).unwrap();

            let result = a + b;

            assert!(result.almost_equal(&expected, &tolerance),
                "Addition check: {} (expected {}, got {})", name, expected, result);
        }
    }

    #[test]
    fn test_subtraction() {
        let tests = vec![
            // Name,    A,          B,          Expected,       Tolerance
            ("Zero",    "0",        "0",        "0",            "e-9"),
            ("Two",     "1",        "1",        "0",            "e-9"),
            ("Neg 1",   "9",        "10",       "-1",           "e-9"),
            ("Neg 2",   "-1",       "1",        "-2",           "e-9"),
            ("Neg 3",   "1",        "-1",       "2",            "e-9"),
            ("Neg 4",   "-1",       "-1",       "0",            "e-9"),
            ("Neg 5",   "-1",       "-9",       "8",            "e-9"),
            ("Neg 6",   "-9",       "-1",       "-8",           "e-9"),
            ("Tiny 1",  "e-4",      "e-5",      "9e-5",         "e-12"),
            ("Tiny 2",  "e-5",      "e-4",      "-9e-5",        "e-12"),
            ("Tiny 3",  "2e-4",     "3e-5",     "1.7e-4",       "e-12"),
            ("Tiny 4",  "6e-4",     "5e-4",     "1e-4",         "e-12"),
            ("Tiny 5",  "e-500",    "e-600",    "e-500",        "e-510"),
            ("Huge 1",  "e308",     "e309",     "-9e308",       "e300"),
            ("Huge 2",  "e500",     "e530",     "-e530",       "e500"),
            ("Huge 3",  "e1e12",    "e9e15",    "-e9e15",        "e9e15"),
            ("Huge 4",  "e1e200",   "e2e200",   "-e2e200",     "e1e200"),
            ("Huge 5",  "e1e308",   "e1e308",   "0",            "e-12")
        ];

        for (name, a, b, expected, tolerance) in tests {
            let a = BigNum::from_string(a).unwrap();
            let b = BigNum::from_string(b).unwrap();
            let expected = BigNum::from_string(expected).unwrap();
            let tolerance = BigNum::from_string(tolerance).unwrap();

            let result = a - b;

            assert!(result.almost_equal(&expected, &tolerance),
                "Subtraction check: {} (expected {}, got {})", name, expected, result);
        }
    }

    #[test]
    fn test_multiplication() {
        let tests = vec![
            // Name,    A,          B,          Expected,       Exponent tolerance
            ("Zero",    "0",        "0",        "0",            1e-13),
            ("Two",     "1",        "2",        "2",            1e-13),
            ("Neg 1",   "1",        "-1",       "-1",           1e-13),
            ("Neg 2",   "-1",       "1",        "-1",           1e-13),
            ("Neg 3",   "-1",       "-1",       "1",            1e-13),
            ("Neg 4",   "-1",       "-9",       "9",            1e-13),
            ("Neg 5",   "-9",       "-1",       "9",            1e-13),
            ("Neg 6",   "21",       "-25",      "-525",         1e-13),
            ("Neg 7",   "42",       "-62",      "-2604",        1e-13),
            ("Neg 8",   "e6",       "-e7",      "-e13",         1e-10),
            ("Tiny 1",  "e-4",      "e-5",      "e-9",          1e-10),
            ("Tiny 2",  "e-5",      "e-4",      "e-9",          1e-10),
            ("Tiny 3",  "2e-4",     "3e-5",     "6e-9",         1e-10),
            ("Tiny 4",  "6e-4",     "5e-4",     "3e-7",         1e-10),
            ("Tiny 5",  "e-500",    "e-600",    "e-1100",       1e-6),
            ("Huge 1",  "e308",     "e309",     "e617",         1e-8),
            ("Huge 2",  "e500",     "e530",     "e1030",        1e-6),
            ("Huge 3",  "e1e12",    "e9e15",    "e9.001e15",    1e+10),
            ("Huge 4",  "e1e200",   "e2e200",   "e3e200",       1e190),
            ("Huge 5",  "e8e307",   "e9e307",   "e1.7e308",     1e300)
        ];

        for (name, a, b, expected, tolerance) in tests {
            let a = BigNum::from_string(a).unwrap();
            let b = BigNum::from_string(b).unwrap();
            let expected = BigNum::from_string(expected).unwrap();

            let result = a * b;

            let result_exponent = result.exponent;

            assert_eq!(result.is_negative, expected.is_negative,
                "Multiplication check: {} (sign check)", name);
            assert!(almost_equal(result_exponent, expected.exponent, tolerance),
                "Multiplication check: {} (exponent check: got {}, expected {})", name, result_exponent, expected.exponent);
            
            println!("Test {} succeeded. Distance from expected: {}/{}",
                name, (result_exponent - expected.exponent).abs(), tolerance
            );
        }
    }

    #[test]
    fn test_division() {
        let tests = vec![
            // Name,            A,                  B,          Expected,       Exponent tolerance
            ("Two",             "2",                "1",        "2",                1e-13),
            ("Neg 1",           "1",                "-1",       "-1",               1e-13),
            ("Neg 2",           "-1",               "1",        "-1",               1e-13),
            ("Neg 3",           "-1",               "-1",       "1",                1e-13),
            ("Small",           "0.1",              "0.2",      "0.5",              1e-13),
            ("Zero Dividend",   "0",                "5",        "0",                1e-13),
            ("Zero Divisor",    "5",                "0",        "inf",              1e-13),
            ("Large",           "e308",             "1",        "e308",             1e-13),
            ("Precision",       "1.0000000000001",  "1",        "1.0000000000001",  1e-13),
            ("Recurring",       "1",                "3",        "0.3333333333333",  1e-13),
            ("Negative",        "-2",               "-2",       "1",                1e-13),
            ("Mixed Sign 1",    "-2",               "1",        "-2",               1e-13),
            ("Mixed Sign 2",    "2",                "-1",       "-2",               1e-13),
        ];
        for (name, a, b, expected, tolerance) in tests {
            let a = BigNum::from_string(a).unwrap();
            let b = BigNum::from_string(b).unwrap();
            let expected = BigNum::from_string(expected).unwrap();
            let result = a / b;
            let result_exponent = result.exponent;
            assert_eq!(result.is_negative, expected.is_negative,
                "Division check: {} (sign check)", name);
            assert!(almost_equal(result_exponent, expected.exponent, tolerance),
                "Division check: {} (exponent check: got {}, expected {})", name, result_exponent, expected.exponent);
            println!("Test {} succeeded. Distance from expected: {}/{}",
                name, (result_exponent - expected.exponent).abs(), tolerance
            );
        }
    }

    #[test]
    fn test_modulo() {
        let tests = vec![
            // Name,              A,                  B,          Expected,       Exponent tolerance
            ("Mod Zero",          "0",                "5",        "0",                1e-13),
            ("Mod by Zero",       "5",                "0",        "0",                1e-13),
            ("Positive",          "10",               "3",        "1",                1e-13),
            ("Negative Dividend", "-10",              "3",        "-1",               1e-13),
            ("Negative Divisor",  "10",               "-3",       "-1",               1e-13),
            ("Both Negative",     "-10",              "-3",       "1",                1e-13),
            ("Large Numbers",     "1E20",             "1E10",     "0",                1e-13),
            ("Small Dividend",    "0.1",              "0.2",      "0.1",              1e-13),
            ("Precision",         "1.0000000000001",  "1",        "0.0000000000001",  1e-3),
            ("Recurring",         "1",                "3",        "1",                1e-13)
        ];

        for (name, a, b, expected, tolerance) in tests {
            let a = BigNum::from_string(a).unwrap();
            let b = BigNum::from_string(b).unwrap();
            let expected = BigNum::from_string(expected).unwrap();
            let result = a % b;
            let result_exponent = result.exponent;
            assert_eq!(result.is_negative, expected.is_negative,
                "Modulo check: {} (sign check)", name);
            assert!(almost_equal(result_exponent, expected.exponent, tolerance),
                "Modulo check: {} (exponent check: got {}, expected {})", name, result_exponent, expected.exponent);
            println!("Test {} succeeded. Distance from expected: {}/{}",
                name, (result_exponent - expected.exponent).abs(), tolerance
            );
        }
    }
}