use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Post {
    uuid: Uuid,
    title: String,
    text: String,
    media: Option<Media>,
    created_at: DateTime<Utc>,
}

impl Post {
    pub fn new(
        title: String,
        text: String,
        media: Option<Media>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            title,
            text,
            media,
            created_at,
        }
    }
}

#[derive(Serialize)]
pub struct Media {
    element: String,
    media_type: MediaType,
}

impl Media {
    pub fn new(element: String, media_type: MediaType) -> Self {
        Self {
            element,
            media_type,
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize)]
pub enum MediaType {
    Picture,
    Video,
    Audio,
}
