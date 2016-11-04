#[macro_use]
extern crate clap;

mod img;

use clap::{App, Arg};
use img::Img;

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
        if matches.is_present("trim") {
            println!("trimming");
        }
    }

    for image in &mut images {
        image.trim();
    }

    let output_name = matches.value_of("OUTPUT").unwrap();
    images[0].save("test.png");
    println!("output: {}", output_name);

    println!("done.");
}
