#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::{Json};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize)]
struct Weather {
    inside_temp: f32,
    outside_temp: f32
}

#[get("/api/v1/latest", format="json")]
fn latest() -> Json<Weather> {
    Json(
        Weather{
            inside_temp: 27.1,
            outside_temp: 29.3
        }
    )
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index, latest])
        .launch();
}