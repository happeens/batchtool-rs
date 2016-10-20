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
        let img = image::open(&Path::new(path)).unwrap();
        let dim = img.dimensions();

        Img {
            name: String::from(path),
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

    pub fn save(&self, path: &str) {
        let mut fout = File::create(&Path::new(path)).unwrap();
        let _ = self.data.save(&mut fout, image::PNG);
    }
}
