use clap::ArgAction;
use clap::Parser;
use uuid::Uuid;

#[derive(Debug, Parser, PartialEq, Eq)]
#[command(
arg_required_else_help = true,
about,
version,
)]
pub struct Args {
    #[clap(short, long, action = ArgAction::Count)]
    pub verbose: u8,
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Debug, Parser, PartialEq, Eq)]
pub enum Subcommand {
    #[clap(about = "Launch roblox player")]
    Player,
    #[clap(about = "Launch roblox studio")]
    Studio,
    #[clap(subcommand)]
    Profile(ProfileSubcommand),
}

#[derive(Debug, Parser, PartialEq, Eq)]
pub enum ProfileSubcommand {
    Create,
    Delete {
        #[clap(short, long, conflicts_with = "name")]
        uuid: Option<Uuid>,
        #[clap(short, long, conflicts_with = "uuid")]
        name: Option<String>,
    },
    List,
}

