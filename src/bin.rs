use std::env;

use pyproj::cli::{cmd_init, cmd_new, parser_main};
use pyproj::err::PyProjErr;
use pyproj::{PyProj, PyProjBuilder};

// fn main() -> Result<(), Box<std::error::Error>> {
fn main() {
    let matches = parser_main().get_matches_from(env::args_os());
    // println!("{:?}", matches);

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

                let pyproj = PyProj::new(name.to_string(), cwd);

                // let pyproj_builder = PyProjBuilder::new()
                //     .with_name(&name.to_string());

                // if matches.is_present("dockerfile") {
                //     let pyproj_builder = pyproj_builder.with_dockerfile();
                // }

                // let pyproj = pyproj_builder.build()?;

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

    match matches.subcommand() {
        ("new", Some(sub_matches)) => {
            cmd_new(sub_matches);
        }
        ("init", Some(sub_matches)) => {
            cmd_init(sub_matches);
        }
        _ => println!("command value was unrecognized"),
    }
}
