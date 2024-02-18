use twilight_model::application::interaction::Interaction;

use crate::StateRef;

pub async fn leave(state: &StateRef, interaction: Interaction) -> anyhow::Result<()> {
    if let Some(manager) = state.songbird.get(interaction.guild_id.unwrap()) {
        manager.lock().await.leave().await?;
    }
    Ok(())
}
