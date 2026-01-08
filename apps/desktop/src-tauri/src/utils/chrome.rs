use std::process::Command;
use which::which;
use std::path::PathBuf;

pub fn launch_chrome(url: &str) -> Result<(), String> {
    let browser_names = if cfg!(target_os = "windows") {
        vec!["chrome.exe", "msedge.exe", "chromium.exe"]
    } else if cfg!(target_os = "macos") {
        vec![
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge"
        ]
    } else {
        vec!["google-chrome", "google-chrome-stable", "chromium", "chromium-browser"]
    };

    let browser_path = browser_names.iter()
        .find_map(|name| {
             if (name.contains('/') || name.contains('\\'))
                && std::path::Path::new(name).exists() {
                 return Some(PathBuf::from(name));
             }
             which(name).ok()
        })
        .ok_or_else(|| "Could not find Chrome, Chromium, or Edge installation".to_string())?;

    // Use a separate user data dir for "Joker" mode (remote debugging)
    let mut data_dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("."));
    data_dir.push("whaswapp-chrome-profile");

    let mut cmd = Command::new(browser_path);
    cmd.arg("--remote-debugging-port=9222");
    cmd.arg(format!("--user-data-dir={}", data_dir.display()));

    if !url.is_empty() {
        cmd.arg(url);
    } else {
         cmd.arg("https://web.whatsapp.com");
    }

    cmd.spawn()
        .map_err(|e| format!("Failed to launch browser: {}", e))?;

    Ok(())
}
