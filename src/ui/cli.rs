pub struct Cli {}

impl RunApp for Cli {
    fn run_app() -> Result<Option<Box<dyn Any>>, crate::Error> {
        let cli_input = CliTemplate::parse();

        let _worker_guards = Cli::init_log(
            cli_input
                .global_arguments
                .verbosity_filter(),
        )
        .context(LoggingSnafu {})
        .context(AppSnafu {})?;

        tracing::info!("{:#?}", cli_input);
        tracing::trace!("This is {}", "trace!".color(AnsiColors::Magenta));
        tracing::debug!("This is {}", "debug!".color(AnsiColors::Blue));
        tracing::info!("This is {}", "info!".color(AnsiColors::Green));
        tracing::warn!("This is {}", "warn!".color(AnsiColors::Yellow));
        tracing::error!("This is {}", "error!".color(AnsiColors::Red));

        Ok(Some(Box::new(_worker_guards)))
    }
}

impl InitLog for Cli {}

pub trait CliModifier {
    fn verbosity_filter(&self) -> Option<LevelFilter>;
    fn is_uncolored(&self) -> bool;
    fn is_json(&self) -> bool;
}

impl CliModifier for GlobalArguments {
    fn verbosity_filter(&self) -> Option<LevelFilter> {
        if self.plain_flag || self.json_flag {
            return Some(LevelFilter::INFO);
        }

        let verbosity_flag_filter = self
            .verbose
            .log_level_filter();

        if verbosity_flag_filter < clap_verbosity_flag::LevelFilter::Debug && self.debug_flag {
            return Some(LevelFilter::DEBUG);
        }

        verbosity_flag_filter
            .as_str()
            .parse()
            .ok()
    }

    fn is_uncolored(&self) -> bool {
        self.plain_flag || self.json_flag
    }

    fn is_json(&self) -> bool {
        self.json_flag
    }
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    Dummy {},
}

// region: IMPORTS

use crate::{
    app::{logging::InitLog, LoggingSnafu, RunApp},
    cli::cli_template::CliTemplate,
    AppSnafu,
};
use clap::Parser;
use clap_verbosity_flag::{Level, LogLevel};
use owo_colors::{AnsiColors, OwoColorize};
use snafu::{ResultExt, Snafu};
use std::any::Any;
use tracing_subscriber::filter::LevelFilter;

use self::cli_template::GlobalArguments;

// endregion: IMPORTS

// region: MODULES

mod cli_template {
    #[derive(Parser, Debug)]
    #[command(version, author, about, args_conflicts_with_subcommands = true)]
    pub struct CliTemplate {
        #[clap(flatten)]
        pub global_arguments: GlobalArguments,
    }

    #[derive(Debug, Args)]
    #[clap(args_conflicts_with_subcommands = true)]
    pub struct GlobalArguments {
        #[clap(
            long = "json",
            help = "Output in the JSON format for machine readability and scripting purposes.",
            global = true
        )]
        pub json_flag: bool,

        #[clap(
            long = "plain",
            help = "Output as plain text without extra information, for machine readability and scripting purposes.",
            global = true
        )]
        pub plain_flag: bool,

        #[clap(long = "debug", help = "Output debug messages.", global = true)]
        pub debug_flag: bool,

        #[clap(flatten)]
        pub verbose: Verbosity<InfoLevel>,
    }

    // region: IMPORTS

    use clap::{Args, Parser, Subcommand};
    use clap_verbosity_flag::{InfoLevel, Verbosity};

    // endregion: IMPORTS
}

//endregion: MODULES
