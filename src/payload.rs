use rocket::Data;
use rocket::http::Status;
use image::{DynamicImage, ImageBuffer, Pixel, ColorType};
use image::ImageFormat;
use image::io::Reader;
use std::io::{Cursor, Read};
use image::codecs::png::PngEncoder;
use std::ops::Deref;

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

// do not touch
pub fn image_to_vec<P, Container>(image: &ImageBuffer<P, Container>) -> Result<Vec<u8>, Status>
    where P: Pixel<Subpixel=u8> + 'static,
          P::Subpixel: 'static,
          Container: Deref<Target=[P::Subpixel]> {
    let mut v = Vec::new();
    let (w, h) = image.dimensions();
    let bytes = &**image;
    PngEncoder::new(&mut v)
        .encode(bytes, w, h, ColorType::Rgba8)
        .map_err(|_| Status::InternalServerError)?;
    Ok(v)
}