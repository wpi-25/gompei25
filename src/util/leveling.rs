use chrono::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct LevelData {
    pub msg_count: u32,
    pub xp: u32,
    pub level: u32,
    pub last_msg: chrono::DateTime<Utc>,
}

fn get_level_number(xp: u32) -> u32 {
    let xp = xp as f64;
    let mut level = (xp / 50f64).powf(1f64/3f64).floor();

    if level < 0f64 {
        level = 0f64;
    }

    level as u32
}

fn get_level_cost(level: u64) -> u64 {
    (level.pow(3) * 50).into()
}

pub async fn get_user_level(user_id: u64, redis_conn: &mut redis::Connection) -> Result<LevelData, Box<dyn std::error::Error>> {
    let msg_count = match redis::cmd("GET").arg(&[format!("{}:count", &user_id)]).query(redis_conn) {
        Ok(count) => count,
        _ => 0
    }; 
    let xp: u32 = match redis::cmd("GET").arg(&[format!("{}:exp", &user_id)]).query(redis_conn) {
        Ok(xp) => xp,
        _ => 0,
    };
    let last_msg_num: i64 = match redis::cmd("GET").arg(&[format!("{}:last", &user_id)]).query(redis_conn) {
        Ok(msg) => msg,
        _ => 0,
    };

    let last_msg = Utc.timestamp(last_msg_num, 0);
    let level = get_level_number(xp.into());

    let level_data = LevelData {
        msg_count,
        xp,
        last_msg,
        level,
    };
        
    Ok(level_data)
}

pub fn set_user_level(user_id: u64, redis_conn: &mut redis::Connection, level_data: LevelData) -> Result<(), Box<dyn std::error::Error>> {
    redis::cmd("SET").arg(&[format!("{}:count", &user_id), level_data.msg_count.to_string()]).execute(redis_conn);
    redis::cmd("SET").arg(&[format!("{}:exp", &user_id), level_data.xp.to_string()]).query(redis_conn)?;
    redis::cmd("SET").arg(&[format!("{}:last", &user_id), level_data.last_msg.timestamp().to_string()]).query(redis_conn)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zero_xp() {
        assert_eq!(super::get_level_number(0), 0)
    }
    #[test]
    fn level_3() {
        assert_eq!(super::get_level_number(2757), 3)
    }
    #[test]
    fn level_14() {
        assert_eq!(get_level_number(149818), 14)
    }

    #[test]
    fn cost_level_3() {
        assert_eq!(get_level_cost(3), 1350)
    }

    #[test]
    fn cost_level_14() {
        assert_eq!(get_level_cost(14), 137200)
    }
}
