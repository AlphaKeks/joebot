use {clap::Parser, std::path::PathBuf};

#[derive(Debug, Parser)]
pub struct Args {
	/// Print debug logs
	#[arg(long)]
	pub debug: bool,

	/// Path to the config file
	#[arg(short, long = "config")]
	#[clap(default_value = "./config.toml")]
	pub config_path: PathBuf,
}

impl Args {
	#[tracing::instrument(level = "TRACE", ret)]
	pub fn get() -> Self {
		Self::parse()
	}
}
