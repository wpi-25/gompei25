use derive_more::Display;

#[derive(Display)]
pub enum GompeiError {
    #[display(fmt = "Command Error: {}", _0)]
    CommandError(String),

    #[display(fmt = "Database Error")]
    DatabaseError,

    #[display(fmt = "Bot error: {}", _0)]
    SerenityError(String),

    #[display(fmt = "Unknown Error: {}", _0)]
    GenericError(String),
}

impl From<Box<dyn std::error::Error>> for GompeiError {
    fn from(e: Box<dyn std::error::Error>) -> GompeiError {
        GompeiError::GenericError(e.to_string())
    }
}

use serenity::Error as BotLibErr;
impl From<serenity::Error> for GompeiError {
    fn from(e: serenity::Error) -> GompeiError {
        match e {
            BotLibErr::Gateway(e) => GompeiError::SerenityError(format!("Gateway Error: {:?}", e)),
            BotLibErr::Model(e) => GompeiError::SerenityError(format!("Model Error: {:?}", e)),
            _ => GompeiError::SerenityError(format!("Unknown Error.")),
        }
    }
}

impl From<redis::RedisError> for GompeiError {
    fn from(_: redis::RedisError) -> GompeiError {
        GompeiError::DatabaseError
    }
}
