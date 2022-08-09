use chrono::Utc;
use now::DateTimeNow;
use serenity::http::Http;
use futures::TryStreamExt;
use serenity::json::{JsonMap, json};
use reqwest::Client as ReqwestClient;
use twitch_api2::{helix, HelixClient};
use serenity::model::guild::ScheduledEventType::External;
use twitch_api2::twitch_oauth2::{AppAccessToken, scopes::Scope};
use twitch_api2::twitch_oauth2::types::{ClientId, ClientSecret};


async fn get_week_schedule() -> Result<Vec<helix::schedule::Segment>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let client: HelixClient<ReqwestClient> =  HelixClient::default();
    let client_id = std::env::var("TWITCH_CLIENT_ID")
    .ok()
    .map(ClientId::new)
    .expect("Please set env: TWITCH_CLIENT_ID");
    let client_secret = std::env::var("TWITCH_CLIENT_SECRET")
    .ok()
    .map(ClientSecret::new)
    .expect("Please set env: TWITCH_CLIENT_SECRET");
    let token = AppAccessToken::get_app_access_token(
        &client.clone_client(), client_id, client_secret, Scope::all()
    ).await?;

    let end_of_week = Utc::now().end_of_week().format("%Y-%m-%d").to_string();
    let broadcaster_id = std::env::var("TWITCH_BROADCASTER_ID")
    .ok()
    .expect("Please set env: TWITCH_BROADCASTER_ID");
    let schedule: Vec<helix::schedule::Segment> = client
    .get_channel_schedule(broadcaster_id, &token)
    .try_take_while(|s| {
        futures::future::ready(
            Ok(s.start_time.to_string() < end_of_week)
        )
    })
    .try_collect()
    .await?;

    Ok(schedule)
}


async fn create_events(events: Vec<helix::schedule::Segment>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let discord_token = std::env::var("DISCORD_TOKEN")
    .ok()
    .expect("Please set env: DISCORD_TOKEN");
    let discord_guild_id = std::env::var("DISCORD_GUILD_ID")
    .ok()
    .expect("Please set env: DISCORD_GUILD_ID");
    let discord_event_location = std::env::var("DISCORD_EVENT_LOCATION")
    .ok()
    .expect("Please set env: DISCORD_EVENT_LOCATION");
    let http = Http::new(&discord_token);
    let mut map = JsonMap::new();

    map.insert("guild_id".to_string(), json!(discord_guild_id));
    map.insert("entity_type".to_string(), json!(External));
    map.insert("privacy_level".to_string(), json!(2));
    map.insert("entity_metadata".to_string(), json!({
        "location": discord_event_location
    }));

    for _event in &events {
        if _event.canceled_until.is_none() {
            map.insert("name".to_string(), json!(_event.category.as_ref().unwrap().name));
            map.insert("scheduled_start_time".to_string(), json!(_event.start_time.to_string()));
            map.insert("scheduled_end_time".to_string(), json!(_event.end_time.to_string()));

            http.create_scheduled_event(
                discord_guild_id.parse::<u64>().unwrap(),
                &map,
                Some("Adding new event from Twitch")
            ).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + std::marker::Send>> {
    let schedule = get_week_schedule().await;
    let events = match schedule {
        Ok(events) => events,
        Err(e) => panic!("Error: {}", e),
    };

    if let Err(e) = create_events(events).await {
        panic!("Error: {}", e);
    }

    Ok(())
}
