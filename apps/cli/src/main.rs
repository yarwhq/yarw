use std::env;

use clap::Parser;
use log::{debug, error};
use yarw_cli::args::Args;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.verbose {
        0 => env::set_var("RUST_LOG", "info"),
        1 => env::set_var("RUST_LOG", "debug"),
        n if n > 1 => env::set_var("RUST_LOG", "trace"),
        _ => {}
    };

    debug!("args: {:?}", args);

    pretty_env_logger::init();

    match yarw_cli::run(args).await {
        Ok(_) => {}
        Err(err) => {
            error!("{}", err);
            std::process::exit(1);
        }
    }
}
