use {
	color_eyre::{eyre::Context, Result},
	poise::serenity_prelude::{ChannelId, GuildId, UserId},
	serde::Deserialize,
	std::{collections::HashSet, path::Path},
};

#[derive(Deserialize)]
pub struct Config {
	/// Discord API Token for the Bot's Discord account.
	pub api_token: String,

	/// Only register Discord slash commands locally.
	/// The value of this is the [`GuildId`] of the server that should register the commands.
	pub run_locally: Option<GuildId>,

	/// List of users with special privileges.
	pub owners: HashSet<UserId>,

	/// The default embed icon URL.
	pub icon: String,

	/// Joebot will say "we are joe back" in this channel when going online.
	pub joeback_channel: ChannelId,

	/// h.
	pub h_channel: ChannelId,

	/// A channel to send error reports to.
	pub error_channel: Option<ChannelId>,
}

impl Config {
	pub fn load(path: &Path) -> Result<Self> {
		let config_file = std::fs::read_to_string(path).context("Failed to read config file")?;
		let config = toml::from_str(&config_file).context("Failed to parse config file")?;
		Ok(config)
	}
}
