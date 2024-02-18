use twilight_model::channel::Message;

use crate::{tts::tts, StateRef};

pub async fn handle_message(state: &StateRef, message: Message) -> anyhow::Result<()> {
    if state.channel_ids.lock().await.contains(&message.channel_id) {
        let audio = tts(message.content.clone(), 1).await?;
        if let Some(manager) = state.songbird.get(message.guild_id.unwrap()) {
            let source = songbird::input::cached::Compressed::new(
                audio.into(),
                songbird::driver::Bitrate::Auto,
            )
            .await?;
            let _ = source.raw.spawn_loader();
            let mut handler = manager.lock().await;
            handler.play_input(source.into());
        }
    }

    Ok(())
}
