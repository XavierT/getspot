
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate clap;
extern crate jpeg_decoder;

pub mod file;
pub mod find;

use std::env::consts::OS;
use std::env::home_dir;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;


use clap::{Arg, App};
use log::LogLevel;
use jpeg_decoder::Decoder;

use file::PictureFile;

fn main() {


    // Command line parameters management
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

    let spotlight_picture_dir = home_dir()
        .unwrap()
        .join("AppData")
        .join("Local")
        .join("Packages")
        .join("Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy")
        .join("LocalState")
        .join("Assets");

    let default_output_dir = home_dir().unwrap().join("Pictures").join("Spotlight");

    let mut list_of_files: Vec<PathBuf> = Vec::new();
    let mut list_of_pic: Vec<PictureFile> = Vec::new();

    find::find_in_directory(&spotlight_picture_dir, &mut list_of_files).unwrap();

    // Need to create a method with what is below
    for p in list_of_files {
        debug!("{:?}",p);

        let file = File::open(&p).expect("failed to open file");
        let mut decoder = Decoder::new(BufReader::new(file));

        if decoder.read_info().is_ok() {
            // This is a valid jpeg file
            if let Some(metadata) = decoder.info() {
                let pic_file = PictureFile::new().with_path(p).with_dimensions(
                    metadata.width,
                    metadata.height,
                );

                list_of_pic.push(pic_file);
            }

        }

    }

    for pic in list_of_pic {
        // println!(
        //     "dimensions : {}, name : {} ",
        //     pic.get_dimension_string(),
        //     pic.get_name().unwrap_or("None".to_string())
        // );


        let pic1 = pic.clone();
        let pic2 = pic.clone();
        let pic3 = pic.clone();

        // TODO there is a better solution here than to clone the pic
        let source_pathbuf = pic1.path.unwrap();
        let destination = default_output_dir.join(pic2.get_dimension_string());
        let name = pic3.get_name().unwrap();

        find::copy_to_dir(
            source_pathbuf.as_path(),
            destination.as_path(),
            name.as_ref(),
        );

    }


}
