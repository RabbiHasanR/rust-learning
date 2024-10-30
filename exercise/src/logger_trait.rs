trait Logger {
    fn log(&self, verbosity: u8, message: &str);
}


struct StdoutLogger;

impl Logger for StdoutLogger {
    fn log(&self, verbosity: u8, message: &str) {
        println!("verbosity={verbosity}: {message}");
    }
}


struct VerbosityFilter {
    max_verbosity: u8,
    inner: StdoutLogger,
}

impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}


pub fn logger_trait_example() {
    let logger = VerbosityFilter { max_verbosity: 10, inner: StdoutLogger};
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}