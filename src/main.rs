#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

use chrono::NaiveDateTime;
use rocket::response::Responder;
use rocket::{
    http::{ContentType, Status},
    response, Request, Response,
};
use rocket_contrib::{json, serve::StaticFiles};
use rocket_contrib::{json::JsonValue, templates::Template};

mod db;
use db::get_latest_data;

#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("index", &context)
}

#[derive(Serialize, Debug)]
pub struct Weather {
    pub internal: f64,
    pub external: f64,
    pub owm_temp: f64,
    pub owm_feels: f64,
    pub owm_condition: String,
    pub latest_formatted: String,
}

#[derive(Debug)]
struct ApiResponse {
    json: JsonValue,
    status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

fn process_date(mut weather_data: Weather) -> Weather {
    let timedata= NaiveDateTime::parse_from_str(&weather_data.latest_formatted, "%Y-%m-%d %H:%M:%S");
    weather_data.latest_formatted = match timedata {
        Ok(e) => e.format("%A, %B %d, %R").to_string(),
        Err(_) => "caca".to_string()
    };
    weather_data
}

#[get("/api/v1/latest")]
fn latest() -> ApiResponse {
    let latest = get_latest_data();
    match latest {
        Ok(latest) => ApiResponse {
            json: json!(process_date(latest)),
            status: Status::Ok,
        },
        Err(_) => ApiResponse {
            json: json!({"error_code": 500}),
            status: Status::InternalServerError,
        },
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, latest])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
