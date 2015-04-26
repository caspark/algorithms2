extern crate getopts;
extern crate lodepng;

use getopts::Options;
use std::env;
use std::path::Path;
use std::convert::AsMut;
use std::convert::AsRef;
use std::process;

mod carving;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} INPUT-FILE [options]", program);
        print!("{}", opts.usage(&brief));
    }

    let mut opts = Options::new();
    opts.optopt("o", "output", "path to output the resulting image", "OUTPUT-FILE");
    opts.optflag("e", "energy", "convert the given image to a display of its energy");
    opts.optflag("p", "preview", "outline the pixels that would be removed in bright red, but don't actually remove them");
    opts.optflag("v", "verbose", "print out energy and discovered seams");
    opts.optflag("h", "help", "print this usage information");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("Invalid arguments: {}", f.to_string());
            print_usage(program, opts);
            process::exit(1);
        },
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let input_img_path = if !matches.free.is_empty() {
        Path::new(&matches.free[0])
    } else {
        print_usage(&program, opts);
        process::exit(1);
    };
    let verbose = matches.opt_present("v");

    let mut bitmap = match lodepng::decode24_file(input_img_path) {
        Ok(bitmap) => bitmap,
        Err(reason) => panic!("Could not load {}, because: {}", input_img_path.display(), reason),
    };

    println!("Decoded {} x {} image at {}", bitmap.width, bitmap.height,
        input_img_path.to_str().expect("path should be valid"));

    if verbose { println!("Calculating pixel energies..."); }
    let energies = carving::calculate_energy(bitmap.width, bitmap.height, bitmap.buffer.as_mut());
    // if verbose { println!("Energies: {:?}", energies); }

    if matches.opt_present("e") {
        println!("Converting image to display its energies");
        for (pixel, energy) in bitmap.buffer.as_mut().iter_mut().zip(energies.iter()) {
            let relative_energy = (energy / carving::MAX_PIXEL_ENERGY * 255.0) as u8;
            pixel.r = relative_energy;
            pixel.g = relative_energy;
            pixel.b = relative_energy;
        }
    }

    let seam = carving::find_seam(bitmap.width, &energies);
    if verbose {
        println!("Seam found: {:?}", seam);
        println!("As energy:  {:?}", seam.iter().map(|seam_pixel_index| energies[*seam_pixel_index]).collect::<Vec<_>>());
    }

    if matches.opt_present("p") {
        if verbose { println!("Previewing seam image to display its energies"); }

        let image_pixels = bitmap.buffer.as_mut();
        for pixel_index in seam {
            image_pixels[pixel_index].r = 255;
            image_pixels[pixel_index].g = 0;
            image_pixels[pixel_index].b = 0;
        }
    }

    match matches.opt_str("o") {
        Some(output_img_str) => {
            let output_img_path = &Path::new(&output_img_str);

            if let Err(e) = lodepng::encode24_file(output_img_path, bitmap.buffer.as_ref(), bitmap.width, bitmap.height) {
                panic!("Failed to save png to {} because: {}", output_img_str, e);
            }

            if verbose { println!("Saved output image to {}", output_img_str); }
        },
        None => println!("Not saving output image; specify -o flag if you want to save the result"),
    };
}
