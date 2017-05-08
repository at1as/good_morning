extern crate image;
extern crate imageproc;
extern crate rusttype;

use std::collections::HashMap;
use std::path::Path;
use imageproc::drawing::{draw_text_mut, draw_line_segment_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;
use image::{Rgb, RgbImage};
use rusttype::Scale;


pub fn text_to_image(text_blocks: Vec<String>, filepath: String) {

  /* Colors */
  let mut colors = HashMap::new();
  colors.insert("white", Rgb([255u8, 255u8, 255u8]));
  colors.insert("black", Rgb([0u8, 0u8, 0u8]));
  colors.insert("light_blue", Rgb([34u8, 132u8, 201u8]));
  
  /* Image - Blue background with 2px black border */
  let path = Path::new(&filepath);
  let mut image = RgbImage::new(400, 600);
  draw_filled_rect_mut(&mut image, Rect::at(2, 2).of_size(396, 596), colors["light_blue"]);

  /* Fonts */
  let font = include_bytes!("assets/DejaVuSans.ttf") as &[u8];
  let height = 10.0;
  let scale = Scale { x: height * 3.0, y: height * 3.0};

  /* Image pixel offsets */
  let mut vertical_offset = 20;
  let horizontal_offset = 20;

  /* Write to Image */
  for i in (0..text_blocks.len()) {
    
    if text_blocks[i] == "prepend" { continue; } // Don't write heading for this message
    let words = text_blocks[i].split(" ");

    let mut j = 0;
    let mut line_contents: Vec<String> = Vec::new();
    let max_chars_per_line = 25;

    /* Split string into blocks of words, no more than 25 characters per line */
    for word in words {
      if line_contents.len() >= j && j != 0 {

        let current_str  = line_contents[j-1].clone();
        let appended_str = format!("{} {}", current_str.clone(), word);

        if appended_str.len() <= max_chars_per_line {
          line_contents[j-1] = appended_str.clone();
        } else {
          line_contents.push(word.to_string());
          j += 1;
        }

      } else {
        line_contents.push(word.to_string());
        if j == 0 { j += 1 }
      }
    }

    /* Write the lines to the image */
    for txt in line_contents {

      if i%2 == 0 { /* HEADING */
        draw_text_mut(&mut image, colors["black"], horizontal_offset, vertical_offset, scale, font, &txt.trim());
      } else {      /* TEXT BODY */
        draw_text_mut(&mut image, colors["white"], horizontal_offset, vertical_offset, scale, font, &txt.trim());
      }

      vertical_offset += 45;
    }

    /* Add padding after each section. Add a line seperator after headings */
    if i%2 == 0 {
      draw_line_segment_mut(&mut image,
                            (80 as f32,  vertical_offset as f32),
                            (330 as f32, vertical_offset as f32),
                            colors["white"]);
      vertical_offset += 20;
    } else {
      vertical_offset += 20;
    }
  }

  let _ = image.save(path).unwrap();
}

