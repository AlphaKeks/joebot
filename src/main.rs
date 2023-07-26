use {
	color_eyre::{eyre::Context as _, Result},
	joebot::{Args, Config, State},
	tracing::debug,
};

#[tokio::main]
async fn main() -> Result<()> {
	let args = Args::get();

	if args.debug {
		joebot::tracing::setup();
	}

	let config = Config::load(&args.config_path).context("Failed to load config")?;

	let state = State::new(config)
		.await
		.context("Failed to build state")?;

	debug!("Created state");

	let framework = state
		.into_framework()
		.await
		.context("Failed to build framework")?;

	debug!("Created framework");

	framework
		.start_autosharded()
		.await
		.context("Failed to run framework")?;

	Ok(())
}
