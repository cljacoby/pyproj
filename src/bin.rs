use std::env;

use pyproj::cli::{cmd_init, cmd_new, parser_main};
use pyproj::err::PyProjErr;
use pyproj::PyProj;

// fn main() -> Result<(), Box<std::error::Error>> {
fn main() {
    let matches = parser_main().get_matches_from(env::args_os());
    let result = match matches.subcommand() {
        ("new", Some(sub_matches)) => cmd_new(sub_matches),
        ("init", Some(sub_matches)) => cmd_init(sub_matches),
        _ => Err(PyProjErr::app_err("Unrecognized command")),
    };

    if let Some(err) = result.err() {
        println!("Error: {:?}", err);
        std::process::exit(1);
    }
}
