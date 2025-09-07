//! Modèles de données pour Discord Rich Presence

use serde::{Deserialize, Serialize};
use discord_rich_presence::activity::ActivityType as DiscordActivityType;

/// Type d'activité Discord
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ActivityType {
    Playing,
    Listening,
    Watching,
    Competing,
}

impl From<ActivityType> for DiscordActivityType {
    fn from(activity_type: ActivityType) -> Self {
        match activity_type {
            ActivityType::Playing => DiscordActivityType::Playing,
            ActivityType::Listening => DiscordActivityType::Listening,
            ActivityType::Watching => DiscordActivityType::Watching,
            ActivityType::Competing => DiscordActivityType::Competing,
        }
    }
}

/// Configuration complète pour Discord Rich Presence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichPresenceConfig {
    pub application_id: Option<String>,
    pub activity_type: ActivityType,
    pub details: Option<String>,
    pub state: Option<String>,
    pub assets: Option<RichPresenceAssets>,
    pub timestamps: Option<RichPresenceTimestamps>,
    pub buttons: Option<Vec<RichPresenceButton>>,
    pub party: Option<RichPresenceParty>,
    pub secrets: Option<RichPresenceSecrets>,
}

/// Assets pour Discord Rich Presence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichPresenceAssets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

/// Timestamps pour Discord Rich Presence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichPresenceTimestamps {
    pub start: Option<i64>,
    pub end: Option<i64>,
}

/// Bouton pour Discord Rich Presence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichPresenceButton {
    pub label: String,
    pub url: String,
}

/// Informations de party pour Discord Rich Presence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichPresenceParty {
    pub id: Option<String>,
    pub size: Option<[i32; 2]>,
}

/// Secrets pour Discord Rich Presence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichPresenceSecrets {
    pub join: Option<String>,
    pub spectate: Option<String>,
    pub match_field: Option<String>,
}
