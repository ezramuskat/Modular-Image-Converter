use std::path::PathBuf;

use clap::Parser;


#[derive(Parser)]
pub struct Cli {
	pub source: PathBuf,
	pub target: PathBuf,
	pub settings: Option<Vec<String>>
}