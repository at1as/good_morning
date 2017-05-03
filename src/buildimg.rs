extern crate image;
extern crate imageproc;
extern crate rusttype;

use std::path::Path;
use imageproc::drawing::draw_text_mut;
use image::{Rgb, RgbImage};
use rusttype::Scale;


pub fn text_to_image(text_blocks: Vec<String>, filepath: String) {

  let path = Path::new(&filepath);
  let mut image = RgbImage::new(400, 600);
  let font = include_bytes!("assets/DejaVuSans.ttf") as &[u8];

  let height = 10.0;
  let scale = Scale { x: height * 3.0, y: height * 3.0};

  let mut vertical_offset = 20;
  let horizontal_offset = 20;

  for i in 0..text_blocks.len() {
    for ch in text_blocks[i].chars().collect::<Vec<_>>().chunks(25) {
      let s: String = ch.into_iter().collect();
      draw_text_mut(&mut image, Rgb([255u8, 255u8, 255u8]), horizontal_offset, vertical_offset, scale, font, &s.trim());
      vertical_offset += 45;
    }

    vertical_offset += 20;
  }

  let _ = image.save(path).unwrap();
}

