use std::path::PathBuf;

use clap::Parser;


#[derive(Parser)]
pub struct Cli {
	source: PathBuf,
	target: PathBuf,
	settings: Option<Vec<String>>
}