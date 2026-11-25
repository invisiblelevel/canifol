use std::collections::HashMap;
use std::path::PathBuf;

/// Credentials that must be available at compile time, because the datasheet
/// modules read them with `option_env!` (this is what makes portable builds work
/// without a `.env` next to the executable).
const CREDENTIAL_KEYS: &[&str] = &[
    "DIGIKEY_CLIENT_ID",
    "DIGIKEY_CLIENT_SECRET",
    "ELEMENT14_API_KEY",
    "TME_TOKEN",
    "TME_SECRET",
];

fn parse_env_file(content: &str) -> HashMap<String, String> {
    let mut values = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let line = line.strip_prefix("export ").unwrap_or(line).trim();
        let Some((key, raw_value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let mut value = raw_value.trim().to_string();
        if value.len() >= 2
            && ((value.starts_with('"') && value.ends_with('"'))
                || (value.starts_with('\'') && value.ends_with('\'')))
        {
            value = value[1..value.len() - 1].to_string();
        }
        if !key.is_empty() && !value.is_empty() {
            values.insert(key.to_string(), value);
        }
    }
    values
}

fn main() {
    // `tauri build` / CI pass the keys as process environment variables; local
    // builds usually have them in `src-tauri/.env`. Support both, with the
    // process environment taking precedence.
    let manifest_dir = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is always set by Cargo"),
    );
    let dotenv_path = manifest_dir.join(".env");
    println!("cargo:rerun-if-changed={}", dotenv_path.display());
    let dotenv = std::fs::read_to_string(&dotenv_path)
        .map(|content| parse_env_file(&content))
        .unwrap_or_default();

    for key in CREDENTIAL_KEYS {
        println!("cargo:rerun-if-env-changed={key}");
        let value = std::env::var(key)
            .ok()
            .filter(|value| !value.trim().is_empty())
            .or_else(|| dotenv.get(*key).cloned());
        if let Some(value) = value {
            // Only the variable name is printed to the build log, never its value.
            println!("cargo:rustc-env={key}={value}");
        }
    }

    tauri_build::build()
}
