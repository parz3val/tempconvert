#[cfg(test)]
mod test_f_units {
    use tempconvert::{F, C, K};

    #[test]
    fn test_f() {
        // create from integer 
        let f: F = "32".parse().unwrap();
        let f1: F = 32.into();
        println!("{:?}", f1);
        println!("{}", f.to_string());
        let f_test: F = F(32.0);

        let f_f: F = 32.0.into();
        assert_eq!(
            f,
            f_test
        );
        assert_eq!(
            f,
            f_f
        );
    }

    #[test]
    fn test_f_from_c() {
        let c: C = 0.into();
        let f: F = c.into();
        let f_test = F(32.0);

        assert_eq!(
            f,
            f_test
        )
    }

    #[test]
    fn test_f_from_k() {
        let k: K = 273.15.into();
        let f: F = k.into();
        let f_test = F(32.0);

        assert_eq!(
            f,
            f_test
        )
    }
}

#[cfg(test)]
mod test_c_units {
    use tempconvert::{F, C, K};
    #[test]
    fn test_c() {
        let c: C = 0.into();
        let k: K = 273.15.into();
        let f: F = 32.into();

        let k_to_c: C = k.into();
        let f_to_c: C = f.into();

        assert_eq!(
            c,
            k_to_c
        );
        assert_eq!(
            f_to_c,
            c,
        )
    }
    #[test]
    fn test_c_from_k() {
        let k: K = 273.15.into();
        let c: C = k.into();
        let c_t: C = 0.into();

        assert_eq!(
            c,
            c_t,
        )
    }
    #[test]
    fn test_c_from_f() {
        let f: F = 32.into();
        let c: C = f.into();
        let c_t: C = 0.into();
        assert_eq!(
            c,
            c_t,
        )
    }
}

#[cfg(test)]
mod test_k_units {
    use tempconvert::{F, C, K};
    #[test]
    fn test_k() {
        let k: K = 273.15.into();
        let c: C = 0.into();
        let f: F = 32.into();

        let c_to_k: K = c.into();
        let f_to_k: K = f.into();

        assert_eq!(
            k,
            c_to_k
        );
        assert_eq!(
            f_to_k,
            k,
        )
    }
    #[test]
    fn test_k_from_c() {
        let c: C = 0.into();
        let k: K = c.into();
        let k_t: K = 273.15.into();

        assert_eq!(
            k,
            k_t,
        )
    }
    #[test]
    fn test_k_from_f() {
        let f: F = 32.into();
        let k: K = f.into();
        let k_t: K = 273.15.into();
        assert_eq!(
            k,
            k_t,
        )
    }

}