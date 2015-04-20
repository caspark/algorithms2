extern crate lodepng;

use std::env;
use std::path::Path;
use std::convert::AsMut;
use std::convert::AsRef;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = format!("Usage: {} <path-to-input-png> <output-path>", args[0]);
    if args.len() < 3 {
        println!("Error: incorrect number of arguments provided.\n{}", usage);
        return;
    }

    let input_img_path = &Path::new(&args[1]);
    let output_img_path = &Path::new(&args[2]);

    let mut bitmap = match lodepng::decode24_file(input_img_path) {
        Ok(bitmap) => bitmap,
        Err(reason) => panic!("Could not load {}, because: {}", input_img_path.display(), reason),
    };

    println!("Decoded {} x {} image at {}", bitmap.width, bitmap.height,
        input_img_path.to_str().expect("path should be valid"));

    //TODO do some processing that's more relevant to the spec
    println!("Decreasing red by 75% and green by 50% (image should be more blue)");
    for pixel in bitmap.buffer.as_mut().iter_mut() {
        pixel.r = pixel.r / 4 % 255;
        pixel.g = pixel.g / 2 % 255;
    }

    if let Err(e) = lodepng::encode24_file(output_img_path, bitmap.buffer.as_ref(), bitmap.width, bitmap.height) {
        panic!("Failed to save png to {} because: {}", output_img_path.to_str().expect("path should be valid"), e);
    }

    println!("Success! Saved output image to {}", output_img_path.to_str().expect("path should be valid"));
}
