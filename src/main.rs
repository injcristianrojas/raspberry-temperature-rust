#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;

use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("index", &context)
}

#[derive(Serialize)]
pub struct Weather {
    pub internal: f64,
    pub external: f64,
    pub owm_temp: f64,
    pub owm_feels: f64,
    pub owm_condition: String,
    pub latest_formatted: String,
}

#[get("/api/v1/latest")]
fn latest() -> Json<Weather> {
    Json(
        Weather{
            internal: 27.1,
            external: 29.3,
            owm_temp: 23.1,
            owm_feels: 24.0,
            owm_condition: "Clear sky".to_string(),
            latest_formatted: "Saturday, December 19, 19:25".to_string(),
        }
    )
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index, latest])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}