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
#[command(name = "fortee-cli")]
#[command(version = "0.01")]
#[command(about = "Fortee CLI Tool")]
#[command(long_about = "Get structured information from the proposal page of the Fortee.")]
struct Args {
    /// URL of the proposal page
    #[arg(short, long)]
    url: String,
}

fn main() {
    let args = Args::parse();

    get_proposal_data_controller(&args.url);
}
