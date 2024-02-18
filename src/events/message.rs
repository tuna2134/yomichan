use twilight_model::channel::Message;

use crate::{tts::tts, StateRef};

pub async fn handle_message(state: &StateRef, message: Message) -> anyhow::Result<()> {
    if message.author.bot {
        return Ok(());
    }
    if state
        .channel_ids
        .lock()
        .await
        .values()
        .any(|&id| id == message.channel_id)
    {
        if let Some(manager) = state.songbird.get(message.guild_id.unwrap()) {
            let source = tts(message.content, 1).await?;
            let mut handler = manager.lock().await;
            handler.play_input(source.into());
        }
    }

    Ok(())
}
