use {
	crate::Result,
	color_eyre::eyre::Context,
	poise::{serenity_prelude as serenity, serenity_prelude::ReactionType},
};

impl super::EventHandler {
	pub(super) async fn message_update(
		old_message: Option<&serenity::Message>,
		new_message: Option<&serenity::Message>,
		ctx: &serenity::Context,
	) -> Result<()> {
		let (Some(old_message), Some(new_message)) = (old_message, new_message) else {
			return Ok(());
		};

		for reaction in old_message
			.reactions
			.iter()
			.filter(|reaction| reaction.me)
		{
			old_message
				.delete_reaction_emoji(&ctx.http, reaction.reaction_type.clone())
				.await
				.context("Failed to remove reactions")?;
		}

		if let Some(reaction) = crate::Emotion::judge(&new_message.content) {
			if let Some(emoji) = Option::<ReactionType>::from(&reaction) {
				new_message
					.react(&ctx.http, emoji)
					.await
					.context("Failed to react to message")?;
			}
		}

		Ok(())
	}
}
