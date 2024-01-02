use std::collections::HashMap;

use dialoguer::{theme::ColorfulTheme, Input, Select};
use uuid::Uuid;
use yarw_core::profile::{FFlagValue, Profile, ProfileError, ProfileManager, Renderer, RobloxType};

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

    let renderer_selection = &["D3D11", "Vulkan", "OpenGL"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select renderer:")
        .items(renderer_selection)
        .default(0)
        .interact()
        .map_err(CommandError::DialoguerError)?;

    let renderer = match selection {
        0 => Renderer::D3D11,
        1 => Renderer::Vulkan,
        2 => Renderer::OpenGL,
        _ => unreachable!(),
    };

    let mut fflags = HashMap::new();
    loop {
        let fflags_selection = &["Yes", "No"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to add FFlags?:")
            .items(fflags_selection)
            .default(0)
            .interact()
            .map_err(CommandError::DialoguerError)?;

        if selection == 0 {
            let fflag_name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("FFlag name:")
                .interact()
                .unwrap();

            let fflag_value: FFlagValue = match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select FFlag type:")
                .items(&["String", "Number", "Boolean"])
                .default(0)
                .interact()
                .map_err(CommandError::DialoguerError)?
            {
                0 => FFlagValue::String(
                    Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("FFlag value (String):")
                        .interact()
                        .map_err(CommandError::DialoguerError)?,
                ),
                1 => FFlagValue::Number(
                    Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("FFlag value (Number):")
                        .interact()
                        .map_err(CommandError::DialoguerError)?,
                ),
                2 => FFlagValue::Boolean(
                    Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("FFlag value (Boolean):")
                        .interact()
                        .map_err(CommandError::DialoguerError)?,
                ),
                _ => unreachable!(),
            };

            fflags.insert(fflag_name, fflag_value);
        } else {
            break;
        }
    }

    let profile = Profile {
        name: profile_name,
        roblox: roblox_type,
        renderer,
        fflags,
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
        println!("Renderer: {:?}", profile.renderer);
        println!("FFlags: {:?}", profile.fflags);
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
                println!("{}", uuid);
            }
            Err(err) => match err {
                ProfileError::ProfileNotExist => {
                    println!("Profile not found");
                }
                _ => {
                    Err(CommandError::ProfileManagerError(err))?;
                }
            },
        }
    } else if let Some(_name) = name {
        todo!("Delete by name");
    }
    Ok(())
}
