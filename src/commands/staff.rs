use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::ArgumentConvert;

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

#[command]
#[description = "Edits a message sent by the bot"]
#[min_args(2)]
#[required_permissions("MANAGE_MESSAGES")]
#[usage("<message link> <new content>")]
pub async fn editmsg(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let target_msg_link = args.single::<String>().unwrap_or(String::from("https://discord.com/channels/000000000000000000/000000000000000000/000000000000000000"));
    let (_, target_msg_link_no_https) = target_msg_link.split_at(8);

    let new_content = args.rest();

    let link_components: Vec<&str> = target_msg_link_no_https.split('/').collect();
    let channel_id: String = link_components[3].to_string();
    let channel_id: u64 = channel_id.parse::<u64>().unwrap_or(0u64);
    let message_id: String = link_components[4].to_string();
    let message_id: u64 = message_id.parse::<u64>().unwrap_or(0u64);

    let mut message_edit: Message = ctx.http.get_message(channel_id, message_id).await?;

    let current_user = ctx.http.get_current_user().await?;

    if message_edit.author != current_user.into() {
        msg.channel_id.say(&ctx, "Cannot edit another user's message.").await?;
    } else {
        message_edit.edit(&ctx, |m| m.content(new_content)).await?;
    }
    Ok(())
}

#[command]
#[description = "Reacts to a message"]
#[num_args(2)]
#[required_permissions("MANAGE_MESSAGES")]
#[usage("<message link> <reaction>")]
pub async fn reactmsg(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
     let target_msg_link = args.single::<String>().unwrap_or(String::from("https://discord.com/channels/000000000000000000/000000000000000000/000000000000000000"));
    let (_, target_msg_link_no_https) = target_msg_link.split_at(8);

    let link_components: Vec<&str> = target_msg_link_no_https.split('/').collect();
    let channel_id: String = link_components[3].to_string();
    let channel_id: u64 = channel_id.parse::<u64>().unwrap_or(0u64);
    let message_id: String = link_components[4].to_string();
    let message_id: u64 = message_id.parse::<u64>().unwrap_or(0u64);

    let reaction = Emoji::convert(ctx, None, Some(channel_id.into()), &args.single::<String>()?).await?;

    msg.channel_id.say(&ctx, format!("Emoji: {:?}", reaction)).await?;

    Ok(())
}
