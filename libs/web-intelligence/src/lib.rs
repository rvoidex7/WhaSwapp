use std::process::{Command, Child};
use which::which;
use std::path::PathBuf;

/// A helper to launch Chrome or compatible browsers with specific flags,
/// useful for developing with Built-in AI features.
pub struct BrowserLauncher {
    user_data_dir_name: String,
}

impl BrowserLauncher {
    /// Creates a new launcher instance.
    /// `profile_name` is the name of the folder in the cache directory
    /// where the browser profile will be stored.
    pub fn new(profile_name: &str) -> Self {
        Self {
            user_data_dir_name: profile_name.to_string(),
        }
    }

    /// Launches the browser with the given URL and extra arguments.
    /// Tries to find Chrome Canary/Dev first as they often have the latest AI features.
    pub fn launch(&self, url: &str, extra_args: &[&str]) -> Result<Child, String> {
        let mut browser_candidates: Vec<String> = Vec::new();

        if cfg!(target_os = "windows") {
             // Standard PATH binaries
            browser_candidates.extend(vec!["chrome.exe".to_string(), "msedge.exe".to_string(), "chromium.exe".to_string()]);

            // Common Windows Installation Paths
            let program_files = std::env::var("ProgramFiles").unwrap_or_else(|_| r"C:\Program Files".to_string());
            let program_files_x86 = std::env::var("ProgramFiles(x86)").unwrap_or_else(|_| r"C:\Program Files (x86)".to_string());
            let local_app_data = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| r"C:\Users\Default\AppData\Local".to_string());

            browser_candidates.push(format!(r"{}\Google\Chrome SxS\Application\chrome.exe", local_app_data)); // Canary
            browser_candidates.push(format!(r"{}\Google\Chrome\Application\chrome.exe", program_files));
            browser_candidates.push(format!(r"{}\Google\Chrome\Application\chrome.exe", program_files_x86));
            browser_candidates.push(format!(r"{}\Microsoft\Edge\Application\msedge.exe", program_files));
            browser_candidates.push(format!(r"{}\Microsoft\Edge\Application\msedge.exe", program_files_x86));

        } else if cfg!(target_os = "macos") {
            // Prioritize Canary/Dev for AI features
            browser_candidates.extend(vec![
                "/Applications/Google Chrome Canary.app/Contents/MacOS/Google Chrome Canary",
                "/Applications/Google Chrome Dev.app/Contents/MacOS/Google Chrome Dev",
                "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
                "/Applications/Chromium.app/Contents/MacOS/Chromium",
                "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge"
            ].into_iter().map(|s| s.to_string()));
        } else {
             // Linux
            browser_candidates.extend(vec!["google-chrome-unstable", "google-chrome-beta", "google-chrome", "google-chrome-stable", "chromium", "chromium-browser"].into_iter().map(|s| s.to_string()));
        };

        let browser_path = browser_candidates.iter()
            .find_map(|name| {
                 let path = PathBuf::from(name);
                 if path.is_absolute() {
                    if path.exists() {
                        return Some(path);
                    }
                 } else {
                     if let Ok(p) = which(name) {
                         return Some(p);
                     }
                 }
                 None
            })
            .ok_or_else(|| "Could not find Chrome, Chromium, or Edge installation".to_string())?;

        let mut data_dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("."));
        data_dir.push(&self.user_data_dir_name);

        let mut cmd = Command::new(browser_path);

        // Base flags for isolation
        cmd.arg(format!("--user-data-dir={}", data_dir.display()));

        for arg in extra_args {
            cmd.arg(arg);
        }

        if !url.is_empty() {
            cmd.arg(url);
        }

        cmd.spawn()
            .map_err(|e| format!("Failed to launch browser: {}", e))
    }

    /// Launches the browser specifically configured for AI development.
    ///
    /// - Enables remote debugging on the specified port.
    /// - Sets flags often needed for local AI features.
    /// - Allows insecure localhost (useful for local dev).
    pub fn launch_for_ai_dev(&self, url: &str, debug_port: u16) -> Result<Child, String> {
        let port_arg = format!("--remote-debugging-port={}", debug_port);
        // Common flags for enabling experimental AI features if they aren't default yet.
        // Note: These change often, so allowing the user to pass more via a different method is good,
        // but these are safe defaults for a "dev environment".
        let args = vec![
            port_arg.as_str(),
            // Ensure optimization guide is enabled (often needed for local LLMs)
            "--enable-features=OptimizationGuideModelDownloading,OptimizationGuideOnDeviceModel,PromptAPIForGeminiNano",
            // Allow HTTP for localhost testing
            "--allow-insecure-localhost",
        ];
        self.launch(url, &args)
    }
}
