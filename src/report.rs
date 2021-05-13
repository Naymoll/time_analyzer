use crate::complexity::{Complexity, LeastSquares};
use crate::run::Run;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Report {
    bin_path: PathBuf,
    cfg_path: PathBuf,
    runs: Vec<Run>,
    complexity: Complexity,
    coef: f64,
    rms: f64,
}

impl Report {
    pub fn new<T, P>(bin_path: P, cfg_path: P, runs: T, least_squares: LeastSquares) -> Self
    where
        T: Into<Vec<Run>>,
        P: AsRef<Path>,
    {
        Self {
            bin_path: bin_path.as_ref().to_path_buf(),
            cfg_path: cfg_path.as_ref().to_path_buf(),
            runs: runs.into(),
            complexity: least_squares.complexity,
            coef: least_squares.coef,
            rms: least_squares.rms,
        }
    }
}

//TODO: Поправить вывод
impl Display for Report {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        {
            let header = format!(
                "Binary file: {}\n\
                Config file: {}\n\
                Len            Min time(sec)   Avg time(sec)   Max time(sec)\n\
                -------------------------------------------------------------",
                self.bin_path.display(),
                self.cfg_path.display()
            );

            writeln!(f, "{}", header)?;
        }

        {
            for run in &self.runs {
                let line = format!(
                    "{:<12.5}{:>16.5}{:>16.5}{:>16.5}",
                    run.len, run.min, run.avg, run.max
                );

                writeln!(f, "{}", line)?;
            }
        }

        let complexity = format!(
            "Complexity: {:.2} {}\nRMS: {:.2}%",
            self.coef,
            self.complexity,
            self.rms * 100.0
        );

        write!(f, "{}", complexity)
    }
}

#[cfg(test)]
mod tests {
    use crate::complexity::{Complexity, LeastSquares};
    use crate::report::Report;
    use crate::run::Run;

    fn new_report() -> Report {
        let run1 = Run {
            min: 20.012939898124,
            max: 40.0032942304923049023,
            avg: 30.002340923050324902,
            len: 201,
        };

        let run2 = Run {
            min: 40.0,
            max: 60.0,
            avg: 50.0,
            len: 4000,
        };

        let squares = LeastSquares {
            coef: 10.0,
            complexity: Complexity::ON,
            rms: 0.23,
        };

        let report = Report::new(
            "some patj/asdkgsi/123.txt",
            "some/path.json",
            vec![run1, run2],
            squares,
        );

        report
    }

    #[test]
    fn display_report() {
        println!("{}", new_report());
    }

    #[test]
    fn debug_report() {
        println!("{:?}", new_report());
    }
}
