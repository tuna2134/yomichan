use crate::{commands, StateRef};
use twilight_model::application::{
    command::CommandType,
    interaction::{Interaction, InteractionData, InteractionType},
};
use twilight_util::builder::command::CommandBuilder;

pub async fn set_application_command(state: &StateRef) -> anyhow::Result<()> {
    let interaction_http = state.http.interaction(state.application_id);

    let join = CommandBuilder::new("join", "読み上げ開始", CommandType::ChatInput).build();
    let leave = CommandBuilder::new("leave", "読み上げ終了", CommandType::ChatInput).build();
    interaction_http.set_global_commands(&[join, leave]).await?;
    Ok(())
}

pub async fn handle_interaction(state: &StateRef, interaction: Interaction) -> anyhow::Result<()> {
    if interaction.kind == InteractionType::ApplicationCommand {
        let command =
            if let Some(InteractionData::ApplicationCommand(ref command)) = interaction.data {
                command
            } else {
                return Ok(());
            };
        match command.name.as_str() {
            "join" => commands::join(state, interaction).await?,
            "leave" => commands::leave(state, interaction).await?,
            _ => {}
        }
    }
    Ok(())
}
