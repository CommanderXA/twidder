use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Settings {
    user_uuid: Uuid,
    theme: Theme,
    language: Language,
    notifications: Notifications,
}

impl Settings {
    pub fn new(
        user_uuid: Uuid,
        theme: Theme,
        language: Language,
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
pub enum Theme {
    Auto,
    Light,
    Dark,
}

#[derive(Serialize)]
pub enum Language {
    English,
    Russian,
    French,
    German,
    Spanish,
}

impl Language {
    pub fn to_code(&self) -> String {
        match &self {
            self::Language::English => "en".to_owned(),
            self::Language::Russian => "ru".to_owned(),
            self::Language::French => "fr".to_owned(),
            self::Language::German => "de".to_owned(),
            self::Language::Spanish => "es".to_owned(),
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
