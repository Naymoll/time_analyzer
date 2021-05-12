extern crate clap;
extern crate serde;
extern crate serde_json;

mod complexity;
mod configs;
mod program;
mod report;
mod run;

use crate::complexity::computate_big_o;
use crate::configs::{ArgumentGenerator, Config};
use crate::program::Program;
use crate::report::Report;
use clap::{App, Arg};
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;

fn load_cfg(cfg_path: &OsStr) -> Vec<Box<dyn ArgumentGenerator>> {
    let json_str = {
        let mut buff = String::new();
        let mut reader = File::open(cfg_path).expect("Can't open config file");
        reader
            .read_to_string(&mut buff)
            .expect("Can't read from config file");

        buff
    };

    let config: Vec<Config> =
        serde_json::from_str(&json_str).expect("Error while parsing config file");
    config
        .into_iter()
        .map(|c| {
            let config: Box<dyn ArgumentGenerator> = match c {
                Config::Array(array) => Box::new(array),
                Config::Matrix(matrix) => Box::new(matrix),
                Config::Range(range) => Box::new(range),
            };
            config
        })
        .collect()
}

fn analysis(bin_path: &OsStr, cfg_path: &OsStr) {
    let runs = {
        let config = load_cfg(cfg_path);
        Program::from(bin_path, config, 15, 1).exec().unwrap()
    };

    let least_sq = computate_big_o(&runs);
    let report = Report::new(bin_path, cfg_path, runs, least_sq);

    println!("{}", report);
}

fn main() {
    let matches = App::new("Time analyzer")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .value_name("FILE")
                .required(true)
                .takes_value(true)
                //.multiple(true)
                .help("Path to binary file"),
        )
        .arg(
            Arg::with_name("cfg")
                .short("c")
                .long("configs")
                .value_name("FILE")
                .required(true)
                .takes_value(true)
                //.multiple(true)
                .help("Path to config file"),
        )
        .get_matches();

    if matches.occurrences_of("path") != matches.occurrences_of("cfg") {
        unimplemented!("Paths != cfgs");
    }

    let binary_paths = matches.values_of_os("path").unwrap();
    let config_paths = matches.values_of_os("cfg").unwrap();

    let paths = binary_paths.zip(config_paths);
    for (bin_path, cfg_path) in paths {
        analysis(bin_path, cfg_path);
    }
}
