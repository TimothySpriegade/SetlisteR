use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::process::Command;

fn hash_python_sources(src_dir: &Path) -> u64 {
    let mut hasher = DefaultHasher::new();
    for filename in ["main.py", "auth.py", "playlist.py", "pyproject.toml"] {
        let path = src_dir.join(filename);
        if let Ok(contents) = fs::read(&path) {
            contents.hash(&mut hasher);
        }
    }
    hasher.finish()
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let python_src_dir = PathBuf::from(&manifest_dir).join("ytmusic_sidecar");
    let out_path = PathBuf::from(&out_dir);

    let sidecar_name = if cfg!(windows) {
        "ytmusic_sidecar.exe"
    } else {
        "ytmusic_sidecar"
    };

    let sidecar_bin = out_path.join(sidecar_name);
    let stamp_file = out_path.join("ytmusic_sidecar.stamp");

    let current_hash = hash_python_sources(&python_src_dir);
    let cached_hash: Option<u64> = fs::read_to_string(&stamp_file)
        .ok()
        .and_then(|s| s.trim().parse().ok());

    let needs_rebuild = !sidecar_bin.exists() || (cached_hash != Some(current_hash));

    if needs_rebuild {
        let status = Command::new("uv")
            .arg("run")
            .arg("pyinstaller")
            .arg("--onefile")
            .arg("--name")
            .arg("ytmusic_sidecar")
            .arg("--collect-all")
            .arg("ytmusicapi")
            .arg("--distpath")
            .arg(&out_dir)
            .arg("main.py")
            .current_dir(&python_src_dir)
            .status()
            .expect("Failed to execute uv run pyinstaller. Is uv installed?");

        if !status.success() {
            panic!("PyInstaller failed to build the Python sidecar.");
        }

        fs::write(&stamp_file, current_hash.to_string())
            .expect("Failed to write sidecar stamp file.");
    }

    println!("cargo:rerun-if-changed=ytmusic_sidecar/main.py");
    println!("cargo:rerun-if-changed=ytmusic_sidecar/auth.py");
    println!("cargo:rerun-if-changed=ytmusic_sidecar/playlist.py");
    println!("cargo:rerun-if-changed=ytmusic_sidecar/pyproject.toml");
}
