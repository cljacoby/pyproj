use std::env;

use pyproj::cli::cli_parser;
use pyproj::err::PyProjErr;
use pyproj::PyProj;

fn main() {
    let matches = cli_parser().get_matches_from(env::args_os());
    match matches.subcommand() {
        ("new", Some(sub_m)) => match sub_m.value_of("name") {
            Some(name) => {

                let cwd = match env::current_dir() {
                    Err(err) => {
                        println!("error: {:?}", err);
                        std::process::exit(1);
                    }
                    Ok(cwd) => cwd,
                };

                let pyproj = PyProj::new(name, cwd);
                match pyproj.create() {
                    Err(err) => {
                        println!("error: {:?}", err);
                        std::process::exit(1);
                    }
                    Ok(val) => {
                        println!("pyroj.crete() was Ok. Result = {:?}", val);
                    }
                };

                

            }
            None => println!("name was none"),
        },
        ("init", Some(sub_m)) => println!("{:?}", sub_m),
        _ => {
            println!("unrecognized command!");
        }
    }
}
