extern crate regex;
extern crate serde;
extern crate serde_json;

mod controller;
mod domain;
mod infrastructure;
mod middleware;
mod presentation;
mod use_case;

use controller::get_proposal_data_controller::get_proposal_data_controller;

use crate::controller::get_proposal_data_controller::GetProposalDataDto;
use clap::Parser;
use middleware::middleware_for_get_proposal_data;

#[derive(Parser)]
#[command(name = "fortee-cli")]
#[command(version = "0.01")]
#[command(about = "Fortee CLI Tool")]
#[command(long_about = "Get structured information from the proposal page of the Fortee.")]
struct Args {
    /// URL of the proposal page
    #[arg(short, long)]
    url: String,

    /// Output OG Image to current directory
    #[arg(short, long)]
    output_og_image: Option<bool>,
}

fn main() {
    let args = Args::parse();

    let dto: GetProposalDataDto = middleware_for_get_proposal_data(&args);

    get_proposal_data_controller(&dto);
}
