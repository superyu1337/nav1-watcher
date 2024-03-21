use clap::Parser;
use watcher::Watcher;

use crate::cli::Cli;

mod config;
mod cli;
mod watcher;
mod webhook;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Check(args) => {
            let mut watcher = Watcher::new(&args.config, args.no_webhook);
            watcher.run();
        },
        cli::Commands::Datadir => {
            if let Some(proj_dirs) = directories::ProjectDirs::from("xyz", "superyu", "nav1truenas") {
                let data_dir = proj_dirs.data_dir().to_path_buf();
                println!("Data directory: {}", data_dir.to_str().expect("Failed to convert pathbuf to string"));
            }
        },
        cli::Commands::Cleardata => {
            if let Some(proj_dirs) = directories::ProjectDirs::from("xyz", "superyu", "nav1truenas") {
                let data_dir = proj_dirs.data_dir().to_path_buf();
                std::fs::remove_dir_all(data_dir).expect("Could not delete data directory");
                println!("Removed data directory!");
            } else {
                panic!("Could not get data directory")
            }
        },
        
    }
}
