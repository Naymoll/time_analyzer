use crate::configs::{ArgumentGenerator, Config};
use crate::run::Run;

use serde::Deserialize;
use validator::Validate;

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{fmt, process};

#[derive(Debug)]
pub enum ErrorKind {
    FailedToStart(std::io::Error),
    NotSuccessful(Option<i32>),
    CantWriteArgs(PathBuf, std::io::Error),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn failed_to_start(error: std::io::Error) -> Self {
        Error {
            kind: ErrorKind::FailedToStart(error),
        }
    }

    pub fn not_successful(status: Option<i32>) -> Self {
        Error {
            kind: ErrorKind::NotSuccessful(status),
        }
    }

    pub fn cant_write_args(path: PathBuf, error: std::io::Error) -> Self {
        Error {
            kind: ErrorKind::CantWriteArgs(path, error),
        }
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::FailedToStart(io_error) => {
                write!(f, "Failed to start program. {}", io_error)
            }
            ErrorKind::CantWriteArgs(path, io_error) => {
                write!(
                    f,
                    "Can't write arguments to temporary file '{}'. {}",
                    path.display(),
                    io_error
                )
            }
            ErrorKind::NotSuccessful(status) => match status {
                Some(code) => write!(
                    f,
                    "Program finished not successful. Exiting code '{}'",
                    code
                ),
                None => write!(f, "Program terminated by signal"),
            },
        }
    }
}

//Специальная стуктура, с помощью который валидируются данные,
// происзодить преобразование с Vec<Config> в Generators
#[derive(Deserialize, Validate)]
struct ProgramConfig {
    path: PathBuf,
    path_to_temp: PathBuf,
    args: Vec<Config>,
    #[validate(range(min = 1))]
    gens: usize,
    #[validate(range(min = 1))]
    iters: usize,
}

type Generators = Vec<Box<dyn ArgumentGenerator>>;

pub struct Program {
    path: PathBuf,
    path_to_temp: PathBuf,
    args: Generators,
    gens: usize,
    iters: usize,
}

impl From<ProgramConfig> for Program {
    fn from(config: ProgramConfig) -> Self {
        let args = config
            .args
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

        Program {
            path: config.path,
            path_to_temp: config.path_to_temp,
            args,
            gens: config.gens,
            iters: config.iters,
        }
    }
}

impl Program {
    pub fn load_from_config<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let json = {
            let mut file = File::open(path.as_ref())?;
            let mut buff = String::new();
            file.read_to_string(&mut buff)?;

            buff
        };

        let program_config: ProgramConfig = serde_json::from_str(&json)?;
        program_config.validate()?;

        Ok(program_config.into())
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
                let file_name = format!(
                    "{}/generation_{}_interation{}.txt",
                    self.path_to_temp.display(),
                    gen,
                    iter
                );
                let path = Path::new(&file_name);
                self.write_args_to_file(path)?;

                let start_time = Instant::now();
                let command = process::Command::new(&self.path)
                    .arg(&path)
                    .output()
                    .map_err(Error::failed_to_start)?;
                let duration = start_time.elapsed();

                if !command.status.success() {
                    return Err(Error::not_successful(command.status.code()));
                }

                run.update(duration.as_secs_f64());
            }
            run.avg /= self.iters as f64;
            runs.push(run);
        }

        Ok(runs)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn write_args_to_file<P>(&self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        let args: Vec<String> = self.args.iter().map(|x| x.generate()).collect();
        let buf: Vec<u8> = args.join(" ").into_bytes(); //Разделитель между значениями

        let mut file =
            File::create(path).map_err(|e| Error::cant_write_args(path.to_path_buf(), e))?;
        file.write_all(&buf)
            .map_err(|e| Error::cant_write_args(path.to_path_buf(), e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::program::{Program, ProgramConfig};
    use validator::Validate;

    #[test]
    fn validate_test() {
        let json = r#"{"path":"123", "args":[], "gens":1, "iters":1}"#;
        let config: ProgramConfig = serde_json::from_str(&json).unwrap();
        config.validate().unwrap();

        let _program: Program = config.into();
    }
}
