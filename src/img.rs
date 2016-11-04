extern crate image;

use std::path::Path;
use std::fs::File;
use self::image::GenericImage;

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
    bounds: Rect,
    pos: Pos,
    offset: Pos,
    data: image::DynamicImage
}

impl Img {
    pub fn from_file(path: &str) -> Img {
        let path = Path::new(path);
        let img = image::open(&path).unwrap();
        let dim = img.dimensions();
        let filename = String::from(path.file_stem().unwrap().to_str().unwrap());

        // DEBUG
        println!("name found: {}", filename);

        Img {
            name: filename,
            size: Size {width: dim.0, height: dim.1},
            o_size: Size {width: dim.0, height: dim.1},
            bounds: Rect {
                pos: Pos {x: 0u32, y: 0u32},
                size: Size {width: 0u32, height: 0u32}
            },
            pos: Pos {x: 0u32, y: 0u32},
            offset: Pos {x: 0u32, y: 0u32},
            data: img
        }
    }

    pub fn trim(&mut self) {
        println!("pixel at 0, 0: {:?}", self.data.get_pixel(20u32, 20u32));

        // find empty rows at top
        let mut top_rows = 0u32;
        let mut empty = true;
        let mut y = 0;
        while empty && y < self.size.height {
            for x in 0..self.size.width {
                if self.data.get_pixel(x, y).data[3] != 0 {
                    empty = false;
                    top_rows = y - 1;
                    println!("visible pixel found at {}, {}", x, y);
                    break;
                }
            }

            y += 1;
        }
    }

    pub fn save(&self, path: &str) {
        let mut fout = File::create(&Path::new(path)).unwrap();
        let _ = self.data.save(&mut fout, image::PNG);
    }
}
