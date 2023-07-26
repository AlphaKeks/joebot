use {crate::Result, poise::serenity_prelude as serenity, tracing::trace};

mod message_create;
mod message_update;

pub struct EventHandler;

impl EventHandler {
	pub async fn handle(
		ctx: &serenity::Context,
		event: &poise::Event<'_>,
		_framework_ctx: poise::FrameworkContext<'_, crate::State, crate::Error>,
		state: &crate::State,
	) -> Result<()> {
		trace!(event = %event.name(), "Something happened!");

		match event {
			poise::Event::Message { new_message } => {
				Self::message_create(new_message, ctx, state).await?;
			}

			poise::Event::MessageUpdate { old_if_available, new, .. } => {
				Self::message_update(old_if_available.as_ref(), new.as_ref(), ctx).await?
			}

			_ => {}
		}

		Ok(())
	}
}
