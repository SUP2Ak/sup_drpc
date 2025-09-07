//! Crate pour la gestion Discord Rich Presence
//!
//! Ce crate fournit une interface simple et robuste pour gérer les connexions
//! Discord Rich Presence avec gestion d'erreurs et reconnexion automatique.

// Modules
mod models;
mod discord_ipc;
mod manager;

// Réexporter SEULEMENT les types publics nécessaires
pub use models::{
    ActivityType,
    RichPresenceConfig,
    RichPresenceAssets,
    RichPresenceTimestamps,
    RichPresenceButton,
    RichPresenceParty,
    RichPresenceSecrets,
};
pub use manager::RichPresenceManager;