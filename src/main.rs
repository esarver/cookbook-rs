use anyhow;

use clap::Parser;

pub mod error {

    #[derive(Debug, thiserror::Error)]
    pub enum Error {

    }
}

pub mod menu {
    use serde::{Deserialize, Serialize};

    use crate::error;

    use tracing::error;

    pub struct Cookbook {
        book: Vec<Meal>,
    }

    impl Cookbook {
        pub async fn connect(file: PathBuf) -> Result<(), error::Error> {
            let file = File::open(file)?;
            let file = BufReader::new(file);

            let book = serde_json::from_reader(file)?;
        }

        pub fn add(&mut self, meal: Meal) -> Result<u64, error::Error> {

        }

        pub fn search(&mut self, pattern: String) -> Result<Vec<Meal>, error::Error> {

        }

        pub fn info(&mut self, name: String) -> Result<Meal, error::Error> {

        }

        pub fn commit(&mut self) -> Result<(), error::Error> {

        }
    }

    impl Drop for Cookbook {
        fn drop(self) {
            if let Err(e) = self.commit() {
                error!("Error committing cookbook: {e}");
            }

        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Meal {
        pub name: String,
        pub tags: Vec<String>,
    }


}

pub mod cli {

    #[derive(Debug, clap::Parser)]
    pub struct Parser {
        /// Show all log messages
        #[arg(long, short)]
        pub verbose: bool,

        /// The database to which to connect to, defaults to PWD
        #[arg(long, short, default_value_t = ".")]
        pub data: PathBuf,

        #[command(subcommand)]
        pub subcommand: SubCommand,
    }

    #[derive(Debug, clap::Subcommand)]
    pub enum SubCommand {
        /// Add a meal to the database
        Add {
            /// The name of the meal to be created
            name: String,

            /// The tags to be associated with a meal
            #[arg(long, short)]
            tags: Option<Vec<String>>,
        },

        /// Find a meal using the provided pattern
        Search {
            /// The pattern to use to find the given meal
            pattern: String,
        },

        /// Show the detailed information about the meal with the given name
        Info {
            /// The name of the meal about which you want information
            name: String,
        },
    }
}

pub mod log {
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::{Registry, prelude::*};

    pub fn start_stderr_logger() -> anyhow::Result<()>{
        let err = tracing_subscriber::fmt::layer()
            .with_ansi(true)
            .with_writer(std::io::stderr);

        let logger = Registry::default().with(LevelFilter::TRACE).with(err);

        tracing::subscriber::set_global_default(logger)?;
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let cli = cli::Parser::parse();

    if cli.verbose {
        log::start_stderr_logger()?;
    }



    Ok(())
}
