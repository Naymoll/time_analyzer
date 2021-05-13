extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate validator;

mod complexity;
mod configs;
mod program;
mod report;
mod run;

use clap::{App, Arg};

use crate::complexity::LeastSquares;
use crate::program::Program;
use crate::report::Report;
use std::path::Path;

fn main() {
    let matches = App::new("Time analyzer")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("cfg")
                .short("c")
                .long("configs")
                .value_name("FILE")
                .required(true)
                .takes_value(true)
                .multiple(true)
                .help("Path to config file"),
        )
        .get_matches();

    let cfg_paths = matches.values_of_os("cfg").unwrap();

    for cfg_path in cfg_paths {
        let cfg_path: &Path = cfg_path.as_ref();
        let mut program = match Program::load_from_config(cfg_path) {
            Ok(program) => program,
            Err(error) => {
                println!(
                    "Failed to load config file '{}'. {}",
                    cfg_path.display(),
                    error
                );
                continue;
            }
        };

        let runs = match program.exec() {
            Ok(runs) => runs,
            Err(error) => {
                println!(
                    "Error while execution '{}'. {}",
                    program.path().display(),
                    error
                );
                continue;
            }
        };

        let least_sq = LeastSquares::computate_big_o(&runs);
        let report = Report::new(program.path(), cfg_path, runs, least_sq);

        println!("{}\n", report);
    }
}
