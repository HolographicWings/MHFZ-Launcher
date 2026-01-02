use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use log::{info, debug, error, warn};
use mhf_iel::MhfConfig;

#[derive(Debug)]
pub struct MhfConfigLinux {
    pub game_folder: PathBuf,
    pub config: MhfConfig,
}

fn log_to_file(msg: &str) {
    let log_path = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string()) + "/mhfz-launcher.log";
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_path) {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let _ = writeln!(file, "[{}] {}", timestamp, msg);
    }
}

/// Rileva se siamo su SteamOS
fn is_steamos() -> bool {
    // Controlla /etc/os-release
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        if content.contains("ID=steamos") || content.contains("ID=\"steamos\"") {
            log_to_file("🎮 Detected SteamOS");
            return true;
        }
    }

    // Fallback: controlla se esiste steamos-readonly command
    if Command::new("which")
        .arg("steamos-readonly")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
        {
            log_to_file("🎮 Detected SteamOS (via steamos-readonly)");
            return true;
        }

        log_to_file("🐧 Detected standard Linux");
    false
}

/// Ottiene il comando Wine appropriato per il sistema
fn get_wine_command() -> (String, Vec<String>) {
    if is_steamos() {
        // SteamOS: usa Wine Flatpak
        log_to_file("🍷 Using Wine Flatpak (SteamOS)");
        ("flatpak".to_string(), vec!["run".to_string(), "org.winehq.Wine".to_string()])
    } else {
        // Linux standard: usa Wine di sistema
        log_to_file("🍷 Using system Wine");
        ("wine".to_string(), vec![])
    }
}

fn install_japanese_fonts(game_folder: &std::path::Path, wineprefix: &str) {
    let fonts_source = game_folder.join("fonts");
    if !fonts_source.exists() {
        log_to_file("⚠️  fonts/ folder not found, skipping font installation");
        warn!("fonts/ folder not found in game directory, skipping font installation");
        return;
    }

    // Crea cartella Fonts
    let fonts_dest = std::path::Path::new(wineprefix)
    .join("drive_c/windows/Fonts");
    if !fonts_dest.exists() {
        log_to_file("🔧 Creating Fonts directory manually...");
        if let Err(e) = std::fs::create_dir_all(&fonts_dest) {
            log_to_file(&format!("❌ Failed to create Fonts directory: {}", e));
            error!("Failed to create Fonts directory: {}", e);
            return;
        }
        log_to_file(&format!("✅ Created: {:?}", fonts_dest));
    }

    log_to_file("🔤 Installing Japanese fonts...");
    info!("Installing Japanese fonts from fonts/ folder...");

    let mut count = 0;
    let mut font_names = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&fonts_source) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if ext_str == "ttf" || ext_str == "ttc" || ext_str == "otf" {
                    if let Some(filename) = path.file_name() {
                        let dest = fonts_dest.join(filename);
                        match std::fs::copy(&path, &dest) {
                            Ok(_) => {
                                log_to_file(&format!("  ✅ Installed: {:?}", filename));
                                font_names.push(filename.to_string_lossy().to_string());
                                count += 1;
                            }
                            Err(e) => log_to_file(&format!("  ❌ Failed to copy {:?}: {}", filename, e)),
                        }
                    }
                }
            }
        }
    }

    log_to_file(&format!("✅ Japanese fonts installed ({} files)", count));
    info!("Japanese fonts installation complete ({} files)", count);

    // ✅ REGISTRA I FONT NEL REGISTRO WINE!
    if count > 0 {
        log_to_file("📝 Registering fonts in Wine registry...");
        register_fonts_in_wine(wineprefix, &font_names);
    }
}

