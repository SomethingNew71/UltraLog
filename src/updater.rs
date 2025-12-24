//! Auto-update functionality for UltraLog.
//!
//! Checks GitHub releases for new versions, downloads updates,
//! and provides installation assistance.

use serde::Deserialize;
use std::io::Write;
use std::path::PathBuf;

const GITHUB_API_URL: &str =
    "https://api.github.com/repos/SomethingNew71/UltraLog/releases/latest";
const USER_AGENT: &str = concat!("UltraLog/", env!("CARGO_PKG_VERSION"));

// ============================================================================
// Data Structures
// ============================================================================

/// GitHub release information from the API
#[derive(Debug, Clone, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    #[allow(dead_code)]
    pub name: String,
    pub html_url: String,
    pub body: Option<String>,
    pub assets: Vec<ReleaseAsset>,
    pub prerelease: bool,
    pub draft: bool,
}

/// A release asset (downloadable file)
#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

/// Information about an available update
#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub current_version: String,
    pub new_version: String,
    pub release_notes: Option<String>,
    pub download_url: String,
    pub download_size: u64,
    pub release_page_url: String,
}

/// Result from update check operation
#[derive(Debug, Clone)]
pub enum UpdateCheckResult {
    /// A newer version is available
    UpdateAvailable(UpdateInfo),
    /// Current version is the latest
    UpToDate,
    /// Error occurred during check
    Error(String),
}

/// Result from download operation
#[derive(Debug, Clone)]
pub enum DownloadResult {
    /// Download completed successfully
    Success(PathBuf),
    /// Download failed
    Error(String),
}

/// Current state of the update process
#[derive(Debug, Clone, Default)]
pub enum UpdateState {
    #[default]
    Idle,
    Checking,
    UpdateAvailable(UpdateInfo),
    Downloading,
    ReadyToInstall(PathBuf),
    Error(String),
}

/// Platform-specific asset detection
#[derive(Debug, Clone, Copy)]
pub enum Platform {
    WindowsX64,
    MacOSIntel,
    MacOSArm,
    LinuxX64,
}

impl Platform {
    /// Detect current platform at compile time
    pub fn current() -> Option<Self> {
        #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
        {
            return Some(Platform::WindowsX64);
        }

        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        {
            return Some(Platform::MacOSIntel);
        }

        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        {
            return Some(Platform::MacOSArm);
        }

        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        {
            return Some(Platform::LinuxX64);
        }

        #[cfg(not(any(
            all(target_os = "windows", target_arch = "x86_64"),
            all(target_os = "macos", target_arch = "x86_64"),
            all(target_os = "macos", target_arch = "aarch64"),
            all(target_os = "linux", target_arch = "x86_64")
        )))]
        {
            return None;
        }
    }

    /// Get the expected asset filename for this platform
    pub fn asset_name(&self) -> &'static str {
        match self {
            Platform::WindowsX64 => "ultralog-windows.zip",
            Platform::MacOSIntel => "ultralog-macos-intel.dmg",
            Platform::MacOSArm => "ultralog-macos-arm64.dmg",
            Platform::LinuxX64 => "ultralog-linux.tar.gz",
        }
    }

    /// Get the file extension for downloaded asset
    pub fn extension(&self) -> &'static str {
        match self {
            Platform::WindowsX64 => "zip",
            Platform::MacOSIntel | Platform::MacOSArm => "dmg",
            Platform::LinuxX64 => "tar.gz",
        }
    }
}

// ============================================================================
// Core Functions
// ============================================================================

