use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use regex::Regex;


#[command]
async fn protonge(ctx: &Context, msg: &Message) -> CommandResult {
    static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    );

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let proton_info = client.get("https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases/latest").send().await?.text().await?;
    let re1 = Regex::new("\"name\":\"([^\"]*)\"").unwrap();
    msg.channel_id.say(&ctx.http,  re1.captures(proton_info.as_str()).unwrap().get(1).unwrap().as_str()).await?;

    Ok(())
}