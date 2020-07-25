use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct WeatherResponse {
    is_available : bool,
    data_cadran : Vec<WeatherCadran> // Flatten?
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct WeatherCadran {
    niveau_pluie : i8
}

async fn _get_weather() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://www.meteofrance.com/mf3-rpc-portlet/rest/pluie/012620")
        .await?
        // .json::<HashMap<String, String>>()
        .json::<WeatherResponse>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

async fn get_weather_mock() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("test/meteo.json")?;
    let reader = BufReader::new(file);
    let w : WeatherResponse = serde_json::from_reader(reader)?;

    println!("{:#?}", w);
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_weather_mock().await
}
