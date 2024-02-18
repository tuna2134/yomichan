use twilight_model::voice::VoiceState;

use crate::{tts::tts, StateRef};

pub async fn member_join(state: &StateRef, voice_state: VoiceState) -> anyhow::Result<()> {
    if let Some(channel_id) = voice_state.channel_id {
        if state
            .channel_ids
            .lock()
            .await
            .values()
            .any(|&id| id == channel_id)
        {
            if let Some(manager) = state.songbird.get(voice_state.guild_id.unwrap()) {
                let user = voice_state.member.unwrap().user;
                let src = tts(
                    format!(
                        "{}さんが参加しました",
                        user.global_name.unwrap_or(user.name)
                    ),
                    1,
                )
                .await?;
                let mut handler = manager.lock().await;
                handler.play_input(src.into());
            }
        }
    }
    Ok(())
}
