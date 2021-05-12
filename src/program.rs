use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;

use crate::configs::ArgumentGenerator;
use crate::run::Run;
use std::time::Instant;

type Generators = Vec<Box<dyn ArgumentGenerator>>;

pub struct Program {
    path: PathBuf,
    args: Generators,
    gens: usize,
    iters: usize,
}

impl Program {
    pub fn from<P>(path: P, args: Generators, gens: usize, iters: usize) -> Self
    where
        P: Into<PathBuf>,
    {
        Program {
            path: path.into(),
            args,
            gens,
            iters,
        }
    }

    pub fn exec(&mut self) -> Result<Vec<Run>, ()> {
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
                self.write_args_to_file(path);

                let start_time = Instant::now();
                let command = process::Command::new(&self.path)
                    .arg(&path)
                    .output()
                    .expect("Can't start program");
                let duration = start_time.elapsed();

                if !command.status.success() {
                    unimplemented!() //TODO: Return error
                }

                run.update(duration.as_secs_f64());
            }
            run.avg /= self.iters as f64;
            runs.push(run);
        }

        Ok(runs)
    }

    fn write_args_to_file<P>(&self, path: P)
    where
        P: AsRef<Path>,
    {
        let args: Vec<String> = self.args.iter().map(|x| x.generate()).collect();
        let buf: Vec<u8> = args.into_iter().flat_map(|s| s.into_bytes()).collect();
        let mut file = File::create(path.as_ref()).expect("Can't create file");
        file.write_all(&buf).expect("Can't write to file");
    }
}
