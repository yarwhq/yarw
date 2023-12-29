use thiserror::Error;

use args::{Args, ProfileSubcommand, Subcommand};
use yarw_core::profile::{ProfileError, ProfileManager}; // Assuming ProfileManager is defined in yarw_core::profile

pub mod args;
pub mod commands;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Failed to run dialoguer: {0}")]
    DialoguerError(#[from] dialoguer::Error),

    #[error("Error occurred with profile manager: {0}")]
    ProfileManagerError(#[from] ProfileError),
}

pub async fn run(args: Args) -> Result<(), CommandError> {
    if let Some(subcommand) = args.subcommand {
        let result = match subcommand {
            Subcommand::Player => todo!(),
            Subcommand::Studio => todo!(),
            Subcommand::Profile(profile_subcommand) => {
                let mut profile_manager = match ProfileManager::load() {
                    Ok(manager) => manager,
                    Err(err) => {
                        return Err(CommandError::ProfileManagerError(err))?;
                    }
                };

                match profile_subcommand {
                    ProfileSubcommand::Create => {
                        commands::profile::create(&mut profile_manager).await
                    }
                    ProfileSubcommand::Delete { uuid, name } => {
                        commands::profile::delete(&mut profile_manager, uuid, name)
                    }
                    ProfileSubcommand::List => commands::profile::list(&profile_manager),
                }
            }
        };

        result
    } else {
        unreachable!("Missing subcommand");
    }
}
