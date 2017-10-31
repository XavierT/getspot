
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate clap;


fn main() {
    simple_logger::init().unwrap();

    info!("This is an example message.");
}

// pub enum LogLevel {
//     Error,
//     Warn,
//     Info,
//     Debug,
//     Trace,
// }