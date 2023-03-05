use std::num::ParseFloatError;
use std::str::FromStr;

use super::c::C;
use super::k::K;

/// # Provides Farenheit Unit F
/// - Can be used to print values as String
/// - Can be used to convert to C and K
/// How to use 
/// ```rust
/// use tempconvert::{F, C, K};
/// fn main() {
///    let f = F(32.0);
///    dbg!(f);
///    // ouput 
///    // F(32.0)
///    dbg!(f.to_string());
///    // output
///    // "32.0°F"
///    let c: C = f.into();
///    dbg!(c);
///    // output
///    // C(0.0)
///    let k: K = f.into();
///    dbg!(k);
///    // output
///    // K(273.0)
/// }
///```
#[derive(Debug, Copy, PartialEq)]
pub struct F(pub f64);


impl FromStr for F {
    type Err = ParseFloatError; // it is a requirement for the type
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(F(s.replace(" ","").parse::<f64>()?));
    }
}

impl ToString for F {
    fn to_string(&self) -> String {
        return format!("{}°F", self.0);
    }
}

  
impl From<usize> for F {
    fn from(i: usize) -> Self {
        return F(i as f64);
    }
}

impl From<f64> for F {
    fn from(f: f64) -> Self {
        return F(f);
    }
}

impl From<C> for F {
    fn from(c: C) -> Self {
        return Self(crate::utils::c_to_f(c.0));
    }
}

impl From<K> for F {
    fn from(k: K) -> Self {
        return Self(crate::utils::k_to_f(k.0))
    }
}