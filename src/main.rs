mod strategies;

/// JustCompress, a universal lossless compression tool that prioritizes compression size
///

use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "JustCompress")]
#[command(version, about, long_about = None)]
struct Args {
	file: Option<PathBuf>,
	#[arg(short, long, value_name = "THREAD_AMOUNT")]
	threads: Option<u8>
}

fn main() {
	let args = Args::parse();
	dbg!(args);
}
