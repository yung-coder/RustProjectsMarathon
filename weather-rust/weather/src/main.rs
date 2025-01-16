use colored::*;
use serde::Deserialize;
use std::io;

#[derive(Deserialize, Debug)]

struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct of Weatehr
#[derive(Deserialize, Debug)]

struct Weather {
    description: String,
}

// struct for main
#[derive(Deserialize, Debug)]

struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

// struct for wind
#[derive(Deserialize, Debug)]

struct Wind {
    speed: f64,
}

// function for api call

fn get_weather(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?;

    let response_json = response.json::<WeatherResponse>()?;

    Ok(response_json)
}

// function to display the weatehr info

fn display_info(response: &WeatherResponse) {
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    // format

    let weather_text = format!(
        "Weather in {}: {} 
        > Temperature: {:.1}°C, 
        > Humidity: {:.1}%, 
        > Pressure: {:.1} hPa, 
        > Wind Speed: {:.1} m/s",
        response.name, description, temperature, humidity, pressure, wind_speed,
    );

    let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => {
            weather_text.dimmed()
        }
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    // Printing the colored weather information
    println!("{}", weather_text_colored);
}

fn main() {
    println!("{}", "Welcome".bright_yellow());

    loop {
        // city
        println!("{}", "Please enter the name of the city".bright_green());
        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Failde to read input");
        let city = city.trim();

        // country

        println!("{}", "Please enter the name of the country".bright_green());
        let mut country_code = String::new();
        io::stdin()
            .read_line(&mut country_code)
            .expect("Failde to read input");
        let country_code = country_code.trim();

        // Get your API key from OpenWeatherMap
        let api_key = "dd49c635696b58f749f3dd5ca6dfbc05";

        // Calling the function to fetch weather information
        match get_weather(&city, &country_code, api_key) {
            Ok(response) => {
                display_info(&response); // Displaying weather information
            }
            Err(err) => {
                eprintln!("Error: {}", err); // Printing error message in case of failure
            }
        }

        println!(
            "{}",
            "Do you want to search for weather in another city? (yes/no):".bright_green()
        ); // Prompting user to continue or exit
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input"); // Reading user input for continuation
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Thank you for using our software!");
            break; // Exiting the loop if user doesn't want to continue
        }
    }
}
