use twilight_model::voice::VoiceState;

use crate::{tts::tts, StateRef};

pub async fn member_join(state: &StateRef, voice_state: VoiceState) -> anyhow::Result<()> {
    if voice_state.channel_id.is_some() {
        if let Some(manager) = state.songbird.get(voice_state.guild_id.unwrap()) {
            let user = voice_state.member.unwrap().user;
            if user.bot {
                return Ok(());
            }
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
    Ok(())
}
