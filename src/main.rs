/*
Title : Decathlon assignment
Short comment :
From the OpenWeatherMap API, get the current weather for the following cities:
Brussels, Antwerp, Ghent, Bruges, Liege, Namur, Mons, Charleroi, Leuven, Mechelen
Print the name of the city and the temperature in Celsius.
From the OpenWeatherMap API, get the forecast for tomorrow and the day after tomorrow for a city entered by the user.
Print the forecast for tomorrow and the day after tomorrow.

Done by: Ismael Secundar

Execution: Cargo run

For the first task, I used the API to get the current weather for the cities in Belgium.

For the second task, I used the API to get the forecast for the next 5 days. I used the latitude and longitude of the city entered by the user to get the forecast. I used the dt value to get the forecast for tomorrow and the day after tomorrow.
You can test the program with the city name "Brussels"(this is the city i tried with).
I used the serde crate to deserialize the JSON response into a struct.

I used the reqwest crate to make HTTP requests.

I used the std::io crate to get the city name from the user.


 */
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    list: Vec<ForecastEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ForecastEntry {
    dt: u64,
    main: MainForecast,
}

#[derive(Serialize, Deserialize, Debug)]
struct MainForecast {
    temp: f32,
}
#[derive(Debug, Deserialize)]
struct CurrentWeather {
    coord: Coord,
}


#[derive(Debug, Deserialize)]
struct Coord {
    lat: f64,
    lon: f64,
}
impl Forecast {
    async fn get(city: &str, country: &str) -> Result<Self, Box<dyn Error>> {
        let api_key = "472a194fcb435d4757231661d7b7a274";
        let url = format!(
            "http://api.openweathermap.org/data/2.5/forecast?q={},{}&units=metric&appid={}",
            city, country, api_key
        );
        let resp = reqwest::get(&url).await?;
        let json = resp.json::<Self>().await?;
        Ok(json)
    }

}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //loop till get the city name from the user
    println!("-------------------------------------Task 1-------------------------------------");
    let country_code = "BE";
    let cities = ["Brussels", "Antwerp", "Ghent", "Bruges", "Liege", "Namur", "Mons", "Charleroi", "Leuven", "Mechelen"];
    for city in cities.iter() {
        let response = Forecast::get(&city, &country_code).await?;
        println!("city : {}, country code : {}, Temperature {}°C ", city, country_code, response.list[0].main.temp);
    }
    let mut city_name = String::new();
    let api_key = "472a194fcb435d4757231661d7b7a274";
    println!("-------------------------------------Task 2-------------------------------------");

    loop {
        println!("Enter the city name for the forecast of tomorrow and day after tomorrow: ");
        std::io::stdin().read_line(&mut city_name).expect("Failed to read line");
        if city_name.trim().len() > 0 {
            break;
        }
    }
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city_name.trim(), api_key);

    let resp = reqwest::get(&url).await?.json::<CurrentWeather>().await?;
    let lat = resp.coord.lat;
    let lon = resp.coord.lon;
    // println!("lat : {}, lon : {}", lat, lon);

    let exclude = "minutely,hourly,alerts";
    let url = format!("http://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&exclude={}&units=metric&appid={}", lat, lon,  exclude,api_key);

    let resp = reqwest::get(&url).await?.json::<Forecast>().await?;
    //
    let now = resp.list[0].dt;
    let tomorrow = now + 86400;//24*60*60
    let day_after_tomorrow = tomorrow + 86400;

    for entry in resp.list {
        if entry.dt == tomorrow {
            println!("Tomorrow: {}°C", entry.main.temp);
        } else if entry.dt == day_after_tomorrow {
            println!("Day after tomorrow: {}°C", entry.main.temp);
        }
    }

    Ok(())
}
