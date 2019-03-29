use std::io;

fn main() {
    let mut input_temp = String::new();

    println!("Temperature in Celsius:");

    io::stdin().read_line(&mut input_temp)
        .expect("Failed to read line");

    let input_temp = input_temp.trim().parse::<f64>()
        .expect("Wrong number");

    let output_temp = input_temp * 1.8 + 32.0;

    println!("{} degrees Celsius is {} degrees Fahrenheit", input_temp, output_temp);
}
