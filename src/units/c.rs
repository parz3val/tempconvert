
use std::num::ParseFloatError;
use std::str::FromStr;

use super::f::F;
use super::k::K;
/// # Provides Celsius Unit C
/// - Can be used to print values as string
/// - Can be used to convert to F and K
/// ## Example 
/// ```rust
/// use tempconvert::{F, C, K}
/// fn main() {
///   let c = C(0.0);
///   dbg!(c);
///   // output
///   // C(0.0)
///   dbg!(c.to_string());
///   // output
///   // "0.0°C"
///   let f: F = c.into();
///   dbg!(f);
///   // output
///   // F(32.0)
///   let k: K = c.into();
///   dbg!(k);
///   // output
///   // K(273.0)
///   }
#[derive(Debug, Copy, PartialEq)]
pub struct C(pub f64);

impl From<usize> for C {
    fn from(i: usize) -> Self {
        return C(i as f64);
    }
}

impl From<f64> for C {
    fn from(f: f64) -> Self {
        return C(f);
    }
}

impl FromStr for C {
    type Err = ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(C(s.replace(" ","").parse::<f64>()?));
    }
}


impl ToString for C {
    fn to_string(&self) -> String {
        return format!("{}°C", self.0);
    }
}
impl From<K> for C {
    fn from(k: K) -> Self {
        return Self(crate::utils::k_to_c(k.0));
    }
}

impl From<F> for C {
    fn from(f: F) -> Self {
        return Self(crate::utils::f_to_c(f.0));
    }
}


