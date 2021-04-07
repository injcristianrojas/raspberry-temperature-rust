use chrono::Local;
use dotenv::dotenv;
use rusqlite::{Connection, Error, Result, params};
use std::env;
use std::fs;

#[derive(Serialize, Debug)]
pub struct Weather {
    pub time_utc: String,
    pub time_local: String,
    pub internal: f64,
    pub external: f64,
    pub owm_temp: f64,
    pub owm_feels: f64,
    pub owm_condition: String,
    pub latest_formatted: String,
}
pub struct TempData {
    pub internal: f64,
    pub external: f64,
    pub owm_temp: f64,
    pub owm_feels: f64,
    pub owm_condition: String,
}

pub fn get_latest_data() -> Result<Weather, Error> {
    dotenv().ok();
    let conn = Connection::open(env::var("DATABASE_FILE").expect("DATABASE_FILE must be set"))?;
    let mut stmt = conn.prepare(
        "SELECT time_utc, time_local, temp_internal, temp_external, temp_owm, temp_owm_feels, condition \
        FROM temperatures \
        ORDER BY time_utc DESC \
        LIMIT 1"
    )?;
    let mut temps = stmt.query_map([], |row| {
        Ok(
            Weather {
                time_utc: row.get(0)?,
                time_local: row.get(1)?,
                internal: row.get(2)?,
                external: row.get(3)?,
                owm_temp: row.get(4)?,
                owm_feels: row.get(5)?,
                owm_condition: row.get(6)?,
                latest_formatted: row.get(1)?,
            }
        )
    })?;
    return temps.next().unwrap();
}

pub fn insert_data(data: TempData) -> Result<()> {
    dotenv().ok();
    let conn = Connection::open(env::var("DATABASE_FILE").expect("DATABASE_FILE must be set"))?;
    conn.execute(
        "INSERT INTO temperatures \
        (time_utc, time_local, temp_internal, temp_external, temp_owm, temp_owm_feels, condition) \
        VALUES (datetime('now'), datetime('now', 'localtime'), ?1, ?2, ?3, ?4, ?5)",
        params![data.external, data.internal, data.owm_temp, data.owm_feels, data.owm_condition]
    )?;
    Ok(())
}

#[derive(Debug)]
struct WeatherJSON {
    time_local: String,
    internal: f64,
    external: f64,
}

#[derive(Serialize, Debug)]
struct JsonData {
    latest: String,
    labels: Vec<String>,
    internal: Vec<f64>,
    external: Vec<f64>
}

pub fn get_last_24() -> Result<()> {
    dotenv().ok();
    let conn = Connection::open(env::var("DATABASE_FILE").expect("DATABASE_FILE must be set"))?;
    let mut stmt = conn.prepare(
        "SELECT strftime('%H:%M', time_local), round(temp_internal, 1), round(temp_external, 1) FROM temperatures \
        WHERE time_local > datetime('now', '-1 day') AND strftime('%M', time_local) % 5 = 0 ORDER BY time_local"
    )?;
    let weather_rows = stmt.query_map([], |row| {
        Ok(WeatherJSON {
            time_local: row.get(0)?,
            internal: row.get(1)?,
            external: row.get(2)?,
        })
    })?;

    let mut jsondata = JsonData {
        latest: Local::now().format("%A, %B %-d, %H:%M").to_string(),
        labels: Vec::new(),
        internal: Vec::new(),
        external: Vec::new()
    };

    for weather_row in weather_rows {
        let weather = weather_row.unwrap();
        jsondata.labels.push(weather.time_local);
        jsondata.internal.push(weather.internal);
        jsondata.external.push(weather.external);
    }

    let serialized = serde_json::to_string(&jsondata).unwrap();
    fs::write("static/last24.json", serialized).expect("Unable to write file");
    println!("Graph data JSON written {}", Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

    Ok(())
}