/// Registra i font nel registro Wine
fn register_fonts_in_wine(wineprefix: &str, font_files: &[String]) {
    let (wine_cmd, wine_args) = get_wine_command();

    for font_file in font_files {
        let font_name = if font_file.contains("gothic") {
            "MS Gothic & MS PGothic & MS UI Gothic (TrueType)"
        } else if font_file.contains("mincho") {
            "MS Mincho (TrueType)"
        } else if font_file.contains("meiryo") {
            "Meiryo (TrueType)"
        } else if font_file.contains("source") || font_file.contains("han") {
            "Source Han Sans (TrueType)"
        } else {
            continue; // Salta font non riconosciuti
        };

        log_to_file(&format!("  Registering: {} → {}", font_name, font_file));

        let mut reg_cmd = Command::new(&wine_cmd);
        for arg in &wine_args {
            reg_cmd.arg(arg);
        }

        if is_steamos() {
            reg_cmd.arg(format!("--env=WINEPREFIX={}", wineprefix));
            reg_cmd.arg("--command=wine");
            reg_cmd.arg("org.winehq.Wine");
        }

        let _ = reg_cmd
        .arg("reg")
        .arg("add")
        .arg("HKLM\\Software\\Microsoft\\Windows NT\\CurrentVersion\\Fonts")
        .arg("/v")
        .arg(font_name)
        .arg("/t")
        .arg("REG_SZ")
        .arg("/d")
        .arg(font_file)
        .arg("/f")
        .env("WINEPREFIX", wineprefix)
        .env("WINEDEBUG", "-all")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    }

    log_to_file("✅ Fonts registered in Wine registry");
}


