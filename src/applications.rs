use crate::{commands, StateRef};
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
    match interaction.kind {
        InteractionType::ApplicationCommand => {
            let command =
                if let Some(InteractionData::ApplicationCommand(ref command)) = interaction.data {
                    command
                } else {
                    return Ok(());
                };
            match command.name.as_str() {
                "join" => commands::join(state, interaction).await?,
                // "leave" => {}
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}
