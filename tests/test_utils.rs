#[cfg(test)]
mod test_utils {

    const T_UNITS: [f64; 5] = [
        89.6,
        32.0,
        41.0,
        44.6,
        254.21,
    ];
            
    #[test]
    fn test_c_to_f() {
        let result_units = [
           193.28,
           89.6,
           105.8,
           112.28,
           489.578,
        ];
        for (idx, item) in T_UNITS.iter().enumerate() {
            assert_eq!(result_units[idx], tempconvert::utils::c_to_f(*item));
        }

    }

    #[test]
    fn test_c_to_k() {
        let results = [
            362.75,
            305.15,
            314.15,
            317.75,
            527.36,
        ];
        for (idx, item) in T_UNITS.iter().enumerate() {
            assert_eq!(results[idx],
                tempconvert::utils::c_to_k(*item)
            );
        }

    }

    #[test]
    fn test_f_to_k() {
        let results = [
            305.15,
            273.15,
            278.15,
            280.15,
            396.6,
        ];

        for (idx, item) in T_UNITS.iter().enumerate() {
            assert_eq!(
                results[idx],
                tempconvert::utils::f_to_k(*item)
            )
        }
    }
    
    #[test]
    fn test_f_to_c() {
        let results = [
            32.0,
            0.0,
            5.0,
            7.0,
            123.45,
        ];
        for (idx, item) in T_UNITS.iter().enumerate() {
            assert_eq!(
                results[idx],
                tempconvert::utils::f_to_c(*item)
            )
        }
    }

    #[test]
    fn test_k_to_c() {
        let results = [
            -183.55,
            -241.15,
            -232.15,
            -228.55,
            -18.94,
        ];
        for (idx, item) in T_UNITS.iter().enumerate() {
            assert_eq!(
                results[idx],
                tempconvert::utils::k_to_c(*item)
            )
        }
    }

    #[test]
    fn test_k_to_f() {
        let results = [
            -298.39,
            -402.07,
            -385.87,
            -379.39,
            -2.092,
        ];
        for (idx, item) in T_UNITS.iter().enumerate() {
            assert_eq!(
                results[idx],
                tempconvert::utils::k_to_f(*item)
            )
        }
    }
}