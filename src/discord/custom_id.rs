use poise::serenity_prelude::UserId;

pub enum InteractionCustomId {
    Page { user_id: UserId, page: i64 },
}

impl InteractionCustomId {
    pub fn to_string(self, idx: &str) -> String {
        match self {
            Self::Page { user_id, page } => format!("page:{}:{}:{}", user_id, page, idx),
        }
    }

    pub fn from_str(str: String) -> Option<Self> {
        let split = str.split(":");
        let vec: Vec<&str> = split.collect();

        if let Some(first) = vec.first() {
            return match *first {
                "page" => {
                    let user_id = UserId(vec[1].parse::<u64>().ok()?);
                    let page = vec[2].parse::<i64>().ok()?;

                    Some(Self::Page { user_id, page })
                }

                _ => None,
            };
        }

        None
    }
}
