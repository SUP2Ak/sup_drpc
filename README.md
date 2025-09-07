# sup_drpc

Crate Rust optimisé pour la gestion Discord Rich Presence avec une API simple, robuste et performante.

## 🚀 Fonctionnalités

- ✅ **API simple** : Interface intuitive pour Discord Rich Presence
- ✅ **Performance optimisée** : Imports spécifiques, pas de `use *`
- ✅ **Type safety** : Types Rust stricts avec `Copy` trait pour les enums
- ✅ **Gestion d'erreurs** : Utilise `anyhow` pour une gestion robuste
- ✅ **Reconnexion automatique** : Gestion intelligente des déconnexions
- ✅ **Architecture modulaire** : Structure claire et extensible
- ✅ **API contrôlée** : Seuls les types nécessaires sont exposés

## 📁 Structure

Ce crate est organisé en modules logiques et optimisés :

- **`models.rs`** - Structures de données avec `Copy` trait pour les enums
- **`discord_ipc.rs`** - Client Discord IPC de bas niveau (visibilité `pub(crate)`)
- **`manager.rs`** - Gestionnaire de haut niveau avec reconnexion automatique
- **`lib.rs`** - Point d'entrée avec exports spécifiques (pas de `pub use *`)

## 💻 Utilisation

```rust
use sup_drpc::{RichPresenceManager, RichPresenceConfig, ActivityType, RichPresenceAssets};

// Créer un gestionnaire
let mut manager = RichPresenceManager::new();

// Initialiser avec un App ID Discord
manager.initialize("123456789012345678")?;

// Créer une configuration (ActivityType implémente Copy)
let config = RichPresenceConfig {
    application_id: Some("123456789012345678".to_string()),
    activity_type: ActivityType::Playing, // Pas de clone nécessaire !
    details: Some("Joue à un jeu".to_string()),
    state: Some("En ligne".to_string()),
    assets: Some(RichPresenceAssets {
        large_image: Some("game_logo".to_string()),
        large_text: Some("Mon Jeu".to_string()),
        small_image: None,
        small_text: None,
    }),
    timestamps: None,
    buttons: None,
    party: None,
    secrets: None,
};

// Mettre à jour la présence
manager.update_presence(&config)?;

// Vérifier la connexion
if manager.is_connected() {
    println!("Discord connecté !");
}

// Se déconnecter proprement
manager.disconnect()?;
```

## 🎯 Types disponibles

### Types principaux
- `RichPresenceManager` - Gestionnaire principal
- `RichPresenceConfig` - Configuration complète
- `ActivityType` - Type d'activité (Playing, Listening, Watching, Competing)

### Types de configuration
- `RichPresenceAssets` - Images et textes
- `RichPresenceTimestamps` - Timestamps de début/fin
- `RichPresenceButton` - Boutons personnalisés
- `RichPresenceParty` - Informations de groupe
- `RichPresenceSecrets` - Secrets pour join/spectate

## ⚡ Optimisations appliquées

- **Imports spécifiques** : Pas de `use *`, seuls les types nécessaires
- **Copy trait** : `ActivityType` implémente `Copy` pour éviter les clones
- **Visibilité contrôlée** : `DiscordRpcClient` en `pub(crate)`
- **API minimaliste** : Exports contrôlés dans `lib.rs`

## 📦 Dépendances

- `discord-rich-presence` - Client Discord RPC
- `serde` - Sérialisation/désérialisation
- `anyhow` - Gestion d'erreurs

## 🔧 Exemple complet

```rust
use sup_drpc::{
    RichPresenceManager, RichPresenceConfig, ActivityType, 
    RichPresenceAssets, RichPresenceTimestamps, RichPresenceButton
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut manager = RichPresenceManager::new();
    
    // Initialisation
    manager.initialize("123456789012345678")?;
    
    // Configuration avec tous les éléments
    let config = RichPresenceConfig {
        application_id: Some("123456789012345678".to_string()),
        activity_type: ActivityType::Listening,
        details: Some("Écoute de la musique".to_string()),
        state: Some("Spotify".to_string()),
        assets: Some(RichPresenceAssets {
            large_image: Some("spotify_logo".to_string()),
            large_text: Some("Spotify".to_string()),
            small_image: Some("music_note".to_string()),
            small_text: Some("En écoute".to_string()),
        }),
        timestamps: Some(RichPresenceTimestamps {
            start: Some(chrono::Utc::now().timestamp()),
            end: None,
        }),
        buttons: Some(vec![
            RichPresenceButton {
                label: "Écouter sur Spotify".to_string(),
                url: "https://open.spotify.com".to_string(),
            }
        ]),
        party: None,
        secrets: None,
    };
    
    // Mise à jour
    manager.update_presence(&config)?;
    
    println!("Rich Presence configuré avec succès !");
    Ok(())
}
```