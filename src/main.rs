use std::io;

fn main() {
    println!("----- Temperature Converter -----\n");
    println!("OPTION           //      FORMAT");
    println!("------------------------------------");
    println!("  0            //      Celcius");
    println!("  1           //      Fahrenheit");
    println!("------------------------------------\n");

    println!("Select an option: \n");
    let input = read_line_f32();
    println!("");

    if input == 0.0 {
        println!("Converting to Celcius...");
        println!("Please, input the fahrenheit temperature: ");
        let mut c_input: f32 = read_line_f32();
        let mut celcius: f32 = fahrenheit_to_celcius(c_input);
        println!("{} fahrenheit is: {} celcius.", c_input, celcius)
    }else {
        println!("Converting to Fahrenheit...");
        print!("Please, input the celcius temperature: ");
        let mut f_input: f32 = read_line_f32();
        let mut fahrenheit: f32 = celcius_to_fahrenheit(f_input);
        println!("{} celcius is: {} fahrenheit.", f_input, fahrenheit);
    }
}

fn read_line_f32() -> f32{
    loop {
            let mut input_string = String::new();
            println!("Input number:");

            io::stdin()
                .read_line(&mut input_string)
                .expect("Unable to read line.");
    
            let input_string: f32 = match input_string.trim().parse(){
                Ok(num) => return num,
                Err(_) => continue,
        };
    }
}

fn celcius_to_fahrenheit(celcius: f32) -> f32{
    let fahrenheit = (celcius * 1.8) + 32.0;
    return fahrenheit;
}

fn fahrenheit_to_celcius(fahrenheit: f32) -> f32{
    let celcius = (fahrenheit - 32.0) / 1.8;
    return  celcius;
}