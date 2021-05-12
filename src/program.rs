use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fmt, process};

use crate::configs::ArgumentGenerator;
use crate::program::ErrorKind::NotSuccessful;
use crate::run::Run;
use std::fmt::{Display, Formatter};
use std::time::Instant;

type Generators = Vec<Box<dyn ArgumentGenerator>>;

#[derive(Debug)]
pub enum ErrorKind {
    IoError(std::io::Error),
    NotSuccessful(Option<i32>),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(error_kind: ErrorKind) -> Self {
        Error { kind: error_kind }
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::IoError(io_error) => {
                write!(f, "{}", io_error)
            }
            ErrorKind::NotSuccessful(status) => match status {
                Some(code) => write!(f, "Program finished not successful. Exiting code: {}", code),
                None => write!(f, "Program terminated by signal"),
            },
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error {
            kind: ErrorKind::IoError(error),
        }
    }
}

pub struct Program {
    path: PathBuf,
    args: Generators,
    gens: usize,
    iters: usize,
}

impl Program {
    pub fn from<P>(path: P, args: Generators, gens: usize, iters: usize) -> Self
    where
        P: AsRef<Path>,
    {
        Program {
            path: path.as_ref().to_path_buf(),
            args,
            gens,
            iters,
        }
    }

    pub fn exec(&mut self) -> Result<Vec<Run>, Error> {
        let mut runs = Vec::with_capacity(self.gens);

        for gen in 0..self.gens {
            let mut run = Run::default();

            run.len = match gen {
                0 => self.args.iter().map(|x| x.len()).sum(),
                _ => self.args.iter_mut().map(|x| x.next_len()).sum(),
            };

            for iter in 0..self.iters {
                let file_name = format!("gen_{}_inter{}.txt", gen, iter);
                let path = Path::new(&file_name);
                self.write_args_to_file(path)?;

                let start_time = Instant::now();
                let command = process::Command::new(&self.path)
                    .arg(&path)
                    .output()
                    .expect("Can't start program");
                let duration = start_time.elapsed();

                if !command.status.success() {
                    return Err(Error::new(NotSuccessful(command.status.code())));
                }

                run.update(duration.as_secs_f64());
            }
            run.avg /= self.iters as f64;
            runs.push(run);
        }

        Ok(runs)
    }

    fn write_args_to_file<P>(&self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let args: Vec<String> = self.args.iter().map(|x| x.generate()).collect();
        let buf: Vec<u8> = args.into_iter().flat_map(|s| s.into_bytes()).collect();
        let mut file = File::create(path.as_ref())?;
        file.write_all(&buf)?;

        Ok(())
    }
}
