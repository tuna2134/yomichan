use twilight_model::{
    application::interaction::Interaction,
    http::interaction::{InteractionResponse, InteractionResponseType},
};

use crate::StateRef;

pub async fn leave(state: &StateRef, interaction: Interaction) -> anyhow::Result<()> {
    if let Some(manager) = state.songbird.get(interaction.guild_id.unwrap()) {
        manager.lock().await.leave().await?;
        state
            .channel_ids
            .lock()
            .await
            .remove(&interaction.guild_id.unwrap());
        let interaction_http = state.http.interaction(state.application_id);
        interaction_http
            .create_response(
                interaction.id,
                &interaction.token,
                &InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(
                        twilight_util::builder::InteractionResponseDataBuilder::new()
                            .content("読み上げを終了しました")
                            .build(),
                    ),
                },
            )
            .await?;
    }
    Ok(())
}
