//! Group of "fun" commands
//!
//! Commands that don't really have a practical use but are still fun to have
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
struct XkcdResponse {
    num: u16,
    safe_title: String,
    alt: String,
    img: String,
}

/// Fetches an XKCD comic via the JSON API
///
/// Args: either nothing, or the number of the XKCD
#[command]
#[description = "Gets an XKCD comic"]
#[only_in(guilds)]
#[max_args(1)]
pub async fn xkcd(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut response_msg = msg.channel_id.say(&ctx, "Fetching XKCD").await?;
    let mut url = String::from("https://xkcd.com/");

    if args.len() == 0 {
        url.push_str("info.0.json");
    } else if args.len() == 1 {
        url.push_str(&format!("{}/info.0.json", args.current().unwrap_or("")));
    }

    let res = reqwest::get(url).await?.json::<XkcdResponse>().await?;

    response_msg.edit(&ctx.http, |m| {
        m.content("");
        m.embed(|e| {
            e.title(format!("#{} ({})", res.num, res.safe_title));
            e.image(res.img.clone());
            e.field("Image URL", res.img, true);
            e.field("Alt text", res.alt, true);

            e
        });
        m
    }).await?;

    Ok(())
}
