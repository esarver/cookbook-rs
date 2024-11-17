#![feature(pattern)]

use clap::Parser;
use menu::{Cookbook, Meal};

pub mod error;
pub mod log;
pub mod menu;

fn main() -> anyhow::Result<()> {
    let cli = cli::Parser::parse();

    if cli.verbose {
        log::start_stderr_logger()?;
    }

    if !cli.data.exists() {
        let _ = std::fs::File::create(&cli.data)?;
    }
    let mut cookbook = Cookbook::connect(&cli.data)?;

    match cli.subcommand {
        cli::SubCommand::Add { name, tags } => {
            cookbook.add(Meal::new(name, tags))?;
        }
        cli::SubCommand::Info { name } => {
            cookbook.info(name)?;
        }
        cli::SubCommand::List => {
            for (i, meal) in cookbook.list().iter().enumerate() {
                println!("{i}: {}", meal.name);
            }
        }
        cli::SubCommand::Search { pattern } => {
            cookbook.search(&pattern)?;
        }
    }

    Ok(())
}

pub mod cli {

    use std::path::PathBuf;

    #[derive(Debug, clap::Parser)]
    pub struct Parser {
        /// Show all log messages
        #[arg(long, short)]
        pub verbose: bool,

        /// The database to which to connect to, defaults to `"${PWD}/cookbook.json"`
        #[arg(long, short, default_value = "./cookbook.json")]
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

        /// Show the detailed information about the meal with the given name
        Info {
            /// The name of the meal about which you want information
            name: String,
        },

        /// List the names of all meals in the cookbook
        List,

        /// Find a meal using the provided pattern
        Search {
            /// The pattern to use to find the given meal
            pattern: String,
        },
    }
}
