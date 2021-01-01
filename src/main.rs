#![feature(proc_macro_hygiene, decl_macro)]

mod payload;
mod blur;

#[macro_use]
extern crate rocket;

use rocket::config::{Config, Environment};
use rocket::Data;
use rocket::http::{Status};
use rocket::response::Content;

macro_rules! deny_out_of_range {
    ($expression:ident, $range:expr) => {
        if !$range.contains(&$expression) {
            return Err(Status::BadRequest)
        }
    };
}

#[post("/blur?<sigma>", data = "<data>")]
fn blur(data: Data, sigma: f32) -> Result<Content<Vec<u8>>, Status> {
    let image = payload::safe_vector_from_data(data)?;
    let reader = payload::image_reader_from_data(&image)?;
    let format = payload::image_format_from_reader(&reader)?;
    deny_out_of_range!(sigma, 0.0 .. 10.0);

    let content = blur::blur_base_on_type(&image, reader, format, sigma);
    Ok(content)
}

fn main() {
    let config = Config::build(Environment::Production)
        .address("0.0.0.0")
        .port(8000)
        .finalize().unwrap();

    rocket::custom(config)
        .mount("/", routes![blur])
        .launch();
}
