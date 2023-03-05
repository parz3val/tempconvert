
use std::num::ParseFloatError;
use std::str::FromStr;

use super::c::C;
use super::f::F;
/// # Provides Kelvin Unit K
/// - Can be used to print values as string 
/// - Can be used to convert to F and C
/// ## Example
/// ```rust
/// use tempconvert::units::{F, C, K};
/// fn main() {
///  let k = K(273.0);
///  let k2: K = 273.into();
///  dbg!(k);
///  // output
///  // K(273.0)
///  dbg!(k.to_string());
///  // output
///  // "273.0K"
///  let f: F = k.into();
///  dbg!(f);
///  // output
///  // F(32.0)
///  let c: C = k.into();
///  dbg!(c);
///  // output
///  // C(0.0)
///  }
///  ```
#[derive(Debug, Copy, PartialEq)]
pub struct K(pub f64);



impl From<usize> for K {
    fn from(i: usize) -> Self {
        return K(i as f64);
    }
}

impl From<f64> for K {
    fn from(f: f64) -> Self {
        return K(f);
    }
}
impl FromStr for K {
    type Err = ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(K(s.replace(" ","").parse::<f64>()?));
    }
}

impl ToString for K {
    fn to_string(&self) -> String {
        return format!("{}°K", self.0);
    }
}

impl From<C> for K {
    fn from(c: C) -> Self {
        return Self(crate::utils::c_to_k(c.0));
    }
}

impl From<F> for K {
    fn from(f: F) -> Self {
        return Self(crate::utils::f_to_k(f.0));
    }
}
