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
        
        Command::new("osqueryi")
            .arg("--version")
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .is_ok()
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Command::new("osqueryi")
            .arg("--version")
            .output()
            .is_ok()
    }
}