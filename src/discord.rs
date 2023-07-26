use {
	crate::Config,
	color_eyre::{eyre::Context as _, Result},
	poise::{
		async_trait,
		serenity_prelude::{CacheHttp as _, GatewayIntents, Http},
		Command, FrameworkOptions,
	},
	std::{
		fmt::{self, Write},
		sync::Arc,
	},
	tracing::{error, info},
};

pub type Context<'ctx> = poise::Context<'ctx, State, crate::Error>;
pub type Framework = Arc<poise::Framework<State, crate::Error>>;

/// The bot's global state.
pub struct State {
	/// The bot's configuration file.
	pub config: Config,
}

impl State {
	#[tracing::instrument(level = "TRACE", skip(config))]
	pub async fn new(config: Config) -> Result<Self> {
		Ok(Self { config })
	}

	#[tracing::instrument(level = "TRACE", skip(self))]
	pub async fn into_framework(self) -> Result<Framework> {
		let token = &self.config.api_token;
		let intents = GatewayIntents::GUILDS
			| GatewayIntents::GUILD_MEMBERS
			| GatewayIntents::GUILD_MESSAGES
			| GatewayIntents::MESSAGE_CONTENT;

		let options = FrameworkOptions {
			commands: vec![],

			on_error: |error: poise::FrameworkError<'_, crate::State, crate::Error>| {
				Box::pin(async move {
					error!(?error, "Oh no");

					let Some(ctx) = error.ctx() else {
						error!("no ctx");
						return;
					};

					let Some(channel) = ctx.data().config.error_channel else {
						error!("no error channel");
						return;
					};

					let message = format!("{error:#?}");
					if let Err(error) = channel.say(ctx.http(), message).await {
						error!(?error, "Failed to send error report");
					}
				})
			},

			event_handler: |ctx, event, framework_ctx, state| {
				Box::pin(crate::EventHandler::handle(ctx, event, framework_ctx, state))
			},

			owners: self.config.owners.clone(),
			..Default::default()
		};

		poise::Framework::builder()
			.token(token)
			.intents(intents)
			.options(options)
			.setup(|ctx, ready, framework| {
				Box::pin(async move {
					println!("Logged into Discord as {}.\n", ready.user.tag());

					let commands = &framework.options().commands;

					if let Some(guild_id) = self.config.run_locally {
						poise::builtins::register_in_guild(&ctx.http, commands, guild_id)
							.await
							.context("Failed to register commands locally")?;
					} else {
						poise::builtins::register_globally(&ctx.http, commands)
							.await
							.context("Failed to register commands globally")?;
					}

					if !commands.is_empty() {
						let mut message = String::from("Commands:");

						for Command { name, .. } in commands {
							write!(&mut message, "\n  • /{name}").expect("This never fails");
						}

						println!("{message}\n");
					}

					self.joeback(ctx.http()).await?;

					Ok(self)
				})
			})
			.build()
			.await
			.context("Failed to build framework")
	}

	#[tracing::instrument(level = "TRACE", skip(self, http))]
	pub async fn joeback(&self, http: impl AsRef<Http>) -> Result<()> {
		self.config
			.joeback_channel
			.say(http.as_ref(), "we are joe back")
			.await
			.context("Failed to announce how joeback we are")?;

		info!("we are joe back");

		Ok(())
	}
}

impl fmt::Debug for State {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "State")
	}
}

/// Extension trait to access [`State`] fields on [`Context`] directly.
#[async_trait]
pub trait StateExt {
	fn config(&self) -> &Config;
	fn icon(&self) -> &String;

	fn color(&self) -> (u8, u8, u8) {
		(116, 128, 194)
	}

	fn command_info(&self) -> CommandInfo;
	async fn defer_message(&self) -> Result<()>;
	async fn reply(&self, message: impl Into<String> + Send) -> Result<()>;
}

#[async_trait]
impl StateExt for Context<'_> {
	fn config(&self) -> &Config {
		&self.data().config
	}

	fn icon(&self) -> &String {
		&self.data().config.icon
	}

	fn command_info(&self) -> CommandInfo {
		let command = self.command().name.clone();
		let user = self.author().tag();
		let channel_id = self.channel_id().0;
		let server_name = self
			.guild()
			.map_or_else(|| String::from("unknown"), |guild| guild.name);

		let server_id = self.guild_id().map(|id| id.0);

		CommandInfo { command, user, channel_id, server_name, server_id }
	}

	async fn defer_message(&self) -> Result<()> {
		self.defer()
			.await
			.context("Failed to defer message")
	}

	async fn reply(&self, message: impl Into<String> + Send) -> Result<()> {
		self.say(message)
			.await
			.context("Failed to send message")?;
		Ok(())
	}
}

#[derive(Clone)]
pub struct CommandInfo {
	pub command: String,
	pub user: String,
	pub channel_id: u64,
	pub server_name: String,
	pub server_id: Option<u64>,
}

impl fmt::Debug for CommandInfo {
	/// This implementation is specialized to this crate's [`tracing-subscriber`] setup.
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let CommandInfo { command, user, channel_id, server_name, server_id } = self;
		write!(
			f,
			"CommandInfo {{\n\tcommand: {command:?}\n\tuser: {user:?}\n\tchannel_id: {channel_id:?}\n\tserver_name: {server_name:?}\n\tserver_id: {server_id:?}\n  }}"
		)
	}
}
