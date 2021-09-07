use chrono::Utc;
use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::util::leveling::get_user_level;
use crate::RedisConnection;
use tracing::error;

use std::collections::HashMap;
use std::cmp::Ordering;

#[command]
#[description = "Gets your level and such"]
#[only_in(guilds)]
#[num_args(0)]
pub async fn rank(ctx: &Context, msg: &Message) -> CommandResult {

    let mut bot_data = ctx.data.write().await;
    let mut redis_conn = bot_data.get_mut::<RedisConnection>().unwrap();

    let level_data = match get_user_level(msg.author.id.0, redis_conn) {
        Ok(data) => data,
        Err(e) => {
            msg.channel_id.say(&ctx.http, format!("Could not get your points: {:?}", e)).await;
            return Ok(());
        }
    };
    let avatar_url = match msg.author.avatar_url() {
        Some(url) => url,
        None => msg.author.default_avatar_url(),
    };
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|mut e| {
            e.author(|m| {
                m.icon_url(avatar_url);
                m.name(msg.author.name.clone());
                m
            });
            e.field("Messages", level_data.msg_count.to_string(), true);
            e.field("XP", level_data.xp.to_string(), true);
            e.field("Level", level_data.level.to_string(), true);
            e
        });
        m
    }).await?;

    Ok(())
}

#[derive(Clone, Debug)]
pub struct LeaderboardData {
    pub member: Member,
    pub xp: u32,
    pub level: u32,
    pub msg_count: u32,
}

impl PartialOrd for LeaderboardData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for LeaderboardData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.xp.cmp(&other.xp)
    }
}

impl Eq for LeaderboardData {
    
}
impl PartialEq for LeaderboardData {
    fn eq(&self, other: &Self) -> bool {
        self.xp == other.xp
    }
}

async fn get_ranked_leaderboard(guild: &Guild, redis_conn: &mut redis::Connection) -> Vec<LeaderboardData> {
    let mut leaderboard = Vec::new();

    for (id, member) in guild.members.iter() {
        let level_data = match get_user_level(id.0, redis_conn) {
            Ok(data) => data,
            Err(e) => {
                error!("Error fetching level data: {:?}", e);
                continue;
            }
        };
        leaderboard.push(LeaderboardData {
            member: member.clone(),
            xp: level_data.xp,
            level: level_data.level,
            msg_count: level_data.msg_count,
        });
    }

    leaderboard.sort();

    leaderboard
}

#[command]
#[description = "Checks the server leaderboard"]
#[max_args(1)]
#[usage("[page]")]
pub async fn levels(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    println!("Called levels");
    let mut bot_data = ctx.data.write().await;
    let redis_conn = bot_data.get_mut::<RedisConnection>().unwrap();
    let leaderboard = get_ranked_leaderboard(&msg.guild(&ctx).await.unwrap(), redis_conn).await;
    println!("Got ranked leaderboard");

    let mut PAGE_SIZE = 10;
    if leaderboard.len() < PAGE_SIZE {
        PAGE_SIZE = (leaderboard.len() - 1);
    }

    let page_num: usize = match args.parse::<usize>() {
        Ok(page) => page,
        Err(_) => 1
    };
    println!("Page num: {}", page_num);
    println!("Leaderboard: {:?}", leaderboard);

    let (tmp, tmp2) = leaderboard.split_at(page_num * PAGE_SIZE);
    println!("mid: {}", page_num * PAGE_SIZE);
    println!("Leaderboard slice: {:?}", tmp);
    println!("Second leaderboard slice: {:?}", tmp2);

    let mut page: Vec<LeaderboardData> = Vec::new();

    for (i, l) in tmp.iter().enumerate() {
        if i < PAGE_SIZE as usize {
            page.push(l.clone());
        }
    }

    page.sort();


    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Leaderboard");
            e.description(format!("**Page {}:** {}-{} of {}", page_num.to_string(), PAGE_SIZE * (page_num - 1) + 1, PAGE_SIZE * page_num, leaderboard.len()));

            for (i, l) in page.iter().enumerate() {
                e.field(format!("#{}: {}", i+1, l.member), format!("{} Exp.\tLvl. {}\t{} Messages", l.xp, l.level, l.msg_count), false);
            }

            e.timestamp(Utc::now().to_string());

            e
        });
        m
    }).await?;

    Ok(())
}