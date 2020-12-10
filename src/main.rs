#![feature(proc_macro_hygiene, decl_macro)]

mod payload;

#[macro_use]
extern crate rocket;

use rocket::Data;
use rocket::http::{Status, ContentType};
use rocket::response::Content;
use image::RgbaImage;
use crate::payload::IntoVec;

macro_rules! deny_out_of_range {
    ($expression:ident, $range:expr) => {
        if !$range.contains(&$expression) {
            return Err(Status::Forbidden)
        }
    };
}

#[post("/blur?<sigma>", data = "<image>")]
fn blur(image: Data, sigma: f32) -> Result<Content<Vec<u8>>, Status> {
    let (image, _) = payload::image_from_data(image)?;
    deny_out_of_range!(sigma, 0.0 .. 10.0);

    let image: RgbaImage = imageproc::filter::gaussian_blur_f32(&image.into_rgba8(), sigma);
    Ok(Content(ContentType::PNG, image.as_vec()?))
}

#[post("/unsharpen?<sigma>&<threshold>", data = "<image>")]
fn unsharpen(image: Data, sigma: f32, threshold: i32) -> Result<Content<Vec<u8>>, Status> {
    let (image, _) = payload::image_from_data(image)?;
    deny_out_of_range!(sigma, 0.0 .. 10.0);

    let image = image.unsharpen(sigma, threshold);
    Ok(Content(ContentType::PNG, image.as_vec()?))
}

fn main() {
    rocket::ignite().mount("/", routes![blur, unsharpen]).launch();
}
