lazy_static! {
    pub static ref APP_NAME: &'static str = "aldm";
}

pub trait RunApp {
    fn run_app() -> Result<Vec<WorkerGuard>, crate::Error>;
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("Logging:\n  {source}"), visibility(pub))]
    Logging { source: logging::Error },

    #[non_exhaustive]
    #[snafu(display("Config:\n  {source}"), visibility(pub))]
    Config { source: config::Error },
}

// region: IMPORTS

use lazy_static::lazy_static;
use snafu::Snafu;

// endregion: IMPORTS

// region: MODULES

pub mod config;
pub mod logging;

// endregion: MODULES

// region: RE-EXPORTS

pub use config::*;
pub use logging::*;
use tracing_appender::non_blocking::WorkerGuard;

// endregion: RE-EXPORTS
