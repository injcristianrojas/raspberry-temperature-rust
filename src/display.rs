use chrono::NaiveDateTime;
use pwr_hd44780::Hd44780;
use pwr_hd44780::frontends::Direct;

use crate::db::get_current_data;

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
    fn set_first_time_data(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn update_data(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn fmt_temp(&mut self, temp: f64) -> String;
    fn fmt_date_to_time(&mut self, date: &str) -> String;
    fn struct_weather_data_for_display (&mut self) -> WeatherDataForDisplay;
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

    fn set_first_time_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let latest: WeatherDataForDisplay = self.struct_weather_data_for_display();

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

        self.lcd.set_backlight(false)?;
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

    fn update_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let latest: WeatherDataForDisplay = self.struct_weather_data_for_display();

        self.lcd.print_at(0, 4, latest.time_local)?;
        self.lcd.print_at(0, 15, latest.time_utc)?;
        self.lcd.print_at(1, 9, latest.internal)?;
        self.lcd.print_at(1, 14, latest.external)?;
        self.lcd.print_at(2, 9, latest.owm_temp)?;
        self.lcd.print_at(2, 14, latest.owm_feels)?;
        self.lcd.print_at(3, 0,format!("W: {}", latest.owm_condition))?;

        Ok(())
    }

    fn fmt_temp(&mut self, temp: f64) -> String {
        if temp > -100.0 {
            if temp.abs() < 10.0 {
                format!(" {:.1}", temp)
            } else {
                format!("{:.1}", temp)
            }
        } else {
            "----".to_string()
        }
    }

    fn fmt_date_to_time(&mut self, date: &str) -> String {
        let data = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S").unwrap();
        data.format("%H:%M").to_string()
    }

    fn struct_weather_data_for_display(&mut self) -> WeatherDataForDisplay {
        let w = get_current_data().unwrap();
        WeatherDataForDisplay {
            time_utc: self.fmt_date_to_time(&w.time_utc),
            time_local: self.fmt_date_to_time(&w.time_local),
            internal: self.fmt_temp(w.internal),
            external: self.fmt_temp(w.external),
            owm_temp: self.fmt_temp(w.owm_temp),
            owm_feels: self.fmt_temp(w.owm_feels),
            owm_condition: format!("{:width$}", w.owm_condition, width=17)
        }
    }
}
