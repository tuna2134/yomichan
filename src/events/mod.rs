use crate::StateRef;
use std::sync::Arc;
use twilight_gateway::Event;

mod message;
mod voice_state;

pub async fn handle_event(state: Arc<StateRef>, event: Event) -> anyhow::Result<()> {
    match event {
        Event::Ready(_) => {
            println!("Connected to gateway");
        }
        Event::InteractionCreate(interaction) => {
            crate::applications::handle_interaction(&state, interaction.0).await?;
        }
        Event::MessageCreate(message) => {
            message::handle_message(&state, message.0).await?;
        }
        Event::VoiceStateUpdate(voice_state) => {
            voice_state::member_join(&state, voice_state.0).await?;
        }
        _ => {}
    }
    Ok(())
}
