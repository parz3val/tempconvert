/// # TempConvert Documentation: Units
/// ## How to use
pub mod units;

/// # Utils Documentation
pub mod utils;


pub use units::c::C as C;
pub use units::f::F as F;
pub use units::k::K as K;

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_units() {
        let f = F(32.0);
        let f_from32: F = 32.into();
        let f_from_float: F = 32.0.into();
        let f_to_c: C = f.into();
        let f_to_k: K = f.into();
        let k_to_c: C = f_to_k.into();

        assert_eq!(f, f_from32);
        assert_eq!(f, f_from_float);

        let c: C = 0.into();
        assert_eq!(c, f_to_c);
        assert_eq!(c, k_to_c);

        let k: K = c.into();


        let c_string = String::from("0°C");
        let k_string = String::from("273.15°K");
        let f_string = String::from("32°F");
        
        assert_eq!(f_string, f.to_string());
        assert_eq!(c_string, c.to_string());
        assert_eq!(k_string, k.to_string());


    }

   #[test] 
   fn test_utils() {
       let c = 0.0;
       let f = 32.0;
       let k = 273.15;
       assert_eq!(f, super::utils::c_to_f(c));
       assert_eq!(k, super::utils::c_to_k(c));
       assert_eq!(c, super::utils::f_to_c(f));
       assert_eq!(k, super::utils::f_to_k(f));
       assert_eq!(c, super::utils::k_to_c(k));
       assert_eq!(f, super::utils::k_to_f(k));}
}
