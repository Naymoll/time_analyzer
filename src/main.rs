mod application_args;
mod configs;
mod generators;
mod time_complexity;

use crate::time_complexity::Stats;
use clap::{App, Arg};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::slice::Chunks;
use std::time::{Duration, Instant};

extern crate clap;

#[allow(dead_code)]
fn write_to_file<P: AsRef<Path>>(path: P, buf: &[u8]) {
    let mut file = match File::create(&path) {
        Ok(f) => f,
        Err(e) => panic!(
            "Can't create/truncate \"{}\": {}",
            path.as_ref().display(),
            e
        ),
    };

    if let Err(e) = file.write_all(&buf) {
        panic!("Can't write to \"{}\": {}", path.as_ref().display(), e);
    }
}

fn get_statistics(chunks: Chunks<Duration>) -> Vec<Stats> {
    //TODO: Может быть не эффективно

    let mut buf = Vec::with_capacity(chunks.len());
    for chunk in chunks {
        let mut stats =
            chunk
                .iter()
                .map(|d| d.as_nanos())
                .fold(Stats::new(0, 0, 0), |mut acc, d| {
                    acc.min = d.min(acc.min);
                    acc.max = d.max(acc.max);
                    acc.avg += d;

                    acc
                });
        stats.avg /= chunk.len() as u128;

        buf.push(stats);
    }

    buf
}

fn dif(stats: Vec<Stats>) -> Stats {
    //TODO: Проверить на разных сортировках, к примеру
    let zip = stats.iter().zip(stats.iter().skip(1));
    let diffs: Vec<Stats> = zip.map(|(f, s)| s - f).collect();
    let dif_sum: Stats = diffs.iter().sum();
    dif_sum / (diffs.len() as u128)
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

    let path_to_binary = Path::new(matches.value_of("path").unwrap());
    let start_time = Instant::now();

    let command = Command::new(path_to_binary)
        .args(&["1 2"])
        .output()
        .expect("Can't start program");
    let duration = start_time.elapsed();

    println!("{}", String::from_utf8_lossy(&command.stdout));
    println!("Exec time: {}sc", duration.as_secs_f32());
}

#[cfg(test)]
mod tests {
    use crate::time_complexity::Stats;
    use crate::*;

    #[test]
    fn zip_test() {
        let vec = vec![1, 2, 3];
        let zip = vec.iter().zip(vec.iter().skip(1));

        for val in zip {
            println!("{}:{}", *val.0, *val.1);
        }
    }

    #[test]
    fn dif_test() {
        let vec = vec![
            Stats::new(0, 20, 10),
            Stats::new(20, 40, 30),
            Stats::new(40, 60, 50),
        ];

        let avg_dif = dif(vec);
        assert_eq!(avg_dif, Stats::new(20, 20, 20))
    }
}