/// Check for updates by querying GitHub releases API.
/// This is a blocking operation - run in a background thread.
pub fn check_for_updates() -> UpdateCheckResult {
    let current_version = env!("CARGO_PKG_VERSION");

    // Make HTTP request to GitHub API
    let mut response = match ureq::get(GITHUB_API_URL)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/vnd.github.v3+json")
        .call()
    {
        Ok(resp) => resp,
        Err(ureq::Error::StatusCode(status)) => {
            return UpdateCheckResult::Error(format!("GitHub API returned status {}", status));
        }
        Err(e) => {
            return UpdateCheckResult::Error(format!("Network error: {}", e));
        }
    };

    // Parse JSON response
    let release: GitHubRelease = match response.body_mut().read_json() {
        Ok(r) => r,
        Err(e) => {
            return UpdateCheckResult::Error(format!("Failed to parse response: {}", e));
        }
    };

    // Skip drafts and prereleases
    if release.draft || release.prerelease {
        return UpdateCheckResult::UpToDate;
    }

    // Parse versions for comparison
    let remote_version_str = release.tag_name.trim_start_matches('v');

    let current = match semver::Version::parse(current_version) {
        Ok(v) => v,
        Err(_) => {
            return UpdateCheckResult::Error("Invalid current version format".to_string());
        }
    };

    let remote = match semver::Version::parse(remote_version_str) {
        Ok(v) => v,
        Err(_) => {
            return UpdateCheckResult::Error(format!(
                "Invalid remote version format: {}",
                remote_version_str
            ));
        }
    };

    // Compare versions
    if remote <= current {
        return UpdateCheckResult::UpToDate;
    }

    // Find platform-specific asset
    let platform = match Platform::current() {
        Some(p) => p,
        None => {
            return UpdateCheckResult::Error("Unsupported platform for auto-update".to_string());
        }
    };

    let asset_name = platform.asset_name();
    let asset = match release.assets.iter().find(|a| a.name == asset_name) {
        Some(a) => a,
        None => {
            return UpdateCheckResult::Error(format!(
                "No release asset found for {} in version {}",
                asset_name, remote_version_str
            ));
        }
    };

    UpdateCheckResult::UpdateAvailable(UpdateInfo {
        current_version: current_version.to_string(),
        new_version: remote_version_str.to_string(),
        release_notes: release.body,
        download_url: asset.browser_download_url.clone(),
        download_size: asset.size,
        release_page_url: release.html_url,
    })
}

/// Download update file to temp directory.
/// This is a blocking operation - run in a background thread.
pub fn download_update(url: &str) -> DownloadResult {
    let platform = match Platform::current() {
        Some(p) => p,
        None => return DownloadResult::Error("Unsupported platform".to_string()),
    };

    // Create temp file path
    let temp_dir = std::env::temp_dir();
    let filename = format!("ultralog-update.{}", platform.extension());
    let download_path = temp_dir.join(&filename);

    // Download file
    let response = match ureq::get(url).header("User-Agent", USER_AGENT).call() {
        Ok(resp) => resp,
        Err(e) => return DownloadResult::Error(format!("Download failed: {}", e)),
    };

    // Create output file
    let mut file = match std::fs::File::create(&download_path) {
        Ok(f) => f,
        Err(e) => return DownloadResult::Error(format!("Failed to create file: {}", e)),
    };

    // Read response body into file
    let mut reader = response.into_body().into_reader();
    let mut buffer = [0u8; 8192];

    loop {
        match std::io::Read::read(&mut reader, &mut buffer) {
            Ok(0) => break, // EOF
            Ok(n) => {
                if let Err(e) = file.write_all(&buffer[..n]) {
                    return DownloadResult::Error(format!("Write error: {}", e));
                }
            }
            Err(e) => return DownloadResult::Error(format!("Read error: {}", e)),
        }
    }

    // Ensure all data is written
    if let Err(e) = file.flush() {
        return DownloadResult::Error(format!("Failed to flush file: {}", e));
    }

    DownloadResult::Success(download_path)
}

/// Open the downloaded update file using system default handler.
pub fn install_update(path: &std::path::Path) -> Result<(), String> {
    open::that(path).map_err(|e| format!("Failed to open update file: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        // Should return Some on supported platforms
        let platform = Platform::current();

        #[cfg(any(
            all(target_os = "windows", target_arch = "x86_64"),
            all(target_os = "macos", target_arch = "x86_64"),
            all(target_os = "macos", target_arch = "aarch64"),
            all(target_os = "linux", target_arch = "x86_64")
        ))]
        assert!(platform.is_some());

        if let Some(p) = platform {
            assert!(!p.asset_name().is_empty());
            assert!(!p.extension().is_empty());
        }
    }

    #[test]
    fn test_asset_names() {
        assert_eq!(Platform::WindowsX64.asset_name(), "ultralog-windows.zip");
        assert_eq!(
            Platform::MacOSIntel.asset_name(),
            "ultralog-macos-intel.dmg"
        );
        assert_eq!(Platform::MacOSArm.asset_name(), "ultralog-macos-arm64.dmg");
        assert_eq!(Platform::LinuxX64.asset_name(), "ultralog-linux.tar.gz");
    }
}
