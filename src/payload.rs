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

pub trait IntoVec<P> {
    type Error;

    fn as_vec(&self) -> Result<Vec<P>, Self::Error>;
}

impl<P, Container> IntoVec<u8> for ImageBuffer<P, Container>
    where P: Pixel<Subpixel=u8> + 'static,
          P::Subpixel: 'static,
          Container: Deref<Target=[P::Subpixel]> {
    type Error = Status;

    fn as_vec(&self) -> Result<Vec<u8>, Status> {
        let mut v = Vec::new();
        let (w, h) = self.dimensions();
        let bytes = &**self;
        PngEncoder::new(&mut v)
            .encode(bytes, w, h, ColorType::Rgba8)
            .map_err(|_| Status::InternalServerError)?;
        Ok(v)
    }
}