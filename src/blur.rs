use image::{ImageFormat, RgbaImage};
use rocket::response::Content;
use image::io::Reader;
use std::io::Cursor;
use rocket::http::ContentType;
use gif::{Encoder, Frame, Repeat, DisposalMethod};
use crate::payload::IntoVec;

pub fn blur_base_on_type(data: &Vec<u8>, image_reader: Reader<Cursor<&Vec<u8>>>, format: ImageFormat, sigma: f32) -> Content<Vec<u8>> {
    return match format {
        ImageFormat::Gif => {
            let mut decoder = gif::DecodeOptions::new();
            decoder.set_color_output(gif::ColorOutput::RGBA);
            let mut decoder = decoder.read_info(Cursor::new(data)).unwrap();

            let width = decoder.width();
            let height = decoder.height();

            let mut frames: Vec<Vec<u8>> = vec!();

            while let Some(frame) = decoder.read_next_frame().unwrap() {
                let frame: &Frame = frame;

                let mut vec = vec!();
                for s in frame.buffer.iter() {
                    vec.push(*s);
                }
                frames.push(vec)
            }
            let buffer_size = width as usize * height as usize * frames.len() * 2;
            let mut vec1 = vec![0; buffer_size];
            {
                let mut encoder = Encoder::new(vec1.as_mut_slice(), width, height, &[]).unwrap();
                encoder.set_repeat(Repeat::Infinite).expect("oh no, couldn't set gif to repeat");

                for frame in frames {
                    let img = RgbaImage::from_raw(width as u32, height as u32, frame).unwrap();
                    let mut image: RgbaImage = imageproc::filter::gaussian_blur_f32(&img, sigma);
                    let mut new_frame = Frame::from_rgba_speed(width, height, image.as_mut(), 10);
                    new_frame.dispose = DisposalMethod::Background;
                    encoder.write_frame(&new_frame).expect("oh no, couldn't write frame");
                }
            }

            Content(ContentType::GIF, vec1)
        }
        _ => {
            let image: RgbaImage = imageproc::filter::gaussian_blur_f32(&(image_reader.decode().unwrap()).as_mut_rgba8().expect("flushed"), sigma);
            Content(ContentType::PNG, image.as_vec().expect("no vec today said vector"))
        }
    };
}
