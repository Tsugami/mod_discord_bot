use std::num::ParseIntError;

use poise::serenity_prelude::UserId;
use sqlx::types::time::PrimitiveDateTime;

enum InteractionCustomId {
    HomePage(UserId),
    NextPage(UserId, PrimitiveDateTime),
    PreviousPage(UserId, PrimitiveDateTime),
}

impl InteractionCustomId {
    pub fn to_string(self) -> String {
        match self {
            Self::HomePage(user_id) => format!("home_p:{}", user_id.to_string()),
            Self::NextPage(user_id, after) => format!(
                "next_p:{}:{}",
                user_id.to_string(),
                after.assume_utc().unix_timestamp()
            ),
            Self::PreviousPage(user_id, after) => format!(
                "previous_p:{}:{}",
                user_id.to_string(),
                after.assume_utc().unix_timestamp()
            ),
        }
    }

    pub fn from_str(str: String) -> Option<Self> {
        let split = str.split(":");
        let vec: Vec<&str> = split.collect();

        if let Some(first) = vec.first() {
            return match *first {
                "home_p" => {
                    if let Ok(user_id) = vec[1].parse::<u64>() {
                        let user_id = UserId(user_id);

                        Some(Self::HomePage(user_id))
                    } else {
                        None
                    }
                }
                "next_p" => {
                    if let Ok(user_id) = vec[1].parse::<u64>() {
                        let user_id = UserId(user_id);

                        Some(Self::HomePage(user_id))
                    } else {
                        None
                    }
                }
                "previous_p" => {
                    if let Ok(user_id) = vec[1].parse::<u64>() {
                        let user_id = UserId(user_id);

                        Some(Self::HomePage(user_id))
                    } else {
                        None
                    }
                }
                _ => None,
            };
        }

        None
    }
}
