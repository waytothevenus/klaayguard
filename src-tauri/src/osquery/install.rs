// src-tauri/src/osquery/install.rs
use anyhow::{Context, Result};
use log::info;
use std::process::Command;

#[cfg(target_os = "windows")]
pub fn install_osquery() -> Result<()> {
    info!("Installing osquery on Windows");
    
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    
    Command::new("choco")
        .args(&["install", "osquery", "-y", "--force"])
        .creation_flags(CREATE_NO_WINDOW)
        .status()
        .context("Failed to install via Chocolatey")?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn install_osquery() -> Result<()> {
    info!("Installing osquery on macOS");
    Command::new("brew")
        .env("HOMEBREW_NO_AUTO_UPDATE", "1")
        .args(&["install", "osquery", "--quiet"])
        .status()
        .context("Failed to install via Homebrew")?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn install_osquery() -> Result<()> {
    info!("Installing osquery on Linux");
    let status = Command::new("sh")
        .arg("-c")
        .arg("curl -sSL https://pkg.osquery.io/deb/osquery_5.10.2-1.linux_amd64.deb -o /tmp/osquery.deb && sudo dpkg -i -y /tmp/osquery.deb")
        .status()
        .context("Failed to install osquery")?;

    if !status.success() {
        anyhow::bail!("Installation failed with status: {}", status);
    }
    Ok(())
}

pub fn is_osquery_installed() -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        const DETACHED_PROCESS: u32 = 0x00000008;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
        
        // Combine multiple flags for maximum suppression
        let flags = CREATE_NO_WINDOW | DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP;
        
        std::process::Command::new("where")  // First check if osqueryi exists in PATH
            .arg("osqueryi")
            .creation_flags(flags)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .is_ok_and(|status| status.success())
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        std::process::Command::new("which")
            .arg("osqueryi")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .is_ok_and(|status| status.success())
    }
}