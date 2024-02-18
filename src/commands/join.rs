use twilight_model::{
    application::interaction::Interaction,
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::StateRef;

pub async fn join(state: &StateRef, interaction: Interaction) -> anyhow::Result<()> {
    println!("Joining");
    let channel_id = if let Some(voice_state) = state.cache.voice_state(
        interaction.author_id().unwrap(),
        interaction.guild_id.unwrap(),
    ) {
        voice_state.channel_id()
    } else {
        return Ok(());
    };
    state
        .songbird
        .join(interaction.guild_id.unwrap(), channel_id)
        .await?;
    state.channel_ids.lock().await.insert(
        interaction.guild_id.unwrap(),
        interaction.channel.unwrap().id,
    );
    let interaction_http = state.http.interaction(state.application_id);
    interaction_http
        .create_response(
            interaction.id,
            &interaction.token,
            &InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(
                    InteractionResponseDataBuilder::new()
                        .content("読み上げを開始しました")
                        .build(),
                ),
            },
        )
        .await?;
    Ok(())
}
