#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

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


// environment is configured in Rocket.toml and can be chosen with the ROCKET_ENV env variable. see https://rocket.rs/v0.4/guide/configuration/
fn main() {
    rocket::ignite()
        .mount("/", routes![blur])
        .launch();
}
