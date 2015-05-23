extern crate itertools;

use std::env;
use std::io;

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

fn main() {
    let args: Vec<_> = env::args().collect();

    let print_usage = || {
        let program_name = &args[0];
        println!("Usage: {} move-to-front [encode|decode]", program_name);
    };

    if args.len() != 3 {
        println!("Error: unexpected number of arguments!");
        print_usage();
    } else {
        let stdin = io::stdin();
        let stdin_lock = stdin.lock();
        let stdout = io::stdout();
        let stdout_lock = &mut stdout.lock();

        match (args[1].as_ref(), args[2].as_ref()) {
            ("move-to-front", "encode") => move_to_front::encode(stdin_lock, stdout_lock),
            ("move-to-front", "decode") => move_to_front::decode(stdin_lock, stdout_lock),
            (operation, command) => {
                println!("Error: unrecognised arguments: {} {}", operation, command);
                print_usage();
            },
        };
    }
}
