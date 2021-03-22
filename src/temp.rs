use chrono::Local;
use dotenv::dotenv;
use serde_json::Value;
use std::env;
use std::error::Error;
use w1_therm_reader::{convert_to_metric, read_from_file};

use crate::db::{TempData, insert_data};

const EXTERNAL_DEVICE: &str = "/sys/bus/w1/devices/28-01191bb88a82/w1_slave";
const INTERNAL_DEVICE: &str = "/sys/bus/w1/devices/28-3c01b556c9c2/w1_slave";

struct OwmData {
    owm_temp: f64,
    owm_feels_like: f64,
    current_condition: String,
}

pub fn get_temperature_data() -> TempData {
    let owm_api_data = match get_owm_api_data() {
        Ok(data) => data,
        Err(_) => OwmData {
            owm_temp: -273.15,
            owm_feels_like: -273.15,
            current_condition: "Error".to_string(),
        },
    };
    let tempdata: TempData = TempData {
        internal: get_internal_temperature_data(),
        external: get_external_temperature_data(),
        owm_temp: owm_api_data.owm_temp,
        owm_feels: owm_api_data.owm_feels_like,
        owm_condition: owm_api_data.current_condition,
    };
    tempdata
}

fn get_temperature_data_from_device(device: &str) -> f64 {
    let t = read_from_file(device).unwrap();
    return convert_to_metric(t) as f64;
}

fn get_internal_temperature_data() -> f64 {
    return get_temperature_data_from_device(INTERNAL_DEVICE);
}

fn get_external_temperature_data() -> f64 {
    return get_temperature_data_from_device(EXTERNAL_DEVICE);
}

fn dekelvinize(k: &Value) -> f64 {
    k.to_string().parse::<f64>().unwrap() - 273.15
}

fn get_owm_api_data() -> Result<OwmData, Box<dyn Error>> {
    dotenv().ok();
    let owmapi_location_id = env::var("OWMAPI_LOCATION_ID").expect("OWMAPI_LOCATION_ID must be set");
    let owmapi_key = env::var("OWMAPI_KEY").expect("OWMAPI_KEY must be set");
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?id={}&appid={}",
        owmapi_location_id,
        owmapi_key
    );
    let data_string = reqwest::blocking::get(&url)?.text()?;
    let data: Value = serde_json::from_str(&data_string)?;
    Ok(OwmData {
        owm_temp: dekelvinize(&data["main"]["temp"]),
        owm_feels_like: dekelvinize(&data["main"]["feels_like"]),
        current_condition: data["weather"][0]["description"].to_string().replace("\"", ""),
    })
}

pub fn get_and_process_data() {
    let tempdata: TempData = get_temperature_data();
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_data = format!(
        "In {}|Out {}|OWM-Temp {}|OWM-Feels {}|Cond: {}",
        tempdata.internal,
        tempdata.external,
        tempdata.owm_temp,
        tempdata.owm_feels,
        tempdata.owm_condition
    );

    match insert_data(tempdata) {
        Ok(()) => println!("{}: {}", now, log_data),
        Err(error) => println!("{}: FAILED. Reason: {}. Data: {}", now, error, log_data),
    }
}
