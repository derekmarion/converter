use std::io;

pub fn temperature() {
    println!("Enter a temperature type (F or C):");
    let mut temp_type = String::new();

    io::stdin().read_line(&mut temp_type).expect("Enter F or C");

    let temp_type: &str = &temp_type.trim();

    println!("Enter a temperature value:");
    let mut temp_value = String::new();

    io::stdin().read_line(&mut temp_value).expect("Failed to read line");

    let temp_value: f32 = temp_value.trim().parse().expect("Please enter a number value");

    if temp_type == "C"{
        let temp_converted:f32 = temp_value * 1.8 + 32.0;
        println!("{temp_converted}F degrees")
    } else {
        let temp_converted: f32 = (temp_value - 32.0) / 1.8;
        println!("{temp_converted}C degrees")
    }
}