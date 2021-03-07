use crate::generators::ArgGen;

pub struct AppArgs {
    args: Vec<Box<dyn ArgGen>>,
    takes: u8,
    iterations: u8,
    remain_iter: u16,
}

impl AppArgs {
    pub fn new(args: Vec<Box<dyn ArgGen>>, takes: u8, iterations: u8) -> Self {
        AppArgs {
            args,
            takes,
            iterations,
            remain_iter: (takes as u16) * (iterations as u16),
        }
    }
}

impl Iterator for AppArgs {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remain_iter == 0 {
            return None;
        }

        let max = (self.takes as u16) * (self.iterations as u16);
        let buf = match self.remain_iter % (self.iterations as u16) == 0 {
            true if self.remain_iter == max => self.args.iter().map(|x| x.generate()).collect(),
            true => self.args.iter_mut().map(|x| x.generate_next()).collect(),
            false => self.args.iter().map(|x| x.generate()).collect(),
        };

        self.remain_iter -= 1;
        Some(buf)
    }
}
