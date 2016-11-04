extern crate image;
extern crate std;

use std::path::Path;
use std::fs::File;
use self::image::{RgbaImage, imageops, DynamicImage, Rgba};

struct Pos {
    x: u32,
    y: u32
}

struct Size {
    width: u32,
    height: u32
}

struct Rect {
    pos: Pos,
    size: Size
}

pub struct Img {
    name: String,
    size: Size,
    o_size: Size,
    pos: Pos,
    offset: Pos,
    pixels: RgbaImage
}

impl Img {
    pub fn from_file(path: &str) -> Img {
        let path = Path::new(path);
        let img: RgbaImage = image::open(&path).unwrap().to_rgba();
        let dim = img.dimensions();
        let name = path.file_stem().unwrap().to_str().unwrap();

        Img {
            name: String::from(name),
            size: Size {width: dim.0, height: dim.1},
            o_size: Size {width: dim.0, height: dim.1},
            pos: Pos {x: 0u32, y: 0u32},
            offset: Pos {x: 0u32, y: 0u32},
            pixels: img
        }
    }

    pub fn save(&self, path: &str) {
        let path = Path::new(path);
        let _ = self.pixels.save(&path);
    }

    pub fn draw_bounds(&mut self) {
        let pixel = Rgba {data: [0u8, 0u8, 0u8, 255u8]};
        for x in 0..(self.size.width) {
            self.pixels.put_pixel(x, 0u32, pixel.clone());
            self.pixels.put_pixel(x, self.size.height - 1, pixel.clone());
        }

        for y in 1..(self.size.height - 1) {
            self.pixels.put_pixel(0u32, y, pixel.clone());
            self.pixels.put_pixel(self.size.width - 1, y, pixel.clone());
        }
    }

    pub fn trim(&mut self) {
        // find white space at top
        let mut top_rows = 0u32;
        let mut left_rows = 0u32;
        let mut right_rows = 0u32;
        let mut bottom_rows = 0u32;
        let mut empty = true;

        // top down
        while empty && (top_rows < self.size.height) {
            for x in 0..(self.size.width) {
                if self.pixels.get_pixel(x, top_rows).data[3] != 0 {
                    empty = false;
                    break;
                }
            }

            top_rows += 1u32;
        }

        empty = true;
        // left in
        while empty && (left_rows < self.size.width) {
            for y in 0..(self.size.height) {
                if self.pixels.get_pixel(left_rows, y).data[3] != 0 {
                    empty = false;
                    break;
                }
            }

            left_rows += 1u32;
        }

        empty = true;
        // bottom up
        while empty && (bottom_rows < self.size.height) {
            for x in 0..(self.size.width) {
                if self.pixels.get_pixel(x, self.size.height - bottom_rows - 1).data[3] != 0 {
                    empty = false;
                    break;
                }
            }

            bottom_rows += 1u32;
        }

        empty = true;
        // left in
        while empty && (right_rows < self.size.width) {
            for y in 0..(self.size.height) {
                if self.pixels.get_pixel(self.size.width - right_rows - 1, y).data[3] != 0 {
                    empty = false;
                    break;
                }
            }

            right_rows += 1u32;
        }

        top_rows -= 1u32;
        left_rows -= 1u32;
        right_rows -= 1u32;
        bottom_rows -= 1u32;

        self.size.height -= top_rows;
        self.offset.y = top_rows;

        self.size.width -= left_rows;
        self.offset.x = left_rows;

        self.size.width -= right_rows;
        self.size.height -= bottom_rows;

        // crop the image
        let mut pixels = self.pixels.clone();
        let subimage = imageops::crop(&mut pixels, self.offset.x, self.offset.y, self.size.width, self.size.height);
        let mut cropped = DynamicImage::new_rgba8(self.size.width, self.size.height).to_rgba();
        cropped.clone_from(&subimage.to_image());
        self.pixels = cropped;
    }
}
