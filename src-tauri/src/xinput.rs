use std::fs;
use std::path::Path;
use log::{info, warn, error};

/// I file di XInputPlus da copiare (embedded nel binary)
const XINPUT_FILES: &[(&str, &[u8])] = &[
    ("XInputPlus.ini", include_bytes!("../resources/xinputplus/XInputPlus.ini")),
    ("Dinput.dll", include_bytes!("../resources/xinputplus/Dinput.dll")),
    ("Dinput8.dll", include_bytes!("../resources/xinputplus/Dinput8.dll")),
    ("XInput1_3.dll", include_bytes!("../resources/xinputplus/XInput1_3.dll")),
];

/// Installa XInputPlus nella cartella del gioco
///
/// Questa funzione copia i file di XInputPlus (DLL e INI) nella cartella
/// dove si trova mhfo.dll / mhfo-hd.dll.
///
/// Funziona sia su Windows (mhf_iel::run integrato) che su Linux (Wine + mhf-iel-cli.exe).
///
/// # Comportamento
/// - Sovrascrive sempre i file per garantire la versione corretta
/// - Crea un file .xinputplus_version per tracciare la versione installata
///
/// # Parametri
/// - `game_folder`: Path alla cartella del gioco (dove si trova mhfo.dll)
pub fn setup_xinputplus(game_folder: &Path) -> Result<(), Box<dyn std::error::Error>> {
    info!("🎮 Installing XInputPlus for controller support...");
    info!("   Target folder: {:?}", game_folder);

    // Verifica che la cartella esista
    if !game_folder.exists() {
        let err = format!("Game folder not found: {:?}", game_folder);
        error!("{}", err);
        return Err(err.into());
    }

    let mut installed_count = 0;

    // Copia ogni file
    for (filename, content) in XINPUT_FILES {
        let target_path = game_folder.join(filename);

        match fs::write(&target_path, content) {
            Ok(_) => {
                info!("   ✅ Installed: {}", filename);
                installed_count += 1;
            }
            Err(e) => {
                warn!("   ⚠️ Failed to install {}: {}", filename, e);
                // Non blocchiamo l'esecuzione, ma avvertiamo
            }
        }
    }

    if installed_count == XINPUT_FILES.len() {
        info!("✅ XInputPlus installed successfully ({} files)", installed_count);

        // Crea file marker per tracciare la versione
        let version_file = game_folder.join(".xinputplus_version");
        let _ = fs::write(version_file, "4.15.2-butterclient");

        Ok(())
    } else {
        let err = format!("Only {} of {} files installed", installed_count, XINPUT_FILES.len());
        error!("❌ {}", err);
        Err(err.into())
    }
}

/// Verifica se XInputPlus è già installato
pub fn is_xinputplus_installed(game_folder: &Path) -> bool {
    let xinput_dll = game_folder.join("Xinput1_3.dll");
    let xinput_ini = game_folder.join("XInputPlus.ini");

    xinput_dll.exists() && xinput_ini.exists()
}

/// Rimuove XInputPlus dalla cartella del gioco
/// (Utile per debug o se l'utente vuole disabilitarlo)
pub fn remove_xinputplus(game_folder: &Path) -> Result<(), Box<dyn std::error::Error>> {
    info!("🗑️ Removing XInputPlus...");

    for (filename, _) in XINPUT_FILES {
        let file_path = game_folder.join(filename);
        if file_path.exists() {
            fs::remove_file(&file_path)?;
            info!("   ✅ Removed: {}", filename);
        }
    }

    // Rimuovi file marker
    let version_file = game_folder.join(".xinputplus_version");
    let _ = fs::remove_file(version_file);

    info!("✅ XInputPlus removed successfully");
    Ok(())
}
