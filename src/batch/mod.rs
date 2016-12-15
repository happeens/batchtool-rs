use img::{Img, Size, Pos};
use std::cmp;
use std::vec::Vec;

pub mod builder;

pub enum Strategy {
    Default,
}

pub enum Format {
    Png,
}

pub struct Batch {
    images: Vec<Img>,
    strategy: Strategy,
    trim: bool,
    pow: bool,
    bounds: bool,
    format: Format
}

impl Batch {
    pub fn pack(&mut self) -> Img {
        for image in &mut self.images {
            if self.trim {
                image.trim();
            }

            if self.bounds {
                image.draw_bounds();
            }
        }

        // sort images by size
        self.images.sort_by(|a, b| b.cmp(a));

        let mut free_anchors = Vec::<Pos>::new();
        let mut abs_anchors = Vec::<Pos>::new();
        free_anchors.push(Pos {x: 0, y: 0});

        for image in &mut self.images {
            // set next image anchor to current 
            // smallest anchor point
            abs_anchors.push(free_anchors.first().unwrap().clone());

            // find new anchors
            let mut new_anchor_right = Pos {
                x: free_anchors.first().unwrap().x + image.size.width,
                y: free_anchors.first().unwrap().y
            };

            let mut new_anchor_bot = Pos {
                x: free_anchors.first().unwrap().x,
                y: free_anchors.first().unwrap().y + image.size.height
            };

            // still finding new anchors
            for i in 1..(free_anchors.len() - 1) {
                let first = free_anchors.first().unwrap().clone();

                // if we removed an anchor before, we might
                // go out of bounds
                if i >= free_anchors.len() {
                    break;
                }

                if free_anchors[i].x >= first.x &&
                   free_anchors[i].x <= new_anchor_right.x {
                       new_anchor_right.y = cmp::min(new_anchor_right.y,
                                                     free_anchors[i].y);
                       free_anchors.remove(i);
                       continue;
                }

                if free_anchors[i].y >= first.y &&
                   free_anchors[i].y <= new_anchor_bot.y {
                       new_anchor_bot.x = cmp::min(new_anchor_bot.x,
                                                   free_anchors[i].x);
                       free_anchors.remove(i);
                       continue;
                }

            }

            // remove first, add new anchors
            free_anchors.remove(0);
            if !free_anchors.contains(&new_anchor_right) {
                free_anchors.push(new_anchor_right);
            }
            if !free_anchors.contains(&new_anchor_bot) {
                free_anchors.push(new_anchor_bot);
            }

            free_anchors.sort_by(|a, b| a.cmp(b));
        }

        // find side length
        let mut side_length_x = 0;
        let mut side_length_y = 0;

        for anchor in free_anchors {
            side_length_x = cmp::max(side_length_x, anchor.x);
            side_length_y = cmp::max(side_length_y, anchor.y);
        }

        let mut result = Img::new("output", side_length_x, side_length_y);

        assert_eq!(abs_anchors.len(), self.images.len());

        for i in 0..self.images.len() {
            let anchor = abs_anchors[i].clone();
            result.insert(self.images[i].clone(), anchor);
        }

        result
    }
}
