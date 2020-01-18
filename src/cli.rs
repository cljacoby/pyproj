extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

pub const APP_NAME: &'static str = "pyproj";
pub const APP_VERSION: &'static str = "0.0.1";
pub const APP_AUTHOR: &'static str = "Chris Jacoby";
pub const APP_ABOUT: &'static str = "Manage Python Projects.";

pub const APP_NEW_NAME: &'static str = "pyproj";

// TODO: I don't actually understand what adding these lifetime parameters
// did in order to let me compile. Get a better understanding.

// TODO: Change usage variable value from 'command' to 'cmd'

// TODO: Assess exiting with usage on subcommands

pub fn cli_parser<'a, 'b>() -> App<'a, 'b> {
    App::new(APP_NAME)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("new")
                .setting(AppSettings::ArgRequiredElseHelp)
                .about("Create a new python project.")
                .arg(
                    Arg::with_name("name")
                        .value_name("name")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("init").about("Init python project in current directory."),
        )
}
