extern crate clap;
extern crate serde;

mod configs;
mod generators;
mod program;
mod time_complexity;

use crate::configs::ArrayConfig;
use crate::generators::ArrayGenerator;
use crate::program::Program;
use clap::{App, Arg};
use std::convert::TryFrom;
use std::path::PathBuf;

fn main() {
    let _matches = App::new("Time analyzer")
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
                .help("Path to binary file"),
        )
        .arg(
            Arg::with_name("cfg")
                .short("c")
                .long("config")
                .value_name("FILE")
                //.required(true)
                .takes_value(true)
                .help("Path to config file"),
        )
        .get_matches();

    let times = {
        let config = ArrayConfig::default();
        let arg: ArrayGenerator<i64> = ArrayGenerator::try_from(config).unwrap();

        Program::from(
            PathBuf::from("/home/naymoll/Projects/Clion/sort.out"),
            vec![Box::new(arg)],
            3,
            1,
        )
        .exec()
        .unwrap()
    };

    println!("Times");
    for time in &times {
        println!("{:?}", time);
    }

    let zip = times.iter().zip(times.iter().skip(1));

    let times_dif: Vec<f64> = zip
        .clone()
        .map(|(f, s)| s.min.as_secs_f64() / f.min.as_secs_f64())
        .collect();
    println!("Times dif");
    println!("{:?}", times_dif);

    let len_dif: Vec<f64> = zip
        .clone()
        .map(|(f, s)| s.args_len as f64 / f.args_len as f64)
        .collect();
    println!("Len dif");
    println!("{:?}", len_dif);

    let dif: Vec<f64> = times_dif
        .iter()
        .zip(len_dif.iter())
        .map(|(t, l)| t / l)
        .collect();
    println!("Dif");
    println!("{:?}", dif);
}
