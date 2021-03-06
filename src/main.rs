#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rand::{thread_rng, Rng};
use rocket::http::Method;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize)]
struct QuoteJsonOutput {
    quote: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct QuoteApi {
    quotes: Vec<String>,
}

const QUOTE_URL: &str =
    "https://raw.githubusercontent.com/anontyro/alan-partridge-interface/quotes/data/quotes.json";

#[get("/")]
fn index() -> Json<QuoteJsonOutput> {
    let quote_list: Vec<String> = get_quotes().unwrap();
    let quote_len = quote_list.len();
    let number = thread_rng().gen_range(0, quote_len);
    let output = format!("{}", quote_list[number]);

    let test = QuoteJsonOutput {
        quote: String::from(output),
    };

    Json(test)
}

fn get_quotes() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let response: QuoteApi = reqwest::get(QUOTE_URL)?.json()?;
    let quotes = response.quotes;

    Ok(quotes)
}

fn main() -> Result<(), Error> {
    let allowed_origins = AllowedOrigins::All;
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .mount("/", routes![index])
        .attach(cors)
        .launch();

    Ok(())
}
