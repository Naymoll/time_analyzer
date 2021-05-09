extern crate clap;
extern crate serde;
extern crate serde_json;

mod configs;
mod program;
mod time_complexity;

use crate::configs::{ArgumentGenerator, Config};
use crate::program::Program;
use clap::{App, Arg};
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use crate::time_complexity::computate_big_o;

fn load_cfg(cfg_path: &OsStr) -> Vec<Box<dyn ArgumentGenerator>> {
    let json_str = {
        let mut buff = String::new();
        let mut reader = File::open(cfg_path).expect(""); //TODO: Вывод ошибки
        reader.read_to_string(&mut buff).expect(""); //TODO: Вывод ошибки

        buff
    };

    let config: Vec<Config> = serde_json::from_str(&json_str).expect("Can't parse config file");
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

    result
}

fn analysis(bin_path: &OsStr, cfg_path: &OsStr) {
    let times = {
        let config = load_cfg(cfg_path);

        Program::from(PathBuf::from(bin_path), config, 15, 1)
            .exec()
            .unwrap()
    };

    println!("Times");
    for time in &times {
        println!("{:?}", time);
    }

    let sq = computate_big_o(&times);
    println!("Complexity: {}\nRMS: {}%\nCoefficient: {}%", sq.complexity, sq.rms * 100.0, sq.coef * 100.0);

    let zip = times.iter().zip(times.iter().skip(1));
    let difs: Vec<(f64, f64)> = zip
        .map(|(f, s)| {
            (
                s.min.as_secs_f64() / f.min.as_secs_f64(),
                s.args_len as f64 / f.args_len as f64,
            )
        })
        .collect();
    println!("Difference between steps");
    println!("{:?}", difs);

    let attitude: Vec<f64> = difs.into_iter().map(|(t, l)| t / l).collect();
    println!("Time/len attitude");
    println!("{:?}", attitude);
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

    /*let times_dif: Vec<f64> = zip
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
    println!("{:?}", dif);*/
}
