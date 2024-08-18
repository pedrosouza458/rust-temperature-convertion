use std::io;

fn main() {
    println!(
        "Enter 1 to convert fahrenheit to celsius \nEnter 2 to convert celsius to fahrenheit"
    );
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("expect to be option");

    let option = option.trim();

    if option == "1" {
        let celsius: f64;

        let mut input = String::new();
        println!("Please enter the fahrenheit temperature:");
        io::stdin()
            .read_line(&mut input)
            .expect("expect to be string");
        let fahrenheit: i64 = input.trim().parse().expect("expect to be string");

        celsius = (fahrenheit - 32) as f64 / 1.8;
        println!("The temperature in celsius is: {celsius}")
    }
    
    if option == "2" {
        let fahrenheit: f64;

        let mut input = String::new();
        println!("Please enter the celsius temperature:");
        io::stdin()
            .read_line(&mut input)
            .expect("expect to be string");
        let celsius: i64 = input.trim().parse().expect("expect to be string");

        fahrenheit = (celsius as f64 * 1.8) + 32 as f64;
        println!("The temperature in fahrenheit is: {fahrenheit}")
    }
}
