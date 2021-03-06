extern crate itertools;
#[cfg(test)]
extern crate quickcheck;

use std::env;
use std::error::Error;
use std::io;

mod burrows_wheeler_transform;
mod circular_suffix_array;
mod move_to_front;

// Use for for debugging
macro_rules! printerrln(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let args: Vec<_> = env::args().collect();

    let print_usage = || {
        let program_name = &args[0];
        println!("Usage: {} [move-to-front|burrows-wheeler] [encode|decode]", program_name);
    };

    if args.len() != 3 {
        println!("Error: unexpected number of arguments!");
        print_usage();
    } else {
        let stdin = io::stdin();
        let stdin_lock = stdin.lock();
        let stdout = io::stdout();
        let stdout_lock = &mut stdout.lock();

        let result = match (args[1].as_ref(), args[2].as_ref()) {
            ("move-to-front", "encode") => move_to_front::encode(stdin_lock, stdout_lock),
            ("move-to-front", "decode") => move_to_front::decode(stdin_lock, stdout_lock),
            ("burrows-wheeler", "encode") => burrows_wheeler_transform::encode(stdin_lock, stdout_lock),
            ("burrows-wheeler", "decode") => burrows_wheeler_transform::decode(stdin_lock, stdout_lock),
            (operation, command) => {
                let msg = format!("unrecognised arguments: {} {}\nRun with no arguments to see available arguments.", operation, command);
                Err(io::Error::new(io::ErrorKind::InvalidInput, msg))
            },
        };
        if let Err(error) = result {
            println!("Error encountered: {}", error.description());
        }
    }
}
