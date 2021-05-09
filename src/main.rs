use std::env;

extern crate chrono;
use chrono::offset::{Utc,Local};
use chrono::DateTime;

use std::time::SystemTime;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "v;ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
        if msg.content == "v;time" {
            let datetime: DateTime<Local> = SystemTime::now().into();
            if let Err(why) = msg.channel_id.say(&ctx.http,datetime.format("The time is: %d/%m/%Y %T, DrVilepis is probably asleep")).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // configure the client with your Discord bot token in the environment.
    let token = env::var("VILEBOT_TOKEN")
        .expect("Expected a token in the environment");

    // Create new bot instance
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // catch starting errors
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}