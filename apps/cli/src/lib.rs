use thiserror::Error;

use args::{Args, Subcommand, ProfileSubcommand};

pub mod commands;
pub mod args;

#[derive(Debug, Error)]
pub enum CommandError {}

pub async fn run(args: Args) -> Result<(), CommandError> {
    if let Some(subcommand) = args.subcommand {
        match subcommand {
            Subcommand::Player => todo!(),
            Subcommand::Studio => todo!(),
            Subcommand::Profile(profile_subcommand) => {
                match profile_subcommand {
                    ProfileSubcommand::Create => commands::profile::create().await,
                    ProfileSubcommand::Delete { uuid: _, name: _ } => commands::profile::delete(),
                    ProfileSubcommand::List => commands::profile::list(),
                }
            }
        }
    } else {
        unreachable!("Missing subcommand");
    }
}