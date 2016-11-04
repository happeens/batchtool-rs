use img::Img;

pub enum Strategy {
    DEFAULT,
}

pub enum Format {
    PNG,
}

pub struct Batch {
    images: Vec<Img>,
    strategy: Strategy,
    trim: bool,
    pow: bool,
    name: String,
    format: Format
}

impl Batch {
    pub fn from_vec(images: Vec<Img>) -> Batch {
        Batch {
            images: images
        }
    }

    pub fn trim(&mut self) {
        println!("trimming");
        for image in &mut self.images {
            image.trim();
        }
    }

    pub fn save(&self, name: &str) {
        println!("saving as {}", name);
        self.images[0].save(name);
    }
}
