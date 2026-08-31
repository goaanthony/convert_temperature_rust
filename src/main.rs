use std::io;

// The formula to convert Celsius to Fahrenheit is: °F = (°C × 1.8) + 32
// La formule pour convertir Fahrenheit en Celsius est : °C = (°F − 32) / 1.8

fn main() {
    println!("=== HeatConvert ===");
    println!("1) Convert celsius to fahrenheit");
    println!("2) Convert fahrenheit to celsius");


    let user_request = read_match();
    
    match user_request {
        1 => {
            println!("Please enter the temperature in Celsius:");

            let celsius_request = read_heat();

            println!("Celsius is equal to Fahrenheit {} F°", convert_celsius_to_fahrenheit(celsius_request));
        },
        
        2 => {
            println!("Please enter the temperature in fahrenheit:");
            
            let fahrenheit_request = read_heat();
        
            println!("Fahrenheit is equal to Celsius {} C°", convert_fahrenheit_to_celsius(fahrenheit_request));
        },
        
        _ => println!("Only number are accepted")
    }
}


fn convert_celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn convert_fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

fn read_match() -> u32 {
    loop {
        let mut request = String::new();

        io::stdin()
            .read_line(&mut request)
            .expect("Echec de la lecture de l'entrée de l'utilisateur");
    
        let request: u32 = match request.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };
        return request;
    }
}

fn read_heat() -> f64 {
    loop {
        let mut request = String::new();

        io::stdin()
            .read_line(&mut request)
            .expect("Echec de la lecture de l'entrée de l'utilisateur");
    
        let request: f64 = match request.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };
        return request;
    }
}