use rocket::Data;
use rocket::http::Status;
use image::{DynamicImage, ImageOutputFormat};
use image::ImageFormat;
use image::io::Reader;
use std::io::{Cursor, Read};

const MAX_SIZE: u64 = 64 * 1_000_000; // 64 MB

pub fn image_from_data(image: Data) -> Result<(DynamicImage, ImageFormat), Status> {
    let mut data: Vec<u8> = Vec::new();
    let read = image.open()
        .take(MAX_SIZE)
        .read_to_end(&mut data)
        .map_err(|_| Status::InternalServerError)?;

    if read == MAX_SIZE as usize {
        return Err(Status::PayloadTooLarge);
    }

    let reader = Reader::new(Cursor::new(data)).with_guessed_format().expect("Cursor io never fails");

    let format = match reader.format() {
        Some(format) => format,
        None => return Err(Status::UnsupportedMediaType),
    };

    Ok((reader.decode().map_err(|_| Status::InternalServerError)?, format))
}

pub fn image_to_vec(image: &DynamicImage, format: ImageOutputFormat) -> Result<Vec<u8>, Status> {
    let mut v: Vec<u8> = Vec::new();
    image.write_to(&mut v, format).map_err(|_| Status::InternalServerError)?;
    Ok(v)
}