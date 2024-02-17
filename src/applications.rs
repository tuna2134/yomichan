use crate::StateRef;
use twilight_model::application::{
    command::CommandType,
    interaction::{Interaction, InteractionData, InteractionType},
};
use twilight_util::builder::command::CommandBuilder;

pub async fn set_application_command(state: &StateRef) -> anyhow::Result<()> {
    let interaction_http = state.http.interaction(state.application_id);

    let join = CommandBuilder::new("join", "読み上げ開始", CommandType::ChatInput).build();
    interaction_http.set_global_commands(&[join]).await?;
    Ok(())
}

pub async fn handle_interaction(state: &StateRef, interaction: Interaction) -> anyhow::Result<()> {
    let interaction_http = state.http.interaction(state.application_id);
    match interaction.kind {
        InteractionType::ApplicationCommand => {
            let command =
                if let Some(InteractionData::ApplicationCommand(command)) = interaction.data {
                    command
                } else {
                    return Ok(());
                };
            println!("{:?}", command.name);
            match command.name.as_str() {
                "join" => {
                    state
                        .songbird
                        .join(
                            interaction.guild_id.unwrap(),
                            interaction.channel.unwrap().id,
                        )
                        .await?;
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}
