//! Gestionnaire de haut niveau pour Discord Rich Presence

use anyhow::Result;
use crate::discord_ipc::DiscordRpcClient;
use crate::models::RichPresenceConfig;

/// Gestionnaire de Rich Presence Discord
pub struct RichPresenceManager {
    client: Option<DiscordRpcClient>,
    app_id: String,
}

impl RichPresenceManager {
    /// Crée un nouveau gestionnaire
    pub fn new() -> Self {
        Self {
            client: None,
            app_id: String::new(),
        }
    }

    /// Initialise la connexion Discord avec un App ID
    pub fn initialize(&mut self, app_id: &str) -> Result<()> {
        self.app_id = app_id.to_string();
        
        // Se déconnecter si déjà connecté
        if let Some(ref mut client) = self.client {
            let _ = client.disconnect();
        }

        // Créer et connecter le nouveau client
        let mut client = DiscordRpcClient::new(app_id)?;
        client.connect()?;
        
        self.client = Some(client);
        Ok(())
    }


    /// Met à jour la présence Discord
    pub fn update_presence(&mut self, config: &RichPresenceConfig) -> Result<()> {
        if let Some(ref mut client) = self.client {
            client.update_presence(config)?;
        } else {
            return Err(anyhow::anyhow!("Client non initialisé"));
        }
        Ok(())
    }

    /// Vérifie si Discord est connecté
    pub fn is_connected(&self) -> bool {
        self.client.as_ref().map_or(false, |c| c.is_connected())
    }

    /// Se déconnecte de Discord
    pub fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut client) = self.client {
            client.disconnect()?;
        }
        self.client = None;
        Ok(())
    }

    /// Obtient l'App ID actuel
    pub fn get_app_id(&self) -> &str {
        &self.app_id
    }
}

impl Default for RichPresenceManager {
    fn default() -> Self {
        Self::new()
    }
}
