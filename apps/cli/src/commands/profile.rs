use dialoguer::{theme::ColorfulTheme, Input, Select};
use uuid::Uuid;
use yarw_core::profile::{Profile, ProfileError, ProfileManager, RobloxType};

use crate::CommandError;

pub async fn create(profile_manager: &mut ProfileManager) -> Result<(), CommandError> {
    let profile_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Profile name:")
        .interact()
        .map_err(CommandError::DialoguerError)?;

    let roblox_type_selection = &["Roblox Player", "Roblox Studio"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Roblox type:")
        .items(roblox_type_selection)
        .default(0)
        .interact()
        .map_err(CommandError::DialoguerError)?;

    let roblox_type = match selection {
        0 => RobloxType::RobloxPlayer,
        1 => RobloxType::RobloxStudio,
        _ => unreachable!(),
    };

    let profile = Profile {
        name: profile_name,
        roblox: roblox_type,
    };

    let uuid = profile_manager.create(profile)?;
    profile_manager.save()?;

    println!("Profile created with UUID, info: {}", uuid);

    Ok(())
}

pub fn list(profile_manager: &ProfileManager) -> Result<(), CommandError> {
    if profile_manager.profiles.is_empty() {
        println!("No profiles found");
        return Ok(());
    }

    println!("--------------------");
    for (uuid, profile) in &profile_manager.profiles {
        println!("Profile UUID: {}", uuid);
        println!("Name: {}", profile.name);
        println!("RobloxType: {:?}", profile.roblox);
        println!("---------------------");
    }
    Ok(())
}

pub fn delete(
    profile_manager: &mut ProfileManager,
    uuid: Option<Uuid>,
    name: Option<String>,
) -> Result<(), CommandError> {
    if let Some(uuid) = uuid {
        match profile_manager.delete(uuid) {
            Ok(()) => {
                profile_manager.save()?;
                println!("{}", uuid.to_string());
            }
            Err(err) => match err {
                ProfileError::ProfileNotExist => {
                    println!("Profile not found");
                }
                _ => {
                    return Err(CommandError::ProfileManagerError(err))?;
                }
            },
        }
    } else if let Some(_name) = name {
        todo!("Delete by name");
    }
    Ok(())
}
