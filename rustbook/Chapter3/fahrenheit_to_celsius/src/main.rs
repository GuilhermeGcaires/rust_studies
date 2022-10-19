use std::io;



fn main() {
    let mut option  = String::new();

    println!("Press 1 to convert Fahrenheit to Celsius");
    println!("Press 2 to convert Celsius to Fahrenheit");

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    let option: i8 = option.trim().parse().expect("Please type a number");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f32 = temperature
        .trim()
        .parse()
        .expect("Please type a number");

    match option {
        1 => println!("{}", fahrenheit_converter(temperature)),
        2 => println!("{}", celsius_converter(temperature)),
        _ => panic!("Fodeu")
    }; 
}

fn fahrenheit_converter(temperature: f32) -> f32 {
    (temperature-32.0) * (5.0/9.0) 
}

fn celsius_converter(temperature: f32) -> f32 {
    (temperature * (9.0/5.0)) + 32.0
}
