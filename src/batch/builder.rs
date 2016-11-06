use img::Img;
use batch::{Strategy, Format, Batch};

pub struct BatchBuilder {
    images: Vec<Img>,
    strategy: Strategy,
    trim: bool,
    pow: bool,
    bounds: bool,
    format: Format
}

impl BatchBuilder {
    pub fn new(images: Vec<Img>) -> BatchBuilder {
        BatchBuilder {
            images: images,
            strategy: Strategy::Default,
            trim: false,
            pow: false,
            bounds: false,
            format: Format::Png
        }
    }

    pub fn packing_strategy(mut self, strategy: Strategy) -> BatchBuilder {
        self.strategy = strategy;
        self
    }

    pub fn trim_images(mut self, trim: bool) -> BatchBuilder {
        self.trim = trim;
        self
    }

    pub fn pow_output(mut self, pow: bool) -> BatchBuilder {
        self.pow = pow;
        self
    }

    pub fn draw_bounds(mut self, bounds: bool) -> BatchBuilder {
        self.bounds = bounds;
        self
    }

    pub fn with_format(mut self, format: Format) -> BatchBuilder {
        self.format = format;
        self
    }

    pub fn finalize(self) -> Batch {
        Batch {
            images: self.images,
            strategy: self.strategy,
            trim: self.trim,
            pow: self.pow,
            bounds: self.bounds,
            format: self.format
        }
    }
}
