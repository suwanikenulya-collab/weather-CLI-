//handles user interactions and program flow.(fetch data from the api and display it to the user)

use std::io;
use colored::*;

mod api; // Import the API module
use crate::api::get_weather_info;

fn main() {
    println!("{}", "Welcome to Weather Station!".bright_yellow());

    let api_key = "d8d31293bee740761c9ba933823c09ea"; // Keep it here for now

    loop {// keeps the program running until the user decides to exit and API outside of the loop
        println!("{}", "Please enter the name of the city:".bright_green());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input");
        let city = city.trim();

        println!("{}", "Please enter the country code (e.g., US for United States):".bright_green());
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input");
        let country_code = country_code.trim();

        match get_weather_info(city, country_code, api_key) {
            Ok(response) => {
                // Display logic kept simple for now
                println!("Weather in {}: {:?}", response.name, response.weather[0].description);
                println!("Temperature: {:.1}°C | Humidity: {:.1}% | Pressure: {:.1} hPa | Wind: {:.1} m/s",
                    response.main.temp, response.main.humidity, response.main.pressure, response.wind.speed);
            }
            Err(err) => eprintln!("Error fetching weather data: {}", err),
        }

        println!("{}", "Do you want to search for another city? (yes/no)".bright_green());
        let mut input = String::new();
        
        io::stdin().read_line(&mut input).expect("Failed to read input");
        if input.trim().to_lowercase() != "yes" {
            println!("Thank you for using Weather Station!");
            break;
        }
    }
}
