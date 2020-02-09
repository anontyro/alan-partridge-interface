#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket_contrib::json::Json;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize)]
struct QuoteJsonOutput {
    quote: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/quote")]
fn quote() -> Json<QuoteJsonOutput> {
    let test = QuoteJsonOutput {
        quote: String::from("test quote out"),
    };

    Json(test)
}

fn main() {
    rocket::ignite().mount("/", routes![index, quote]).launch();
}
