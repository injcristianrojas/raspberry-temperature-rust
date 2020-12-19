#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;

use rocket_contrib::json::{Json};
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn index() -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    context.insert("name", "you dumb fuck!");
    Template::render("index", &context)
}

#[derive(Serialize)]
struct Weather {
    inside: f32,
    outside: f32,
    latest_formatted: &'static str
}

#[get("/api/v1/latest", format="json")]
fn latest() -> Json<Weather> {
    Json(
        Weather{
            inside: 27.1,
            outside: 29.3,
            latest_formatted: "Saturday, December 19, 19:25"
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