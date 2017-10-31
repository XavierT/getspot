
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate clap;


use clap::{Arg, App};
use log::LogLevel;

fn main() {

  let matches = App::new("getspot")
                          .version("0.1")
                          .author("Xavier T. <sseingalt@gmail.com>")
                          .about("Copy pictures from Spotlight directory")
                          .arg(Arg::with_name("target")
                               .short("t")
                               .long("target")
                               .value_name("PATH")
                               .help("Sets output picture directory")
                               .takes_value(true))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("target").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    let level;

// pub enum LogLevel {
//     Error,
//     Warn,
//     Info,
//     Debug,
//     Trace,
// }   

    match matches.occurrences_of("v") {
        0 => { level = LogLevel::Error},
        1 => { level = LogLevel::Warn},
        2 => { level = LogLevel::Info},
        3 | _ => { level = LogLevel::Trace},
    }
     
    simple_logger::init_with_level(level).unwrap();

    info!("This is an example message.");
    warn!("This is an example message.");
    error!("This is an example message.");

}

