extern crate clap;

use std::env;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};


use crate::err::PyProjErr;
use crate::PyProj;

pub const APP_NAME: &'static str = "pyproj";
pub const APP_VERSION: &'static str = "0.0.1";
pub const APP_AUTHOR: &'static str = "Chris Jacoby";
pub const APP_ABOUT: &'static str = "Manage Python Projects.";
pub const APP_NEW_NAME: &'static str = "pyproj";

// TODO: I don't actually understand what adding these lifetime parameters
// did in order to let me compile. Get a better understanding.

// TODO: Change usage variable value from 'command' to 'cmd'

// TODO: Assess exiting with usage on subcommands

pub fn parser_cmd_new<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("new")
        .about("Create a new python project.")
        .arg(
            Arg::with_name("name")
                .value_name("name")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dockerfile")
                //.short("d")
                .long("dockerfile")
                .help("Create project with dockerfile"),
        )
        .arg(
            Arg::with_name("src_name")
                .long("src-name")
                .help("Set name of the src directory")
                .takes_value(true)
        )
}

pub fn parser_cmd_init<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("init").about("Initialize python project in current directory.")
}

pub fn parser_main<'a, 'b>() -> App<'a, 'b> {
    App::new(APP_NAME)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(parser_cmd_new())
        .subcommand(parser_cmd_init())
}


// TOOD: I don't like the idea of just setting the pyproj fields with a mutable reference,
// but the builder I deleted was also not helpful, and was very verbose.
// Find a better pattern for the interface of PyProj.

pub fn cmd_new(matches: &ArgMatches) -> Result<(), PyProjErr> {
    let name = matches.value_of("name").ok_or(PyProjErr::app_err(
        "cli argument `name` was expected to be present, but was None",
    ))?;

    let mut pyproj = PyProj::new();
    pyproj.name = String::from(name);

    let cwd = env::current_dir().map_err(PyProjErr::Io)?;
    pyproj.path = Some(cwd);
    
    if matches.is_present("dockerfile") {
        pyproj.with_dockerfile = true;
    }

    if matches.is_present("src_name") {
        let src_name = matches.value_of("src_name")
            .expect("cli argument `src_name` was expected to be present, but was None.");
        pyproj.src_name = String::from(src_name);
    }

    pyproj.create()?;
    Ok(())
}

pub fn cmd_init(matches: &ArgMatches) -> Result<(), PyProjErr> {
    Ok(())
}
