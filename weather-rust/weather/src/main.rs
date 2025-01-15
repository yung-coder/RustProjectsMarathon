use colored::*;
use serde::Deserialize;
use std::io;

#[derive(Deserialize, Debug)]

struct WeatherResponse {
    weatehr: Vec<Weather>,
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
