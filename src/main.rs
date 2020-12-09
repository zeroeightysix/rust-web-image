#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::Data;
use std::io::Read;
use rocket::http::{Status, ContentType};
use image::io::Reader;
use std::io::Cursor;
use rocket::response::Content;

const MAX_SIZE: u64 = 64 * 1_000_000; // 64 MB

#[post("/blur", data="<image>")]
fn blur(image: Data<>) -> Result<Content<Vec<u8>>, Status> {
    let mut data: Vec<u8> = Vec::new();
    let read = image.open()
        .take(MAX_SIZE)
        .read_to_end(&mut data)
        .map_err(|_| Status::InternalServerError)?;

    if read == MAX_SIZE as usize {
        return Err(Status::PayloadTooLarge)
    }

    let reader = Reader::new(Cursor::new(data)).with_guessed_format().expect("Cursor io never fails");

    let format = match reader.format() {
        Some(format) => format,
        None => return Err(Status::UnsupportedMediaType),
    };

    let image = reader.decode().map_err(|_| Status::InternalServerError)?;
    let image = image.blur(5.0);

    let mut out: Vec<u8> = Vec::new();
    let _ = image.write_to(&mut out, image::ImageOutputFormat::Png);

    println!("read {} bytes, blurred format {:?}, and sent {} bytes", read, format, out.len());

    Ok(Content(ContentType::PNG, out))
}

fn main() {
    rocket::ignite().mount("/", routes![blur]).launch();
}
