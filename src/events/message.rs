use twilight_model::channel::Message;

use crate::{tts::tts, StateRef};

pub async fn handle_message(state: &StateRef, message: Message) -> anyhow::Result<()> {
    if state.channel_ids.lock().await.contains(&message.channel_id) {
        if let Some(manager) = state.songbird.get(message.guild_id.unwrap()) {
            let source = tts(message.content, 1).await?;
            let _ = source.raw.spawn_loader();
            let mut handler = manager.lock().await;
            handler.play_input(source.into());
        }
    }

    Ok(())
}
