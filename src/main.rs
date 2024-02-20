extern crate regex;
extern crate serde;
extern crate serde_json;

mod controller;
mod domain;
mod infrastructure;
mod presentation;
mod use_case;

use controller::get_proposal_data_controller::get_proposal_data_controller;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    url: String,
}

#[derive(Parser, Debug)]
enum Commands {
    #[command(about = "Fetch the proposal page from the given url")]
    Fetch,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Fetch) => {
            get_proposal_data_controller(&cli.url);
        }
        None => {
            println!("No command provided. Exiting...");
        }
    }
}
