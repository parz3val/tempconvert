// Simple Utility functions 

////////////CELSIUS////////////

/// Converts a f64 from Celsius to Fahrenheit
/// ```rust
/// use tempconvert::utils::c_to_f;
/// c_to_f(0.0);
/// // output
/// // 32.0
/// ```
pub fn c_to_f(c: f64) -> f64 {
    return c * 9.0 / 5.0 + 32.0;
}

/// Converts a f64 from Celsius to Kelvin
/// ```rust
/// use tempconvert::utils::c_to_k;
/// c_to_k(0.0);
/// // ouput --> 273.0
/// ```
pub fn c_to_k(c: f64) -> f64 {
    return c + 273.15;
}



////////////FAHRENHEIT////////////

/// Converts a f64 from Fahrenheit to Celsius 
/// ```rust 
/// use tempconvert::utils::f_to_c;
/// f_to_c(32.0);
/// // output --> 0.0
/// ```
pub fn f_to_c(f: f64) -> f64 {
    return (((f - 32.0) * 5.0 / 9.0) * 1000.0).round() / 1000.0;
}

/// Converts a f64 from Fahrenheit to Kelvin
/// ```rust
/// use tempconvert::utils::f_to_k;
/// f_to_k(32.0);
/// // output --> 273.0
/// 
pub fn f_to_k(f: f64) -> f64 {
    return (((f - 32.0) * 5.0 / 9.0 + 273.15) * 1000.0 ).round()/ 1000.0;
}

////////////KELVIN////////////

/// Converts a f64 from Kelvin to Celsius
/// ```rust
/// use tempconvert::utils::k_to_c;
/// k_to_c(273.0);
/// // output --> 0.0
/// ```
pub fn k_to_c(k: f64) -> f64 {
    return ((k - 273.15) * 1000.0).round() / 1000.0;
}


/// Converts a f64 from Kelvin to Fahrenheit
/// ```rust
/// use tempconvert::utils::k_to_f;
/// k_to_f(273.0);
/// // output --> 32.0
/// ```
pub fn k_to_f(k: f64) -> f64 {
    return (((k - 273.15) * 9.0 / 5.0 + 32.0) * 1000.0).round() / 1000.0;
}


