use std::path::PathBuf;

use clap::Parser;


#[derive(Parser)]
pub struct Cli {
	pub source: PathBuf,
	pub target: PathBuf,
	pub flags: Option<Vec<String>>
}