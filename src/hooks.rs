use serenity::client::Context;
use serenity::framework::standard::macros::hook;
use serenity::framework::standard::{DispatchError, Reason, CommandError};
use serenity::model::channel::Message;
use tracing::error;

#[hook]
pub async fn dispatch_error_hook(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::NotEnoughArguments { min, given } => {
            let mut arguments = String::from("arguments");
            if min == 1 {
                arguments = String::from("argument");
            }
            msg.channel_id
                .say(
                    &ctx,
                    format!("Need {} {}, only got {}!", min, arguments, given),
                )
                .await.unwrap();
        }
        DispatchError::TooManyArguments { max, given } => {
            let mut arguments = String::from("arguments");
            if max == 1 {
                arguments = String::from("argument");
            }
            msg.channel_id
                .say(&ctx, format!("Need {} {}, got {}!", max, arguments, given))
                .await.unwrap();
        }
        DispatchError::CheckFailed(_, reason) => {
            match reason {
                Reason::User(details) => {
                    msg.channel_id
                        .say(&ctx, format!("Permission denied: {}", details))
                        .await.unwrap();
                }
                Reason::UserAndLog { user, log } => {
                    msg.channel_id
                        .say(&ctx, format!("Permission denied: {}", user))
                        .await.unwrap();
                }
                _ => (),
            };
        }
        DispatchError::Ratelimited(info) => {
            msg.channel_id
                .say(
                    &ctx,
                    format!(
                        "Please wait {} seconds before trying again",
                        info.rate_limit.as_secs()
                    ),
                )
                .await.unwrap();
        }
        DispatchError::CommandDisabled(_) => {
            msg.channel_id
                .say(&ctx, "That command has been disabled.")
                .await.unwrap();
        }
        DispatchError::BlockedUser => {
            msg.channel_id
                .say(&ctx, "You have been banned from the bot.")
                .await.unwrap();
        }
        DispatchError::BlockedGuild => {
            msg.channel_id
                .say(&ctx, "This server has been banned from the bot.")
                .await.unwrap();
        }
        DispatchError::BlockedChannel => {
            msg.channel_id
                .say(&ctx, "This channel has been banned from the bot.")
                .await.unwrap();
        }
        DispatchError::OnlyForDM => {
            if let Ok(pm) = msg.author.create_dm_channel(&ctx).await {
                pm.say(&ctx, "That command can only be used in PM!").await.unwrap();
            } else {
                msg.channel_id
                    .say(&ctx, "That command can only be used in PM!")
                    .await.unwrap();
            }
        }
        DispatchError::OnlyForGuilds => {
            msg.channel_id
                .say(&ctx, "That command can only be used in a server")
                .await.unwrap();
        }
        DispatchError::OnlyForOwners => {
            msg.channel_id
                .say(&ctx, "Permission denied: Bot owners only")
                .await.unwrap();
        }
        DispatchError::LackingRole => {
            msg.channel_id
                .say(&ctx, "Permission denied: You don't have the right role!")
                .await.unwrap();
        }
        DispatchError::LackingPermissions(_) => {
            msg.channel_id
                .say(
                    &ctx,
                    "Permission denied: You don't have enough permissions!",
                )
                .await.unwrap();
        }
        _ => {
            msg.channel_id.say(&ctx, "Unhandled dispatch error").await.unwrap();
        }
    };
}

#[hook]
pub async fn after_cmd(_: &Context, _: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    if let Err(err) = error {
        error!("Error in {}: {}", cmd_name, err);
    }
}
