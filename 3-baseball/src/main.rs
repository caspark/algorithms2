use baseball::BaseballDivision;
use std::env;
use std::io;

mod baseball;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let args: Vec<_> = env::args().collect();

    let print_usage = || {
        let program_name = &args[0];
        println!("Usage: {} TEAMS-FILE", program_name);
    };

    if args.len() != 2 {
        println!("Error: unexpected number of arguments!");
        print_usage();
    } else {
        let div = parse_baseball_division(&args[1]).unwrap();

        for team in div.teams() {
            if div.is_eliminated(team) {
                print!("{} is eliminated by the subset R = {{ ", team);
                for ce in div.certificate_of_elimination(team) {
                    print!("{} ", ce);
                }
                print!("}}");
            } else {
                println!("{} is not eliminated", team);
            }
        }
    }
}

#[cfg_attr(test, allow(dead_code))]
fn parse_baseball_division(path: &String) -> io::Result<BaseballDivision> {
    println!("Parsing baseball teams from {}", path);
    unimplemented!();
}
