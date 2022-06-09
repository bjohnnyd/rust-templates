use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Cli {
    /// Number of threads
    #[structopt(short, long, default_value = "4")]
    pub threads: usize,
    /// Determines verbosity of the processing, can be specified multiple times -vvv
    #[structopt(short, long, parse(from_occurrences))]
    pub verbosity: u8,
}

impl Cli {
    /// Sets logging level suplied by verbosity count.
    ///
    /// Levels possible are:
    ///
    /// - `Warn` (default)
    /// - `Info`
    /// - `Debug`
    /// - `Trace`
    pub fn set_logging(&self) {
        use log::LevelFilter::*;

        let log_level = match self.verbosity {
            level if level == 1 => Info,
            level if level == 2 => Debug,
            level if level > 2 => Trace,
            _ => Warn,
        };

        pretty_env_logger::formatted_builder()
            .format_module_path(true)
            .filter_level(log_level)
            .init();
    }
}
