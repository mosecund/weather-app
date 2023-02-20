// use structopt::StructOpt;
use exitfailure::{ ExitFailure };
use serde_derive::{ Deserialize, Serialize };
use reqwest::Url;
use tokio;
// #[derive(StructOpt)]
// struct Cli {
//     city: String,
//     country_code: String,
// }
#[derive(Deserialize, Serialize, Debug)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}
#[derive(Deserialize, Serialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}
#[derive(Deserialize, Serialize, Debug)]
struct Weather {
    details: Details,
}
#[derive(Deserialize, Serialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}
#[derive(Deserialize, Serialize, Debug)]
struct Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}
#[derive(Deserialize, Serialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
}
#[derive(Deserialize, Serialize, Debug)]
struct Clouds {
    all: i32,
}
#[derive(Deserialize, Serialize, Debug)]
struct Sys {
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32,
}

impl Forecast{
    async fn get(
        city: &&&str,
        country_code: &&str,
    ) -> Result<Self, ExitFailure>{
        let url:String = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid=472a194fcb435d4757231661d7b7a274", city, country_code);
        let url = Url::parse(&url)?;
        let resp = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;
        Ok(resp)
    }
}
#[tokio::main]
async fn main() -> Result<(), ExitFailure>{
    let country_code = "BE";
    let cities = ["Brussels", "Antwerp", "Ghent", "Bruges", "Liege", "Namur", "Mons", "Charleroi", "Leuven", "Mechelen"];
    for city in cities.iter() {
        let response = Forecast::get(&city, &country_code).await?;
        println!("city : {}, country code : {}, Temperature {}°C ",city,country_code,response.main.temp);
    }
    Ok(())
}

