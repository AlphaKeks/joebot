use {
	crate::Result,
	color_eyre::eyre::Context as _,
	poise::{serenity_prelude as serenity, serenity_prelude::ReactionType},
};

impl super::EventHandler {
	pub(super) async fn message_create(
		message: &serenity::Message,
		ctx: &serenity::Context,
		state: &crate::State,
	) -> Result<()> {
		// Ignore bots
		if message.author.bot {
			return Ok(());
		}

		// Check for h thread
		if message.channel_id == state.config.h_channel {
			return Self::send_h(ctx, state).await;
		}

		// Check for prompt
		if let Some(response) = crate::PROMPTS.get(&message.content) {
			message
				.reply(&ctx.http, response)
				.await
				.context("Failed to reply to prompt")?;
		}

		// Only react to 25% of messages
		if !(rand::random::<bool>() && rand::random::<bool>()) {
			return Ok(());
		}

		// Check if we need to react
		if let Some(reaction) = crate::Emotion::judge(&message.content) {
			if let Some(emoji) = Option::<ReactionType>::from(&reaction) {
				message
					.react(&ctx.http, emoji)
					.await
					.context("Failed to react to message")?;
			}
		}

		// Check if we have a copy pasta
		if let Some(prompt) = message
			.content
			.split(' ')
			.find_map(|word| crate::COPY_PASTAS.get(word))
		{
			message
				.reply(&ctx.http, prompt)
				.await
				.context("Failed to reply with copy pasta")?;
		}

		Ok(())
	}

	async fn send_h(ctx: &serenity::Context, state: &crate::State) -> Result<()> {
		state
			.config
			.h_channel
			.say(&ctx.http, "h")
			.await
			.context("Failed to h")?;

		Ok(())
	}
}
