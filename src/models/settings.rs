use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Settings {
    user_uuid: Uuid,
    theme: String,
    language: String,
    notifications: Notifications,
}

impl Settings {
    pub fn new(
        user_uuid: Uuid,
        theme: String,
        language: String,
        notifications: Notifications,
    ) -> Self {
        Self {
            user_uuid,
            theme,
            language,
            notifications,
        }
    }
}

#[derive(Serialize)]
pub struct Notifications {
    email: bool,
    push: bool,
}

impl Notifications {
    pub fn new(email: bool, push: bool) -> Self {
        Self { email, push }
    }
}
