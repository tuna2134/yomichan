use songbird::{shards::TwilightMap, Songbird};
use std::{collections::HashMap, env, sync::Arc};
use tokio::sync::Mutex;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{Intents, Shard, ShardId};
use twilight_http::Client as HttpClient;
use twilight_model::id::{
    marker::{ApplicationMarker, ChannelMarker},
    Id,
};

mod applications;
mod events;
mod tts;

pub struct StateRef {
    pub http: HttpClient,
    pub songbird: Songbird,
    pub application_id: Id<ApplicationMarker>,
    pub cache: InMemoryCache,
    pub channel_ids: Mutex<Vec<Id<ChannelMarker>>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    dotenvy::dotenv().ok();
    let token = env::var("DISCORD_TOKEN")?;

    let http = HttpClient::new(token.clone());
    let user = http.current_user().await?.model().await?;
    let application = http.current_user_application().await?.model().await?;

    let intents = Intents::GUILDS
        | Intents::MESSAGE_CONTENT
        | Intents::GUILD_VOICE_STATES
        | Intents::GUILD_MESSAGES;
    let mut shard = Shard::new(ShardId::ONE, token, intents);
    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::VOICE_STATE)
        .build();
    let twilight_map = Arc::new(TwilightMap::new(HashMap::from([(
        shard.id().number(),
        shard.sender(),
    )])));
    let songbird = Songbird::twilight(twilight_map, user.id);
    let state_ref = Arc::new(StateRef {
        http,
        songbird,
        application_id: application.id,
        cache,
        channel_ids: Mutex::new(Vec::new()),
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
        state_ref.cache.update(&event);
        state_ref.songbird.process(&event).await;

        tokio::spawn(events::handle_event(Arc::clone(&state_ref), event));
    }
    Ok(())
}
