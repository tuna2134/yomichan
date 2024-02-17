use songbird::{shards::TwilightMap, Songbird};
use std::{collections::HashMap, env, sync::Arc};
use twilight_gateway::{Intents, Shard, ShardId};
use twilight_http::Client as HttpClient;
use twilight_model::id::{marker::ApplicationMarker, Id};
mod applications;
mod events;

pub struct StateRef {
    pub http: HttpClient,
    pub songbird: Songbird,
    pub application_id: Id<ApplicationMarker>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let token = env::var("DISCORD_TOKEN")?;

    let http = HttpClient::new(token.clone());
    let user = http.current_user().await?.model().await?;
    let application = http.current_user_application().await?.model().await?;

    let intents = Intents::GUILDS | Intents::MESSAGE_CONTENT | Intents::GUILD_VOICE_STATES;
    let mut shard = Shard::new(ShardId::ONE, token, intents);
    let twilight_map = Arc::new(TwilightMap::new(HashMap::from([(
        shard.id().number(),
        shard.sender(),
    )])));
    let songbird = Songbird::twilight(twilight_map, user.id);
    let state_ref = Arc::new(StateRef {
        http,
        songbird,
        application_id: application.id,
    });
    applications::set_application_command(&state_ref).await?;

    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(err) => {
                println!("Error: {:?}", err);
                if err.is_fatal() {
                    break;
                }
                continue;
            }
        };

        state_ref.songbird.process(&event).await;
        tokio::spawn(events::handle_event(Arc::clone(&state_ref), event));
    }
    Ok(())
}
