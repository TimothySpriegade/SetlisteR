use crate::data::models::playlist_data::PlaylistData;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[cfg(windows)]
const SIDECAR_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ytmusic_sidecar.exe"));

#[cfg(not(windows))]
const SIDECAR_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ytmusic_sidecar"));

#[cfg(windows)]
const SIDECAR_FILENAME: &str = "ytmusic_sidecar.exe";

#[cfg(not(windows))]
const SIDECAR_FILENAME: &str = "ytmusic_sidecar";

pub struct YTMusicSidecar {
    executable_path: PathBuf,
}

impl YTMusicSidecar {
    pub fn initialize() -> Result<Self, String> {
        let temp_dir = env::temp_dir();
        let sidecar_path = temp_dir.join(SIDECAR_FILENAME);

        fs::write(&sidecar_path, SIDECAR_BYTES)
            .map_err(|e| format!("Failed to write sidecar executable: {}", e))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(mut perms) = fs::metadata(&sidecar_path).map(|m| m.permissions()) {
                perms.set_mode(0o755);
                let _ = fs::set_permissions(&sidecar_path, perms);
            }
        }

        Ok(Self {
            executable_path: sidecar_path,
        })
    }

    pub fn login(&self, headers_raw: &str) -> Result<String, String> {
        let output_file = env::temp_dir().join("ytmusic_browser.json");

        let status = Command::new(&self.executable_path)
            .arg("login")
            .arg("--headers")
            .arg(headers_raw)
            .arg("--output-file")
            .arg(&output_file)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .map_err(|e| format!("Failed to execute sidecar: {}", e))?;

        if !status.success() {
            return Err("Login flow failed".to_string());
        }

        fs::read_to_string(&output_file)
            .map_err(|e| format!("Failed to read browser JSON from output file: {}", e))
    }

    pub fn create_playlist(
        &self,
        playlist_data: &PlaylistData,
        browser_json: &str,
    ) -> Result<(), String> {
        let json_payload = serde_json::to_string(playlist_data)
            .map_err(|e| format!("Failed to serialize playlist data: {}", e))?;

        let mut child = Command::new(&self.executable_path)
            .arg("create-playlist")
            .env("YTMUSIC_BROWSER_JSON", browser_json)
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn sidecar: {}", e))?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(json_payload.as_bytes())
                .map_err(|e| format!("Failed to write to sidecar stdin: {}", e))?;
        }

        let status = child
            .wait()
            .map_err(|e| format!("Failed to wait for sidecar: {}", e))?;

        if status.success() {
            Ok(())
        } else {
            Err(format!(
                "Playlist creation failed with exit status: {}",
                status
            ))
        }
    }
}
