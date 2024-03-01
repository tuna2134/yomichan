use twilight_model::channel::Message;

use crate::{tts::tts, StateRef};

pub async fn handle_message(state: &StateRef, message: Message) -> anyhow::Result<()> {
    if message.author.bot {
        return Ok(());
    }
    let content = if message.content.len() > 100 {
        format!(
            "{}、以下省略",
            &message.content.chars().take(100).collect::<String>()
        )
    } else {
        message.content
    };
    if state
        .channel_ids
        .lock()
        .await
        .values()
        .any(|&id| id == message.channel_id)
    {
        if let Some(manager) = state.songbird.get(message.guild_id.unwrap()) {
            let source = tts(content, 1).await?;
            let mut handler = manager.lock().await;
            handler.play_input(source.into());
        }
    }

    Ok(())
}
