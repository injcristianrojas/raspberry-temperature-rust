use chrono::NaiveDateTime;
use pwr_hd44780::Hd44780;
use pwr_hd44780::frontends::Direct;

use crate::db::{get_latest_data, Weather};

pub struct WeatherDataForDisplay {
    time_utc: String,
    time_local: String,
    internal: String,
    external: String,
    owm_temp: String,
    owm_feels: String,
    owm_condition: String,
}

pub trait Create {
    fn show_data(&mut self, num: i32) -> Result<(), Box<dyn std::error::Error>>;
    fn set_first_time_data(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn fmt_temp(&mut self, temp: f64) -> String;
    fn fmt_date_to_time(&mut self, date: &str) -> String;
    fn struct_weather_data_for_display (&mut self, w: Weather) -> WeatherDataForDisplay;
}

pub struct LCDDisplay {
    pub lcd: Direct
}

pub fn createlcd() -> LCDDisplay {
    let lcd_bus = pwr_hd44780::I2CBus::new("/dev/i2c-1", 0x27).unwrap();
    let lcd = pwr_hd44780::DirectLcd::new(Box::new(lcd_bus), 20, 4).unwrap();
    LCDDisplay {
        lcd
    }
}

impl Create for LCDDisplay {
    
    fn show_data(&mut self, num: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.lcd.clear()?;
        self.lcd.print(num.to_string())?;

        Ok(())
    }

    fn set_first_time_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let latest: WeatherDataForDisplay = self.struct_weather_data_for_display(get_latest_data().unwrap());

        self.lcd.create_char(
            1,
            [
                0b00000100,
                0b00001010,
                0b00000100,
                0b00000000,
                0b00000000,
                0b00000000,
                0b00000000,
                0b00000000,
            ],
        )?;

        self.lcd.clear()?;
        self.lcd.print_at(0, 0, format!(
            "SCL {}  UTC {}",
            latest.time_local,
            latest.time_utc
        ))?;
        self.lcd.print_at(1, 0,format!(
            "In/Out:  {}/{} C",
            latest.internal,
            latest.external
        ))?;
        self.lcd.print_at(2, 0,format!(
            "T/Feel:  {}/{} C",
            latest.owm_temp,
            latest.owm_feels
        ))?;
        self.lcd.print_at(3, 0,format!("W: {}", latest.owm_condition))?;

        self.lcd.print_char_at(1, 18, 1)?;
        self.lcd.print_char_at(2, 18, 1)?;

        Ok(())
    }

    fn fmt_temp(&mut self, temp: f64) -> String {
        format!("{:.1}", temp)
    }

    fn fmt_date_to_time(&mut self, date: &str) -> String {
        let data = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S").unwrap();
        data.format("%H:%M").to_string()
    }

    fn struct_weather_data_for_display(&mut self, w: Weather) -> WeatherDataForDisplay {
        WeatherDataForDisplay {
            time_utc: self.fmt_date_to_time(&w.time_utc),
            time_local: self.fmt_date_to_time(&w.time_local),
            internal: self.fmt_temp(w.internal),
            external: self.fmt_temp(w.external),
            owm_temp: self.fmt_temp(w.owm_temp),
            owm_feels: self.fmt_temp(w.owm_feels),
            owm_condition: w.owm_condition,
        }
    }
}

/* 
    pub fn set_first_time_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let latest: Weather = get_latest_data().unwrap();

        self.lcd.create_char(
            1,
            [
                0b00001000,
                0b00010100,
                0b00001000,
                0b00000000,
                0b00000000,
                0b00000000,
                0b00000000,
                0b00000000,
            ],
        )?;

        self.lcd.clear()?;
        self.lcd.print(format!(
            "SCL {}  UTC {}",
            self.fmt_date_to_time(&latest.time_local),
            self.fmt_date_to_time(&latest.time_utc)
        ))?;
        self.lcd.print(format!(
            "In/Out:  {}/{} C",
            self.fmt_temp(&latest.internal),
            self.fmt_temp(&latest.external)
        ))?;
        self.lcd.print(format!(
            "T/Feel:  {}/{} C",
            self.fmt_temp(&latest.owm_temp),
            self.fmt_temp(&latest.owm_feels)
        ))?;
        self.lcd.print(format!("W: {}", latest.owm_condition))?;

        self.lcd.print_char_at(1, 18, 1)?;
        self.lcd.print_char_at(2, 18, 1)?;

        Ok(())
    }

    fn fmt_temp(&mut self, temp: &f64) -> String {
        format!("{:.1}", temp)
    }

    fn fmt_date_to_time(&mut self, date: &str) -> String {
        let data = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S").unwrap();
        data.format("%H:%M").to_string()
    }

*/