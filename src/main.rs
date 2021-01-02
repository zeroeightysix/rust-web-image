#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::env;

use rocket::config::{Config, Environment};
use rocket::Data;
use rocket::http::Status;
use rocket::response::Content;

mod payload;
mod blur;

macro_rules! deny_out_of_range {
    ($expression:ident, $range:expr) => {
        if !$range.contains(&$expression) {
            return Err(Status::BadRequest)
        }
    };
}

macro_rules! deny_smaller_than {
    ($expression:ident, $smallest:expr) => {
        if $smallest > $expression {
            return Err(Status::BadRequest)
        }
    };
}

macro_rules! deny_bigger_than {
    ($expression:ident, $biggest:expr) => {
        if $biggest < $expression {
            return Err(Status::BadRequest)
        }
    };
}

#[post("/blur?<sigma>&<repeat>&<delay>", data = "<data>")]
fn blur(data: Data, sigma: f32, repeat: i16, delay: u16) -> Result<Content<Vec<u8>>, Status> {
    let image = payload::safe_vector_from_data(data)?;
    let reader = payload::image_reader_from_data(&image)?;
    let format = payload::image_format_from_reader(&reader)?;
    deny_out_of_range!(sigma, 0.0 .. 10.0);
    deny_smaller_than!(repeat, -1);
    deny_bigger_than!(delay, 2000);

    let content = blur::blur_base_on_type(&image, reader, format, sigma, repeat, delay);
    Ok(content)
}


// First argument supplied to the executable will be parsed and used as environment
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut env_arg = &String::from("development");
    if args.len() > 1 {
        env_arg = &args[1];
    }

    let env: Environment;
    if env_arg == "production" {
        env = Environment::Production;
    } else if env_arg == "staging" {
        env = Environment::Staging;
    } else {
        env = Environment::Development;
    };

    let config = Config::build(env)
        .address("0.0.0.0")
        .port(8000)
        .finalize().unwrap();

    rocket::custom(config)
        .mount("/", routes![blur])
        .launch();
}
