
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate clap;


use clap::{Arg, App};

fn main() {
    simple_logger::init().unwrap();

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


    info!("This is an example message.");

}

// pub enum LogLevel {
//     Error,
//     Warn,
//     Info,
//     Debug,
//     Trace,
// }