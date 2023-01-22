use crate::parsers::master_playlist::MasterPlaylist;
use clap::Parser;
use std::{path::PathBuf, process::exit};
use utils_lib::make_request_and_save_to_file;

mod error_lib;
mod parsers;
mod tag_lib;
mod utils_lib;

#[derive(Parser)]
#[command(name = "HLS Parser")]
#[command(author = "Christian Cecilia <christian.cecilia1@gmail.com@gmail.com>")]
#[command(version = "0.0.1")]
#[command(about = "Parses an HLS stream for all relevant data and serializes tags", long_about = None)]
struct CLI {
    #[arg(long, help = "Stream url to fetch manifest from.")]
    stream_url: Option<String>,
    #[arg(long, help = "Stream url to fetch manifest from.")]
    master_file: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli_args = CLI::parse();
    let mut master_playlist = MasterPlaylist::new();

    if let Some(stream_url) = cli_args.stream_url {
        let stream_url = stream_url;
        let master_download = make_request_and_save_to_file(&stream_url).await;

        match master_download {
            Ok(file_path) => {
                if master_playlist.parse_from_file(&file_path).is_err() {
                    eprintln!("Failed to parse master playlist");
                };
                println!("{:?}", master_playlist)
            }
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    }

    if let Some(master_file_path_str) = cli_args.master_file {
        let master_file_pathbuf = PathBuf::from(master_file_path_str);
        if master_playlist
            .parse_from_file(&master_file_pathbuf)
            .is_err()
        {
            eprintln!("Failed to parse master playlist");
        };
        println!("{:?}", master_playlist)
    }
}
