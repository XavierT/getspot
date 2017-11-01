
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate clap;

pub mod file;

use clap::{Arg, App};
use log::LogLevel;
use std::env::consts::OS;

fn main() {

    // Command line paramenters management
    let matches = App::new("getspot")
        .version("0.1")
        .author("Xavier T. <sseingalt@gmail.com>")
        .about(
            "Copy pictures from Spotlight directory. Only works on Windows 10",
        )
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .value_name("PATH")
                .help("Sets output picture directory")
                .takes_value(true),
        )
        .arg(Arg::with_name("v").short("v").multiple(true).help(
            "Sets the level of verbosity, from -v to -vvv",
        ))
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("target").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // pub enum LogLevel {
    //     Error,
    //     Warn,
    //     Info,
    //     Debug,
    //     Trace,
    // }

    // logging configuration
    let log_level;

    match matches.occurrences_of("v") {
        0 => log_level = LogLevel::Error,
        1 => log_level = LogLevel::Warn,
        2 => log_level = LogLevel::Info,
        3 | _ => log_level = LogLevel::Trace,
    }

    simple_logger::init_with_level(log_level).unwrap();

    // TODO it would be better to detect wich Windows version is running..
    if OS == "windows" {
        debug!("yeah windows");
    } else {
        error!("getspot only runs on Windows.");
        std::process::exit(0);
    }


}
