#[macro_use]
extern crate clap;

mod img;
mod batch;

use clap::{App, Arg};
use img::Img;
use batch::Batch;
use batch::builder::BatchBuilder;

fn main() {
    let matches = App::new("batchtool-rs")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(Arg::with_name("SOURCES")
             .help("Input files")
             .min_values(2)
             .required(true))
        .arg(Arg::with_name("OUTPUT")
             .help("Output file name")
             .takes_value(true)
             .required(true)
             .short("o")
             .long("output"))
        .arg(Arg::with_name("verbose")
             .help("Show verbose output")
             .short("v")
             .long("verbose"))
        .arg(Arg::with_name("trim")
             .help("Trim images")
             .short("t")
             .long("trim"))
        .arg(Arg::with_name("bounds")
             .help("Draw bounding boxes")
             .short("b")
             .long("bounds"))
        .arg(Arg::with_name("pow")
             .help("Make output square and pow2")
             .short("p")
             .long("pow"))
        .get_matches();

    let files: Vec<_> = matches.values_of("SOURCES").unwrap().collect();
    let mut images: Vec<_> = Vec::new();
    for file in &files {
        images.push(Img::from_file(&file));
    }

    let mut batch = BatchBuilder::new(images)
        //TODO: strategy
        .trim_images(matches.is_present("trim"))
        .pow_output(matches.is_present("pow"))
        //TODO: format
        .finalize();

    let output_name = matches.value_of("OUTPUT").unwrap();
    batch.pack().save(output_name);
}
