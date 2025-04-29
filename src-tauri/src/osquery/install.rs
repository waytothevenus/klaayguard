// src-tauri/src/osquery/install.rs
use anyhow::{Context, Result};
use log::info;
use std::process::Command;

#[cfg(target_os = "windows")]
pub fn install_osquery() -> Result<()> {
    info!("Installing osquery on Windows");
    Command::new("choco")
        .args(&["install", "osquery", "-y"])
        .status()
        .context("Failed to install via Chocolatey")?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn install_osquery() -> Result<()> {
    info!("Installing osquery on macOS");
    Command::new("brew")
        .args(&["install", "osquery"])
        .status()
        .context("Failed to install via Homebrew")?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn install_osquery() -> Result<()> {
    info!("Installing osquery on Linux");
    let status = Command::new("sh")
        .arg("-c")
        .arg("curl -L https://pkg.osquery.io/deb/osquery_5.10.2-1.linux_amd64.deb -o /tmp/osquery.deb && sudo dpkg -i /tmp/osquery.deb")
        .status()
        .context("Failed to install osquery")?;

    if !status.success() {
        error!("Osquery installation failed");
        anyhow::bail!("Installation failed with status: {}", status);
    }
    Ok(())
}

pub fn is_osquery_installed() -> bool {
    Command::new("osqueryi").arg("--version").output().is_ok()
}
