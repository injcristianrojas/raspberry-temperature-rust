#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use std::thread;
use std::time::Duration;

mod db;
mod web;

fn main() {
    let web_handler = thread::spawn( || {
        let mut i = 0;
        loop {
            // Data grabber goes here then
            println!("hi number {} from the spawned thread!", { i += 1; i });
            thread::sleep(Duration::from_millis(100));
        }
    });
    web::startup();
    web_handler.join().unwrap();
}
