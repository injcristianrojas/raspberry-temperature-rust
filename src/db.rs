use dotenv::dotenv;
use rusqlite::{Connection, Error, NO_PARAMS, Result, params};
use std::env;

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
    let conn = Connection::open(env::var("DATABASE_URL").expect("DATABASE_URL must be set"))?;
    let mut stmt = conn.prepare(
        "SELECT time_utc, time_local, temp_internal, temp_external, temp_owm, temp_owm_feels, condition \
        FROM temperatures \
        ORDER BY time_utc DESC \
        LIMIT 1"
    )?;
    let mut temps = stmt.query_map(NO_PARAMS, |row| {
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
    let conn = Connection::open(env::var("DATABASE_URL").expect("DATABASE_URL must be set"))?;
    conn.execute(
        "INSERT INTO temperatures \
        (time_utc, time_local, temp_internal, temp_external, temp_owm, temp_owm_feels, condition) \
        VALUES (datetime('now'), datetime('now', 'localtime'), ?1, ?2, ?3, ?4, ?5)",
        params![data.external, data.internal, data.owm_temp, data.owm_feels, data.owm_condition]
    )?;
    Ok(())
}