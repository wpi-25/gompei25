use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Removes the specified number of messages from a channel"]
#[num_args(1)]
#[required_permissions("MANAGE_MESSAGES")]
#[usage("<number of messages>")]
pub async fn clear(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let amount: u64 = args.parse::<u64>()?;
    if amount > 100 {
        msg.channel_id
            .say(&ctx, "Cannot delete more than 100 messages")
            .await?;
        return Ok(());
    }

    let messages = msg
        .channel_id
        .messages(&ctx, |r| r.before(msg.id).limit(amount))
        .await?;

    msg.channel_id.delete_messages(&ctx, messages).await?;
    let confirmation_msg = msg
        .channel_id
        .say(&ctx, format!("Deleted {} messages", amount))
        .await?;
    msg.delete(&ctx).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    confirmation_msg.delete(&ctx).await?;
    Ok(())
}

#[command]
#[description = "Sends a message as the bot"]
#[min_args(2)]
#[required_permissions("MANAGE_MESSAGES")]
#[usage("<#channel> <message>")]
pub async fn sendmsg(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let target: ChannelId = args.parse::<ChannelId>()?;
    args.advance();
    let message: String = args.rest().into();

    target.say(&ctx, message).await?;

    msg.react(&ctx, '✅').await?;

    Ok(())
}
