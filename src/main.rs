extern crate clap;
extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::io::Read;

use clap::{App, Arg};

use crate::complexity::computate_big_o;
use crate::configs::{ArgumentGenerator, Config};
use crate::program::Program;
use crate::report::Report;
use std::path::Path;

mod complexity;
mod configs;
mod program;
mod report;
mod run;

pub type Generators = Vec<Box<dyn ArgumentGenerator>>;

fn load_cfg(cfg_path: &Path) -> Result<Generators, Box<dyn Error>> {
    let json_str = {
        let mut buff = String::new();
        let mut reader = File::open(cfg_path)?;
        reader.read_to_string(&mut buff)?;

        buff
    };

    let config: Vec<Config> = serde_json::from_str(&json_str)?;
    let result = config
        .into_iter()
        .map(|c| {
            let config: Box<dyn ArgumentGenerator> = match c {
                Config::Array(array) => Box::new(array),
                Config::Matrix(matrix) => Box::new(matrix),
                Config::Range(range) => Box::new(range),
            };
            config
        })
        .collect();

    Ok(result)
}

fn analysis<P: AsRef<Path>>(bin_path: P, cfg_path: P) {
    let bin_path = bin_path.as_ref();
    let cfg_path = cfg_path.as_ref();

    let runs = {
        let config = match load_cfg(cfg_path) {
            Ok(cfg) => cfg,
            Err(error) => {
                println!("Error while loading '{}'. {}", cfg_path.display(), error);
                return;
            }
        };
        let mut program = Program::from(bin_path, config, 15, 1);
        match program.exec() {
            Ok(runs) => runs,
            Err(error) => {
                println!(
                    "Error while execution program '{}'. {}",
                    bin_path.display(),
                    error
                );
                return;
            }
        }
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
