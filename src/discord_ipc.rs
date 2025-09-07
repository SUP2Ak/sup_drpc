//! Client Discord IPC de bas niveau

use discord_rich_presence::{
    DiscordIpc, DiscordIpcClient, 
    activity::{Activity, Assets, Timestamps, Button, Party, Secrets}
};
use anyhow::Result;
use crate::models::RichPresenceConfig;

/// Client Discord RPC de bas niveau (interne)
pub(crate) struct DiscordRpcClient {
    client: DiscordIpcClient,
    is_connected: bool,
}

impl DiscordRpcClient {
    /// Crée un nouveau client Discord RPC
    pub fn new(app_id: &str) -> Result<Self> {
        let client = DiscordIpcClient::new(app_id);
        Ok(Self {
            client,
            is_connected: false,
        })
    }

    /// Se connecte à Discord
    pub fn connect(&mut self) -> Result<()> {
        self.client.connect()?;
        self.is_connected = true;
        Ok(())
    }

    /// Se déconnecte de Discord
    pub fn disconnect(&mut self) -> Result<()> {
        if self.is_connected {
            self.client.close()?;
            self.is_connected = false;
        }
        Ok(())
    }

    /// Met à jour la présence Discord
    pub fn update_presence(&mut self, config: &RichPresenceConfig) -> Result<()> {
        if !self.is_connected {
            return Err(anyhow::anyhow!("Client non connecté"));
        }

        let mut activity = Activity::new();
        
        // Type d'activité
        activity = activity.activity_type(config.activity_type.into());

        // Détails et état
        if let Some(details) = &config.details {
            activity = activity.details(details);
        }
        if let Some(state) = &config.state {
            activity = activity.state(state);
        }

        // Assets
        if let Some(assets) = &config.assets {
            let mut discord_assets = Assets::new();
            
            if let Some(large_image) = &assets.large_image {
                discord_assets = discord_assets.large_image(large_image);
            }
            if let Some(large_text) = &assets.large_text {
                discord_assets = discord_assets.large_text(large_text);
            }
            if let Some(small_image) = &assets.small_image {
                discord_assets = discord_assets.small_image(small_image);
            }
            if let Some(small_text) = &assets.small_text {
                discord_assets = discord_assets.small_text(small_text);
            }
            
            activity = activity.assets(discord_assets);
        }

        // Timestamps
        if let Some(timestamps) = &config.timestamps {
            let mut discord_timestamps = Timestamps::new();
            
            if let Some(start) = timestamps.start {
                discord_timestamps = discord_timestamps.start(start);
            }
            if let Some(end) = timestamps.end {
                discord_timestamps = discord_timestamps.end(end);
            }
            
            activity = activity.timestamps(discord_timestamps);
        }

        // Boutons
        if let Some(buttons) = &config.buttons {
            let discord_buttons: Vec<Button> = buttons
                .iter()
                .map(|button| Button::new(&button.label, &button.url))
                .collect();
            activity = activity.buttons(discord_buttons);
        }

        // Party
        if let Some(party) = &config.party {
            let mut discord_party = Party::new();
            
            if let Some(id) = &party.id {
                discord_party = discord_party.id(id);
            }
            if let Some(size) = party.size {
                discord_party = discord_party.size(size);
            }
            
            activity = activity.party(discord_party);
        }

        // Secrets
        if let Some(secrets) = &config.secrets {
            let mut discord_secrets = Secrets::new();
            
            if let Some(join) = &secrets.join {
                discord_secrets = discord_secrets.join(join);
            }
            if let Some(spectate) = &secrets.spectate {
                discord_secrets = discord_secrets.spectate(spectate);
            }
            // Note: match_field n'est pas supporté par l'API discord-rich-presence
            
            activity = activity.secrets(discord_secrets);
        }

        self.client.set_activity(activity)?;
        Ok(())
    }

    /// Vérifie si le client est connecté
    pub fn is_connected(&self) -> bool {
        self.is_connected
    }
}
