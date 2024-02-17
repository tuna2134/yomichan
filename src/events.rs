use crate::{tts::tts, StateRef};
use songbird::tracks::Track;
use std::sync::Arc;
use twilight_gateway::Event;

pub async fn handle_event(state: Arc<StateRef>, event: Event) -> anyhow::Result<()> {
    match event {
        Event::Ready(_) => {
            println!("Connected to gateway");
        }
        Event::InteractionCreate(interaction) => {
            crate::applications::handle_interaction(&state, interaction.0).await?;
        }
        Event::MessageCreate(message) => {
            if state.channel_ids.lock().await.contains(&message.channel_id) {
                let audio = tts(message.content.clone()).await?;
                if let Some(manager) = state.songbird.get(message.guild_id.unwrap()) {
                    let mut handler = manager.lock().await;
                    let _ = handler.play(Track::from(audio));
                }
            }
        }
        _ => {}
    }
    Ok(())
}
