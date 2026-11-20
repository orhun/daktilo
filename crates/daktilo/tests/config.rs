#![cfg(target_os = "linux")]

use std::{fs, io, process::Command};

#[test]
fn finds_config_in_documented_locations() -> io::Result<()> {
    let config_dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("config-discovery");
    let app_dir = config_dir.join("daktilo");
    fs::create_dir_all(&app_dir)?;

    // Populate all locations before checking their precedence.
    let configs = [
        (config_dir.join("daktilo.toml"), "top-level"),
        (app_dir.join("daktilo.toml"), "app-toml"),
        (app_dir.join("config"), "app-config"),
    ];
    for (path, preset) in &configs {
        fs::write(
            path,
            format!("[[sound_preset]]\nname = \"{preset}\"\nkey_config = []\n"),
        )?;
    }

    for (path, preset) in &configs {
        let output = Command::new(env!("CARGO_BIN_EXE_daktilo"))
            .arg("--list-presets")
            .env("XDG_CONFIG_HOME", &config_dir)
            .env_remove("DAKTILO_CONFIG")
            .output()?;
        assert!(output.status.success(), "{output:?}");
        let stdout = String::from_utf8_lossy(&output.stdout);
        fs::remove_file(path)?;
        assert!(
            stdout.contains(&format!("[{preset}]")),
            "{path:?}: {stdout}"
        );
    }
    Ok(())
}
