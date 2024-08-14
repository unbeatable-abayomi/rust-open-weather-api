use std::io;
use serde::Deserialize;
use colored::*;

//Struct to deserialize the JSON from openWeatherMap Api

#[derive(Deserialize, Debug)]

struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

//Struct to represnt the weather description

#[derive(Deserialize,Debug)]
struct Weather{
    description: String,

}


//Struct to represent main weather parameters

#[derive(Deserialize,Debug)]
struct Main{
    temp: f64,
    humidity: f64,
    pressure: f64,
}
//9012358797

// Struct to represent wind information

#[derive(Deserialize,Debug)]
struct Wind{
    speed: f64,

}

//e43c60c1d74b3bccd1b3654eb8db80bd
//Function to get the weather information from the  OpenWeatherAPI

fn get_weather_info(city: &str, country_code: &str,api_key: &str) -> Result<WeatherResponse, reqwest::Error>{
  let url : String = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}", city, country_code, api_key);
let response = reqwest::blocking::get(&url)?;
let response_json : WeatherResponse = response.json::<WeatherResponse>()?;
Ok(response_json)
  //https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={API key}
}

//Fuction to display the weather information
fn display_weather_info(response: &WeatherResponse){
    //Extract wheater info from the response
    let description: &String = &response.weather[0].description;
    let temperature: f64 = response.main.temp;
    let humidity: f64 = response.main.humidity;
    let pressure: f64 = response.main.pressure;
   let wind_speed: f64 = response.wind.speed;

//FOrmatiting weather information into a string
let weather_text: String  = format!(
   "Weather in {}: {} {}
   > Temperature:{:.1}C
   > Humidity: {:.1}%,
   > Pressure: {:.1} hPa,
   > Wind Speed: {:.1} m/s",
   response.name,
   description,
   get_temp_emoji(temperature),
   temperature,
   humidity,
   pressure,
   wind_speed,
   
   

   //neting taken
);
//}
//Coloring the weather text based on weather conditions

let weather_text_colored: ColoredString = match description.as_str(){
    "clear sky" => weather_text.bright_yellow(),
    "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
    "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
    "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_yellow(),
    _ => weather_text.normal(),
};

println!("KSKSKK{}", weather_text_colored);
}
//Function get emoji
fn get_temp_emoji(temperature: f64) -> &'static str {
    // match temperature {
    //     temp if temp >= 30 =>  "🌞",
    //     temp if temp >= 20 => "��️",
    //     temp if temp >= 10 => "��",
    //     temp if temp >= 0 => "��️",
    //     _ => "��",
    // }

    if temperature < 0.0 {
        "🌞"
    }else if temperature >= 0.0 && temperature < 10.0   {
       "🌩️"
    }else if temperature >= 10.0 && temperature < 20.0 {
        "⛈"
    }else if temperature >= 20.0 && temperature < 30.0 {
        "⛅"
    }else{
        "🔥"
    }



}

fn main() {
    println!("{}" ,"Welcome to Weather Station!".bright_yellow());
    loop{
        println!("{}", "Please enter the name of the city:".bright_green());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read line");
        let city: &str = city.trim();

        println!("{}", "Please enter the country code (2 letters US for United states):".bright_green());

        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read line");
        let country_code: &str = country_code.trim();

        let api_key ="";

        match get_weather_info(&city, &country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response);
                //println!("{}", weather_text_colored);
            }
            //Err(e) => println!("Error: {}", e),
             Err (err) => {
                    eprintln!("Error: {}", err);
            }
        }

        println!("{}", "Do you want to check weather for another city? (Y/N):".bright_green());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Thank you for using weather software");
           break;
        }
    }
}
