extern crate image;
extern crate imageproc;
extern crate rusttype;

use std::path::Path;
use imageproc::drawing::draw_text_mut;
use image::{Rgb, RgbImage};
use rusttype::Scale;


pub fn text_to_image(text: String, filepath: String) -> String {

  let path = Path::new(&filepath);
  let mut image = RgbImage::new(400, 600);
  let font = include_bytes!("DejaVuSans.ttf") as &[u8];

  let height = 10.0;
  let scale = Scale { x: height * 4.0, y: height * 3.0};

  let mut vertical_offset = 10;

  for ch in text.chars().collect::<Vec<_>>().chunks(25) {
    let s: String = ch.into_iter().collect();
    draw_text_mut(&mut image, Rgb([255u8, 255u8, 255u8]), 10, vertical_offset, scale, font, &s);
    vertical_offset += 40;
  }

  let _ = image.save(path).unwrap();
  "done".to_owned()
}
