# `🌡️ 🥶 tempconvert 🥶 🌡 ️`

## Easy Temperature Conversion and Types

- Kelvin - K - 🥶
- Celsius - C - 🥵
- Fahrenheit - F - 🤒

Provides  easy types for temperature conversion.

## 💻  Install

Add package in your Cargo.toml to use

```toml
[dependencies]
tempconvert = "0.1.0"
```

Can be initialized from integer, float or String

```rust
use tempconvert:: {K, C, F};
fn main() {
    // create Kelvin from string by parsing
    let k: K = "273".parse().unwrap();

    // Gives beautified string with K
    let k_string: String = k.to_string();
    
    // Convert into celsius easily
    let c1: C = k.into();

    // Create Kelivin  from i32
    let k1: K = 273.into();

    // convert to Fareinheit
    let f1 = k.into::<F>();

    let k2: K = 273.0.into();

    let f2: F = K.into();

    println!("{}", k.to_string());
    println!("{}", k1.to_string());
    println!("{}", k2.to_string());
    println!("{}", k.into::<C>.to_string())

}

```

## 👩🏻‍💻🧑🏼‍💻 How to use

### Convert temperature from one unit to other

You can either initialize the temperature with the tuple struct or use the `into` method to convert from differnt types.
All of the units support conversion from integer, float and the String type

```rust
    use tempconvert::{C, F, K};
    fn main() {
        let f1: F = 32.into();
        let f2: F = 32.0.into();
        let f = F(32.0);
        dbg!(f);
        // ouput 
        // F(32.0)
        dbg!(f.to_string());
        // output
        // "32.0°F"
        let c: C = f.into();
        dbg!(c);
        // output
        // C(0.0)
        let k: K = f.into();
        dbg!(k);
        // output
        // K(273.0)
    }
```

### 📝📝 Use utils to covnert f64 to different units easily

`tempconvert` has independent utils module
which can be used to convert f64 to different units without the types

```rust
use tempconvert::utils::{
    c_to_f,
    c_to_k,
    f_to_c,
    f_to_k,
    k_to_c,
    k_to_f
};


fn main() {
    dbg!(c_to_k(0.0));
    dbg!(c_to_f(0.0));
    dbg!(f_to_c(32.0));
    dbg!(k_to_c(273.0));
    dbg!(k_to_f(273.0));
}
```
