use dotenv::dotenv;
use std::env;
use rusqlite::{Connection, Error, NO_PARAMS, Result};
use crate::Weather;

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