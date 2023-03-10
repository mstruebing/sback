pub struct Logger {
    pub verbose: bool,
    pub debug: bool,
}

impl Logger {
    pub fn log(&self, msg: &str) {
        println!("{}", msg)
    }

    pub fn debug(&self, msg: &str) {
        if self.debug {
            println!("{}", msg)
        }
    }

    pub fn verbose(&self, msg: &str) {
        if self.verbose || self.debug {
            println!("{}", msg)
        }
    }
}