pub fn run_linux(cfg: MhfConfigLinux) -> std::io::Result<()> {
    log_to_file("════════════════════════════════════════════════════");
    log_to_file("🎮 Monster Hunter Frontier Z - Linux Launcher");
    log_to_file("════════════════════════════════════════════════════");

    info!("=== Monster Hunter Frontier - Linux Launcher ===");
    debug!("Game folder: {:?}", cfg.game_folder);
    log_to_file(&format!("📁 Game folder: {:?}", cfg.game_folder));

    // ✅ Scrivi config.json
    info!("📝 Writing config.json...");
    log_to_file("📝 Writing config.json...");
    let config_path = cfg.game_folder.join("config.json");

    let notices_json: Vec<serde_json::Value> = cfg.config.notices.iter().map(|n| {
        serde_json::json!({
            "flags": n.flags,
            "data": &n.data
        })
    }).collect();

    let friends_json: Vec<serde_json::Value> = cfg.config.friends.iter().map(|f| {
        serde_json::json!({
            "cid": f.cid,
            "id": f.id,
            "name": &f.name
        })
    }).collect();

    let mez_stalls_str: Vec<String> = cfg.config.mez_stalls.iter().map(|s| {
        format!("{:?}", s)
    }).collect();

    let config_json = serde_json::json!({
        "char_id": cfg.config.char_id,
        "char_name": &cfg.config.char_name,
        "char_new": cfg.config.char_new,
        "char_hr": cfg.config.char_hr,
        "char_gr": cfg.config.char_gr,
        "char_ids": &cfg.config.char_ids,
        "user_rights": cfg.config.user_rights,
        "user_token": &cfg.config.user_token,
        "user_token_id": cfg.config.user_token_id,
        "user_name": &cfg.config.user_name,
        "user_password": &cfg.config.user_password,
        "server_host": &cfg.config.server_host,
        "server_port": cfg.config.server_port,
        "notices": notices_json,
        "version": format!("{:?}", cfg.config.version),
                                        "entrance_count": cfg.config.entrance_count,
                                        "current_ts": cfg.config.current_ts,
                                        "expiry_ts": cfg.config.expiry_ts,
                                        "messages": Vec::<String>::new(),
                                        "mez_event_id": cfg.config.mez_event_id,
                                        "mez_start": cfg.config.mez_start,
                                        "mez_end": cfg.config.mez_end,
                                        "mez_solo_tickets": cfg.config.mez_solo_tickets,
                                        "mez_group_tickets": cfg.config.mez_group_tickets,
                                        "mez_stalls": mez_stalls_str,
                                        "friends": friends_json,
    });

    std::fs::write(&config_path, serde_json::to_string_pretty(&config_json).unwrap())
    .map_err(|e| {
        let err_msg = format!("Failed to write config.json: {}", e);
        error!("❌ {}", err_msg);
        log_to_file(&format!("❌ {}", err_msg));
        std::io::Error::new(std::io::ErrorKind::Other, err_msg)
    })?;

    info!("✅ config.json written");
    log_to_file(&format!("✅ config.json written to: {:?}", config_path));

    // Cerca exe
    let mut mhf_iel_exe = cfg.game_folder.join("mhf-iel.exe");
    let mut exe_name = "mhf-iel.exe";

    if !mhf_iel_exe.exists() {
        mhf_iel_exe = cfg.game_folder.join("mhf-iel-cli.exe");
        exe_name = "mhf-iel-cli.exe";
    }

    if !mhf_iel_exe.exists() {
        let err_msg = "mhf-iel.exe or mhf-iel-cli.exe not found in game folder";
        error!("{}", err_msg);
        log_to_file(&format!("❌ {}", err_msg));
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, err_msg));
    }

    info!("Found game executable: {}", exe_name);
    log_to_file(&format!("✅ Found game executable: {}", exe_name));

    // Font config
    let fontconfig_path = env::var("FONTCONFIG_PATH")
    .unwrap_or_else(|_| "/etc/fonts".to_string());
    let fontconfig_file = env::var("FONTCONFIG_FILE")
    .unwrap_or_else(|_| "/etc/fonts/fonts.conf".to_string());
    let xdg_data_dirs = env::var("XDG_DATA_DIRS")
    .unwrap_or_else(|_| "/usr/share:/usr/local/share".to_string());

    log_to_file("🔤 Font configuration:");
    log_to_file(&format!("   FONTCONFIG_PATH: {}", fontconfig_path));
    log_to_file(&format!("   FONTCONFIG_FILE: {}", fontconfig_file));
    log_to_file(&format!("   XDG_DATA_DIRS: {}", xdg_data_dirs));

    // Wine prefix
    let wineprefix = env::var("WINEPREFIX").unwrap_or_else(|_| {
        let pfx_path = cfg.game_folder.join("pfx");
        pfx_path.to_string_lossy().to_string()
    });

    log_to_file(&format!("🍷 WINEPREFIX: {}", wineprefix));
    info!("WINEPREFIX: {}", wineprefix);

    // Ottieni comando Wine
    let (wine_cmd, wine_args) = get_wine_command();

    // ✅ Controlla se il prefix esiste
    let prefix_path = std::path::Path::new(&wineprefix);
    let system_reg = prefix_path.join("system.reg");

    let need_init = !prefix_path.exists() || !system_reg.exists();

    if need_init {
        if prefix_path.exists() {
            log_to_file("⚠️  Wine prefix folder exists but incomplete (missing system.reg)");
            warn!("Wine prefix exists but incomplete, re-initializing...");
        } else {
            log_to_file("🔧 First launch detected - Wine prefix does not exist");
            info!("Creating Wine prefix (this may take 1-2 minutes on first launch)...");
        }

        let _ = std::fs::create_dir_all(&wineprefix);
        log_to_file("⏳ Running wineboot --init (this may take 1-2 minutes on SteamOS)...");

        // ✅ FIX: Cattura stderr per debug
        let output = if is_steamos() {
            Command::new("flatpak")
            .arg("run")
            .arg(format!("--env=WINEPREFIX={}", &wineprefix))  // ← COSÌ!
            .arg(format!("--env=WINEDEBUG=-all"))
            .arg(format!("--env=WINEDLLOVERRIDES=winemenubuilder.exe=d"))
            .arg("--command=wineboot")
            .arg("org.winehq.Wine")
            .arg("--init")
            .env("FONTCONFIG_PATH", &fontconfig_path)
            .env("FONTCONFIG_FILE", &fontconfig_file)
            .env("XDG_DATA_DIRS", &xdg_data_dirs)
            .env("WINEDLLOVERRIDES", "winemenubuilder.exe=d")
            .env("WINEDEBUG", "-all")
            .stdin(Stdio::null())
            .output()
        } else {
            Command::new("wineboot")
            .arg("--init")
            .env("WINEPREFIX", &wineprefix)
            .env("FONTCONFIG_PATH", &fontconfig_path)
            .env("FONTCONFIG_FILE", &fontconfig_file)
            .env("XDG_DATA_DIRS", &xdg_data_dirs)
            .env("WINEDLLOVERRIDES", "winemenubuilder.exe=d")
            .env("WINEDEBUG", "-all")
            .stdin(Stdio::null())
            .output()
        };

        match output {
            Ok(out) => {
                if out.status.success() {
                    log_to_file("✅ wineboot --init completed successfully");
                    info!("Wine prefix initialized successfully");
                } else {
                    log_to_file(&format!("⚠️  wineboot exited with status: {}", out.status));
                    log_to_file(&format!("   stderr: {}", String::from_utf8_lossy(&out.stderr)));
                    warn!("wineboot exited with non-zero status but continuing");
                }

                // ✅ FIX CRITICO: Attendi di più su SteamOS (10 secondi)
                log_to_file("⏳ Waiting for Wine initialization to complete...");
                let wait_time = if is_steamos() { 10 } else { 3 };
                log_to_file(&format!("   Waiting {} seconds...", wait_time));
                std::thread::sleep(std::time::Duration::from_secs(wait_time));

                // Verifica system.reg (solo per info, non blocchiamo più)
                if system_reg.exists() {
                    log_to_file("✅ Wine prefix created successfully (system.reg found)");
                } else {
                    log_to_file("⚠️  system.reg not found yet, but continuing anyway");
                    log_to_file("   (Wine Flatpak may take longer to finish initialization)");
                }
            }
            Err(e) => {
                log_to_file(&format!("❌ Failed to run wineboot: {}", e));
                error!("Failed to run wineboot: {}", e);
            }
        }

        // ✅ FIX: Installa i font SEMPRE, anche se system.reg manca
        install_japanese_fonts(&cfg.game_folder, &wineprefix);

    } else {
        log_to_file("✅ Wine prefix already exists and configured");
        info!("✅ Wine prefix already configured");
    }

    // XAUTHORITY
    let xauthority = env::var("XAUTHORITY").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        format!("{}/.Xauthority", home)
    });
    log_to_file(&format!("🖥️  XAUTHORITY: {}", xauthority));

    // Inizializza wineserver
    debug!("Initializing wineserver...");
    log_to_file("🔧 Initializing wineserver...");

    let mut server_cmd = if is_steamos() {
        let mut cmd = Command::new(&wine_cmd);
        for arg in &wine_args {
            cmd.arg(arg);
        }
        cmd.arg("wineserver");
        cmd
    } else {
        Command::new("wineserver")
    };

    let _ = server_cmd
    .arg("-w")
    .env("WINEPREFIX", &wineprefix)
    .env("FONTCONFIG_PATH", &fontconfig_path)
    .env("FONTCONFIG_FILE", &fontconfig_file)
    .env("XDG_DATA_DIRS", &xdg_data_dirs)
    .env("WINEDEBUG", "-all")
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn();

    std::thread::sleep(std::time::Duration::from_secs(1));

    // Lancia gioco
    info!("🚀 Starting game via Wine...");
    log_to_file("🚀 Launching game...");
    log_to_file(&format!("   Wine command: {} {:?}", wine_cmd, wine_args));
    log_to_file(&format!("   Executable: {:?}", mhf_iel_exe));

    let mut game_cmd = Command::new("setsid");
    game_cmd.arg(&wine_cmd);

    for arg in &wine_args {
        game_cmd.arg(arg);
    }

    game_cmd.arg(&mhf_iel_exe);
    game_cmd.current_dir(&cfg.game_folder);
    game_cmd.env("WINEDEBUG", "-all");
    game_cmd.env("WINEPREFIX", &wineprefix);
    game_cmd.env("FONTCONFIG_PATH", &fontconfig_path);
    game_cmd.env("FONTCONFIG_FILE", &fontconfig_file);
    game_cmd.env("XDG_DATA_DIRS", &xdg_data_dirs);
    game_cmd.env("XAUTHORITY", &xauthority);
    game_cmd.stdin(Stdio::null());
    game_cmd.stdout(Stdio::null());
    game_cmd.stderr(Stdio::null());

    let result = game_cmd.spawn();

    match result {
        Ok(child) => {
            log_to_file(&format!("✅ Game launched successfully (PID: {})", child.id()));
            log_to_file("🎮 Game is running!");
            log_to_file("════════════════════════════════════════════════════");
            info!("✅ Game launched successfully (PID: {})", child.id());
            info!("🎮 Game is running");
            Ok(())
        }
        Err(e) => {
            log_to_file(&format!("❌ Failed to launch game: {}", e));
            log_to_file("════════════════════════════════════════════════════");
            error!("❌ Failed to launch game: {}", e);
            Err(e)
        }
    }
}
