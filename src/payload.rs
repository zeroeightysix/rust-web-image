use rocket::Data;
use rocket::http::Status;
use image::{DynamicImage, ImageBuffer, Pixel, ColorType, ImageOutputFormat};
use image::ImageFormat;
use image::io::Reader;
use std::io::{Cursor, Read};
use image::codecs::png::PngEncoder;
use std::ops::Deref;

const MAX_SIZE: u64 = 64 * 1_000_000; // 64 MB

pub fn safe_vector_from_data(image: Data) -> Result<Vec<u8>, Status> {
    let mut data: Vec<u8> = Vec::new();
    let read = image.open()
        .take(MAX_SIZE)
        .read_to_end(&mut data)
        .map_err(|_| {
            log::error!("Error while reading the data, possibly reached MAX_SIZE ({})", MAX_SIZE);
            Status::InternalServerError
        })?;

    if read == MAX_SIZE as usize {
        return Err(Status::PayloadTooLarge);
    }

    return Ok(data);
}

pub fn image_reader_from_data(data: &Vec<u8>) -> Result<Reader<Cursor<&Vec<u8>>>, Status> {
    let reader = Reader::new(Cursor::new(data))
        .with_guessed_format()
        .expect("Cursor io never fails");

    return Ok(reader);
}


#[inline(always)]
pub fn image_format_from_reader(reader: &Reader<Cursor<&Vec<u8>>>) -> Result<ImageFormat, Status> {
    return match reader.format() {
        None => Err(Status::UnsupportedMediaType),
        Some(format) => Ok(format),
    };
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

    fn as_vec(&self) -> Result<Vec<u8>, Self::Error> {
        let mut v = Vec::new();
        let (w, h) = self.dimensions();
        let bytes = &**self;
        PngEncoder::new(&mut v)
            .encode(bytes, w, h, ColorType::Rgba8)
            .map_err(|_| Status::InternalServerError)?;
        Ok(v)
    }
}

impl IntoVec<u8> for DynamicImage {
    type Error = Status;

    fn as_vec(&self) -> Result<Vec<u8>, Self::Error> {
        let mut v = Vec::new();
        self.write_to(&mut v, ImageOutputFormat::Png).map_err(|_| Status::InternalServerError)?;
        Ok(v)
    }
}