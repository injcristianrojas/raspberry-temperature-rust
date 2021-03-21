#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use std::thread;

use display::Display;

mod db;
mod web;
mod temp;
mod display;

fn main() {
    let handler = thread::spawn( || {
        let tick = schedule_recv::periodic_ms(60000);
        temp::get_and_process_data();
        let mut display: Display = Display::new();
        display.set_first_time_data().unwrap();
        loop {
            tick.recv().unwrap();
            temp::get_and_process_data();
        }
    });
    web::startup();
    handler.join().unwrap();
}
