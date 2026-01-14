use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::endpoint::Endpoint;

use log::{warn, info};

use serde::{Deserialize, Serialize};

const APP_NAME: &str = "mhf-launcher";

#[derive(Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub username: String,
    pub remember_me: bool,
}

// ✅ Struttura per salvare password su file
#[derive(Default, Deserialize, Serialize)]
struct CredentialsFile {
    // HashMap<endpoint_name, HashMap<username, password>>
    passwords: HashMap<String, HashMap<String, String>>,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct UserManager {
    data: [HashMap<String, UserData>; 2],
}

impl UserManager {
    fn get_target(&self, endpoint: &'_ Endpoint) -> String {
        format!("{}:{}", endpoint.name, endpoint.is_remote)
    }

    // ✅ Percorso file credenziali
    fn credentials_file_path() -> PathBuf {
        // ~/.config/mhfz-launcher/credentials.json
        dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("mhfz-launcher")
        .join("credentials.json")
    }

    // ✅ Carica password da file JSON (STATIC - non usa &self)
    fn load_password_from_file(endpoint: &Endpoint, username: &str) -> Option<String> {
        let path = Self::credentials_file_path();

        if !path.exists() {
            return None;
        }

        match fs::read_to_string(&path) {
            Ok(content) => {
                match serde_json::from_str::<CredentialsFile>(&content) {
                    Ok(creds) => {
                        creds.passwords
                        .get(&endpoint.name)
                        .and_then(|users| users.get(username))
                        .cloned()
                    }
                    Err(e) => {
                        warn!("Failed to parse credentials file: {}", e);
                        None
                    }
                }
            }
            Err(e) => {
                warn!("Failed to read credentials file: {}", e);
                None
            }
        }
    }

    // ✅ Salva password su file JSON (STATIC - non usa &self)
    fn save_password_to_file(endpoint: &Endpoint, username: &str, password: &str) {
        let path = Self::credentials_file_path();

        // Crea directory se non esiste
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        // Carica credenziali esistenti o crea nuove
        let mut creds = if path.exists() {
            fs::read_to_string(&path)
            .ok()
            .and_then(|content| serde_json::from_str::<CredentialsFile>(&content).ok())
            .unwrap_or_default()
        } else {
            CredentialsFile::default()
        };

        // Aggiungi/aggiorna password
        creds.passwords
        .entry(endpoint.name.clone())
        .or_insert_with(HashMap::new)
        .insert(username.to_owned(), password.to_owned());

        // Salva su disco
        if let Ok(json) = serde_json::to_string_pretty(&creds) {
            if let Err(e) = fs::write(&path, json) {
                warn!("Failed to write credentials file: {}", e);
            } else {
                info!("✅ Password saved to config file (fallback)");
            }
        }
    }

    // ✅ Rimuovi password da file JSON (STATIC - non usa &self)
    fn delete_password_from_file(endpoint: &Endpoint, username: &str) {
        let path = Self::credentials_file_path();

        if !path.exists() {
            return;
        }

        let mut creds = match fs::read_to_string(&path)
        .ok()
        .and_then(|content| serde_json::from_str::<CredentialsFile>(&content).ok())
        {
            Some(c) => c,
            None => return,
        };

        // Rimuovi password
        if let Some(users) = creds.passwords.get_mut(&endpoint.name) {
            users.remove(username);
        }

        // Salva modifiche
        if let Ok(json) = serde_json::to_string_pretty(&creds) {
            let _ = fs::write(&path, json);
        }
    }

    pub fn get(&self, endpoint: &'_ Endpoint) -> (UserData, String) {
        let target = self.get_target(endpoint);
        let data = &self.data[endpoint.is_remote as usize];
        let userdata = data
        .get(&endpoint.name)
        .cloned()
        .unwrap_or_else(|| UserData {
            username: "".into(),
                        remember_me: true,
        });

        let password = if !userdata.username.is_empty() {
            // ✅ Prova keyring prima
            let keyring_password = keyring::Entry::new_with_target(&target, APP_NAME, &userdata.username)
            .and_then(|entry| entry.get_password())
            .ok();

            if let Some(pwd) = keyring_password {
                info!("🔑 Password loaded from system keyring");
                pwd
            } else {
                // ✅ FALLBACK: Carica da file se keyring fallisce
                if let Some(pwd) = Self::load_password_from_file(endpoint, &userdata.username) {
                    info!("📄 Password loaded from config file (fallback)");
                    pwd
                } else {
                    warn!("⚠️ No password found in keyring or config file");
                    "".to_owned()
                }
            }
        } else {
            "".to_owned()
        };

        (userdata, password)
    }

    pub fn set(&mut self, endpoint: &'_ Endpoint, userdata: UserData, password: String) {
        let target = self.get_target(endpoint);
        let entry = keyring::Entry::new_with_target(&target, APP_NAME, &userdata.username);

        if userdata.remember_me {
            // ✅ FIX: Chiama le funzioni PRIMA di prendere il borrow mutabile
            match entry.and_then(|e| e.set_password(&password)) {
                Ok(_) => {
                    info!("✅ Password saved to system keyring");
                }
                Err(e) => {
                    warn!("⚠️ Keyring failed: {}, using config file fallback", e);
                    // ✅ FALLBACK: Salva su file (chiamata PRIMA del borrow mutabile)
                    Self::save_password_to_file(endpoint, &userdata.username, &password);
                }
            }

            // ✅ Ora possiamo prendere il borrow mutabile
            let data = &mut self.data[endpoint.is_remote as usize];
            data.insert(endpoint.name.to_owned(), userdata);
        } else {
            // ✅ FIX: Cancella password PRIMA del borrow mutabile
            entry
            .and_then(|e| e.delete_password())
            .unwrap_or_else(|e| warn!("Failed to delete from keyring: {}", e));

            Self::delete_password_from_file(endpoint, &userdata.username);

            // ✅ Ora possiamo prendere il borrow mutabile
            let data = &mut self.data[endpoint.is_remote as usize];
            data.remove(&endpoint.name);
        }
    }
}
