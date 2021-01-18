#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use std::thread;

use temp::get_and_process_data;

mod db;
mod web;
mod temp;

fn main() {
    let web_handler = thread::spawn( || {
        let tick = schedule_recv::periodic_ms(60000);
        get_and_process_data();
        loop {
            tick.recv().unwrap();
            get_and_process_data();
        }
    });
    web::startup();
    web_handler.join().unwrap();
}
