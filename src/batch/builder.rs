use img::Img;
use batch::{Strategy, Format, Batch}

pub struct BatchBuilder {
    images: Vec<Img>,
    strategy:
}

impl BatchBuilder {
    pub fn new(images: Vec<Img>) -> BatchBuilder {
        BatchBuilder {
            images: images
        }
    }
}
