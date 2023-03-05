use tempconvert::{F, C, K};

fn print_green_text(text: &str) {
    println!("{}{}{}", "\x1b[32m", text, "\x1b[0m");
}

fn print_pink_text(text: &str) {
    println!("{}{}{}", "\x1b[35m", text, "\x1b[0m");
}

fn print_purple_text(text: &str) {
    println!("{}{}{}", "\x1b[34m", text, "\x1b[0m");
}

fn print_teal_text(text: &str) {
    println!("{}{}{}", "\x1b[36m", text, "\x1b[0m");
}
enum Temp {
    F(F),
    C(C),
    K(K),
}

fn main() {
    let mut input_unit: String = String::new();
    let mut input_temp: String = String::new();

    println!("Enter your temperature unit (c, f, or k):");
    std::io::stdin().read_line(&mut input_unit).expect("Failed to read line");
    println!("Enter your temperature:");
    std::io::stdin().read_line(&mut input_temp).expect("Failed to read temperature");

    let unit : Temp = match input_unit.clone().trim() {
        "f" => {
            let f: F = input_temp.trim().parse().unwrap();
            Temp::F(f)
            
        },
        "c" => {
            let c: C = input_temp.trim().parse().unwrap();
            Temp::C(c)
        },
        "k" => {
            let k: K = input_temp.trim().parse().unwrap();
            Temp::K(k)
        },
        _ => {
            println!("Invalid unit");
            return;
        }
        
    };
    convert_unit_print(unit)


}

fn convert_unit_print(unit: Temp) {
    match unit {
        Temp::F(f) => {
            let c: C = f.into();
            let k: K = f.into();
            print_teal_text("Fahrenheit to Celsius and Kelvin:");
            print_pink_text(c.to_string().trim());
            print_purple_text(k.to_string().trim());
        },
        Temp::C(c) => {
            let f: F = c.into();
            let k: K = c.into();
            print_teal_text("Celsius to Fahrenheit and Kelvin:");
            print_purple_text(k.to_string().trim());
            print_green_text(f.to_string().trim());
        },
        Temp::K(k) => {
            let f: F = k.into();
            let c: C = k.into();
            print_teal_text("Kelvin to Fahrenheit and Celsius:");
            print_green_text(f.to_string().trim());
            print_pink_text(c.to_string().trim());
        },
    }
}