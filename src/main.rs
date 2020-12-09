#![feature(proc_macro_hygiene, decl_macro)]

mod payload;

#[macro_use]
extern crate rocket;

use rocket::Data;
use rocket::http::{Status, ContentType};
use rocket::response::Content;
use image::ImageOutputFormat;

#[post("/blur", data = "<image>")]
fn blur(image: Data) -> Result<Content<Vec<u8>>, Status> {
    let (image, _) = payload::image_from_data(image)?;

    let image = image.blur(5.0);

    Ok(Content(ContentType::PNG, payload::image_to_vec(&image, ImageOutputFormat::Png)?))
}

fn main() {
    rocket::ignite().mount("/", routes![blur]).launch();
}
