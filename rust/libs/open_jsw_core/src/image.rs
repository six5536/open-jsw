use macroquad::{color::Color, texture::Image};
use std::cmp::max;

use crate::{Error, Result};

pub const TRANSPARENT: Color = Color::new(0.0, 0.0, 0.0, 0.0);

pub fn create_image_from_sprite_data(
    data: &[u8],
    width: usize,
    height: usize,
    fg: Color,
    bg: Color,
) -> Result<Image> {
    let mut image = Image::gen_image_color(width as u16, height as u16, TRANSPARENT);
    let mut colors: Vec<Color> = vec![TRANSPARENT; width * height];
    for (i, row) in data.iter().enumerate() {
        for col in 0..width {
            let bit = (row >> (width - col - 1)) & 1;
            let mut color = bg;
            if bit == 1 {
                color = fg;
            }

            // Update the color at the index
            // (may fail if the index is out of bounds if the input data splice is too small)
            if let Some(c) = colors.get_mut(i * 8 + col) {
                *c = color;
            } else {
                Err(Error::IndexOutOfBounds {
                    index: i * 8 + col,
                    length: colors.len(),
                })?;
            }
        }
    }
    image.update(&colors);

    Ok(image)
}

pub fn create_spritesheet(images: Vec<&Image>) -> Image {
    let count = images.len() as u32;
    let max_width = images.iter().map(|img| img.width as u32).max().unwrap_or(1);
    let max_height = images
        .iter()
        .map(|img| img.height as u32)
        .max()
        .unwrap_or(1);

    let mut grid_size = 1;
    while grid_size * grid_size < count {
        grid_size *= 2;
    }

    let raw_width = grid_size * max_width;
    let raw_height = grid_size * max_height;

    // Make sure the sheet is square and size is a power of two
    let side = max(raw_width, raw_height).next_power_of_two();

    let mut sheet_bytes = vec![0u8; (side * side * 4) as usize];

    for (i, image) in images.iter().enumerate() {
        let col = (i as u32) % grid_size;
        let row = (i as u32) / grid_size;

        let x_offset = col * max_width;
        let y_offset = row * max_height;

        let image_data = image.get_image_data();

        for y in 0..image.height as u32 {
            for x in 0..image.width as u32 {
                let dst_index = (((y_offset + y) * side + (x_offset + x)) * 4) as usize;
                let src_index = (y * image.width as u32 + x) as usize;
                sheet_bytes[dst_index..dst_index + 4].copy_from_slice(&image_data[src_index]);
            }
        }
    }

    Image {
        bytes: sheet_bytes,
        width: side as u16,
        height: side as u16,
    }
}
