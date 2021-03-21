use chrono::NaiveDateTime;
use pwr_hd44780::Hd44780;
use pwr_hd44780::frontends::Direct;

use crate::db::{get_latest_data, Weather};

pub struct Display {
    pub lcd: Direct
}

impl Display {
    
    pub fn new() -> Display {
        let lcd_bus = pwr_hd44780::I2CBus::new("/dev/i2c-1", 0x27).unwrap();
        let lcd = pwr_hd44780::DirectLcd::new(Box::new(lcd_bus), 20, 4).unwrap();
        Display {
            lcd
        }
    }

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
}
