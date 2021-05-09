use crate::configs::ArgumentGenerator;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{fmt, process, time};

type Generators = Vec<Box<dyn ArgumentGenerator>>;

pub struct Program {
    path: PathBuf,
    args: Generators,
    gens: usize,
    iters: usize,
}

impl Program {
    pub fn from(path: PathBuf, args: Generators, gens: usize, iters: usize) -> Self {
        Program {
            path,
            args,
            gens,
            iters,
        }
    }

    pub fn exec(&mut self) -> Result<Vec<TimeStat>, ()> {
        let mut times = Vec::with_capacity(self.gens);

        for gen in 0..self.gens {
            let mut time = TimeStat::default();

            time.args_len = match gen {
                0 => self.args.iter().map(|x| x.len()).sum(),
                _ => self.args.iter_mut().map(|x| x.next_len()).sum(),
            };

            for iter in 0..self.iters {
                let path = PathBuf::from(format!("./gen_{}_inter{}.txt", gen, iter));
                write_args_to_file(&path, &self.args);

                let start_time = time::Instant::now();
                let command = process::Command::new(&self.path)
                    .arg(&path)
                    .output()
                    .expect("Can't start program");
                let duration = start_time.elapsed();

                if !command.status.success() {
                    unimplemented!() //TODO: Return error
                }

                time.add(duration);
            }
            time.avg /= self.iters as u32;
            times.push(time);
        }

        Ok(times)
    }
}

fn write_args_to_file(path: &Path, generators: &[Box<dyn ArgumentGenerator>]) {
    let args: Vec<String> = generators.iter().map(|x| x.generate()).collect();
    let buf: Vec<u8> = args.into_iter().flat_map(|s| s.into_bytes()).collect();
    let mut file = File::create(&path).expect("Can't create file");
    file.write_all(&buf).expect("Can't write to file");
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct TimeStat {
    pub min: Duration,
    pub max: Duration,
    pub avg: Duration,
    pub args_len: usize,
}

impl TimeStat {
    pub fn add(&mut self, duration: Duration) {
        self.min = self.min.min(duration);
        self.max = self.max.max(duration);
        self.avg += duration;
    }
}

impl Debug for TimeStat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("TimeStatistic")
            .field("min", &self.min.as_secs_f64())
            .field("max", &self.max.as_secs_f64())
            .field("avg", &self.avg.as_secs_f64())
            .field("args_len", &self.args_len)
            .finish()
    }
}

impl Default for TimeStat {
    fn default() -> Self {
        Self {
            min: Duration::new(u64::MAX, 0),
            max: Duration::new(0, 0),
            avg: Duration::new(0, 0),
            args_len: 0,
        }
    }
}
