use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::vec;

#[command]
async fn embed(ctx: &Context, msg: &Message) -> CommandResult {
   msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("AHA");
                e.description("oooooooooooooooooooooooooo");
                e.fields(vec![
                    ("yup", "yup", true),
                    ("yup", "yup", true),
                ]);
                e.field("wack", "very", true);
                e.footer(|f| {
                    f.text("????????");

                    f
                });

                e
            });
            m
        }).await?;
    Ok(())
}