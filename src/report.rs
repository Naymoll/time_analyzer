//! Итоговый отчет.

use crate::complexity::{Complexity, LeastSquares};
use crate::run::Run;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};

/// Итоговый отчет по запуску пользовательской программы.
#[derive(Debug)]
pub struct Report {
    /// Путь до пользовательской программы.
    bin_path: PathBuf,
    /// Путь до конфигурационного файла.
    cfg_path: PathBuf,
    /// Массив запусков.
    runs: Vec<Run>,
    /// Итоговая асимптотическая сложность.
    complexity: Complexity,
    /// Коэффициент.
    coef: f64,
    /// Ошибка.
    rms: f64,
}

impl Report {
    pub fn new<T, B, C>(bin_path: B, cfg_path: C, runs: T, least_squares: LeastSquares) -> Self
    where
        T: Into<Vec<Run>>,
        B: AsRef<Path>,
        C: AsRef<Path>,
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

//TODO: Подумать над выводом
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
            "Complexity: {} {}\nRMS: {:.2}%",
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
        let run1 = Run::default();

        let run2 = Run::default();

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
