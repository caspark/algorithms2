use baseball::{BaseballDivision, BaseballTeam};
use std::env;
use std::io;

mod baseball;
mod flow_network;
mod ford_fulkerson;

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
            match div.certificate_of_elimination(team) {
                None => println!("{} is not eliminated", team),
                Some(certificates_of_elimination) => {
                    print!("{} is eliminated by the subset R = {{ ", team);
                    for ce in certificates_of_elimination {
                        print!("{} ", ce);
                    }
                    println!("}}");
                },
            }
        }
    }
}

#[cfg_attr(test, allow(dead_code))]
fn parse_baseball_division(path: &String) -> io::Result<BaseballDivision> {
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::File;

    println!("Parsing baseball teams from {}", path);

    let file_reader = BufReader::new(try!(File::open(path)));
    let mut possible_team_count = None;
    let mut teams = Vec::new();
    let mut w = Vec::new();
    let mut l = Vec::new();
    let mut r = Vec::new();
    let mut g = Vec::new();

    for line_result in file_reader.lines() {
        let line = try!(line_result);
        let line = line.trim(); // trimming is necessary to deal with CRLF in some input files :(
        match possible_team_count {
            None => possible_team_count = Some(line.parse::<usize>().unwrap()),
            Some(team_count) => {
                let mut gi = vec![0; team_count];
                let components = line.split(" ");
                for (i, component) in components.filter(|s| s.len() > 0).enumerate() {
                    match i {
                        0 => teams.push(BaseballTeam(component.to_owned())),
                        1 => w.push(component.parse::<i32>().ok().expect("win count should be a number")),
                        2 => l.push(component.parse::<i32>().ok().expect("loss count should be a number")),
                        3 => r.push(component.parse::<i32>().ok().expect("remaining games count should be a number")),
                        _ => gi[i - 4] = component.parse::<i32>().ok()
                            .expect(format!("remaining games against team {} count should be a number", i - 3).as_ref()),
                    }
                }
                assert_eq!(gi.len(), team_count);
                g.push(gi);
            }
        }
    }
    let team_count = possible_team_count.expect("Team count should have been parsed by now");
    assert_eq!(teams.len(), team_count);
    Ok(BaseballDivision::new(teams, w, l, r, g))
}
