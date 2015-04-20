extern crate lodepng;

use std::env;
use std::path::Path;
use std::convert::AsMut;
use lodepng::RGB;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = format!("Usage: {} <png-file>", args[0]);
    if args.len() < 2 {
        println!("Error: incorrect number of arguments provided.\n{}", usage);
        return;
    }

    let img_path = &Path::new(&args[1]);

    match lodepng::decode24_file(img_path) {
        Ok(ref mut bitmap) => {
            println!("Decoded {} x {} image at {}", bitmap.width, bitmap.height,
                img_path.to_str().expect("path should be valid"));

            let mapped = bitmap.buffer.as_mut().iter().collect::<Vec<&RGB<u8>>>();

            println!("Image data: {:?}", mapped);
        },
        Err(reason) => println!("Could not load {}, because: {}", img_path.display(), reason),
    }

}
