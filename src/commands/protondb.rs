use futures::{future};
use reqwest::Client;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use tracing::{error};

#[command]
async fn protondb(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut message = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| e.title("Querying games list..."))
        })
        .await
        .unwrap();

    let proton_data = reqwest::get("https://protondb.max-p.me/games/")
        .await?
        .text()
        .await?;
    let name = args.rest();

    let client = Client::new();

    let re1 = regex::Regex::new(
        format!(
            r#"(?i)\{{"appid":"(\d*)","title":"([^\{{\}}]*{}.*?)"\}}"#,
            name
        )
        .as_str(),
    )
    .unwrap();
    let re2 = regex::Regex::new(r#""timestamp":"([^"]*)","rating":"(.*?)""#).unwrap();

    message
        .edit(&ctx.http, |m| m.embed(|e| e.title("Querying scores...")))
        .await
        .unwrap();

    let mut scores = std::vec::Vec::<&str>::new();
    let caps = re1.captures_iter(&proton_data);
    let bodies = future::join_all(caps.map(|cap| {
        let client = &client;
        async move {
            let url = format!(
                "https://protondb.max-p.me/games/{}/reports",
                cap.get(1).unwrap().as_str(),
            );
            let resp = client.get(url).send().await.unwrap();
            resp.text().await
        }
    }))
    .await;

    for b in bodies {
        match b {
            Ok(full_score) => {
                let mut all_scores = [0, 0, 0, 0, 0];
                for score in re2.captures_iter(full_score.as_str()) {
                    match score.get(2).unwrap().as_str() {
                        "Borked" => all_scores[0] += 1,
                        "Bronze" => all_scores[1] += 1,
                        "Silver" => all_scores[2] += 1,
                        "Gold" => all_scores[3] += 1,
                        "Platinum" => all_scores[4] += 1,
                        _ => (),
                    }
                }
                let mut largest = (0, 0);
                for (i, j) in all_scores.clone().iter().enumerate() {
                    if j >= &largest.0 {
                        largest.0 = j.clone();
                        largest.1 = i;
                    };
                }
                scores.push(match largest.1 {
                    0 => "Borked",
                    1 => "Bronze",
                    2 => "Silver",
                    3 => "Gold",
                    4 => "Platinum",
                    _ => "No score",
                });
            }
            Err(e) => error!("{}", e),
        }
    }

    let unicode_regex = regex::Regex::new(r#"\\u([0-9a-f]{4,5})"#).unwrap();

    message
        .edit(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Queried games");
                e.fields({
                    let mut names = vec![];
                    for (i, cap) in re1.captures_iter(&proton_data.as_str()).enumerate() {
                        let name = unicode_regex.replace_all(
                            cap.get(2).unwrap().as_str(),
                            |caps: &regex::Captures| {
                                let num: u32 =
                                    u32::from_str_radix(caps.get(1).unwrap().as_str(), 16).unwrap();
                                let c: char = std::char::from_u32(num).unwrap();
                                c.to_string()
                            },
                        );
                        let score= scores[i];
                        names.push((name.to_string(), score, true));
                    }
                    names
                })
            })
        })
        .await
        .unwrap();

    Ok(())
}
