#![feature(proc_macro_hygiene, decl_macro)]

mod payload;

#[macro_use]
extern crate rocket;

use rocket::Data;
use rocket::http::{Status, ContentType};
use rocket::response::Content;
use image::RgbaImage;
use crate::payload::IntoVec;

#[post("/blur/<sigma>", data = "<image>")]
fn blur(image: Data, sigma: f32) -> Result<Content<Vec<u8>>, Status> {
    let (image, _) = payload::image_from_data(image)?;
    if sigma <= 0.0 || sigma > 10.0 {
        return Err(Status::Forbidden)
    }

    let image: RgbaImage = imageproc::filter::gaussian_blur_f32(&image.into_rgba8(), sigma);
    Ok(Content(ContentType::PNG, image.as_vec()?))
}

fn main() {
    rocket::ignite().mount("/", routes![blur]).launch();
}
