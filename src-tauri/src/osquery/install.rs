// src-tauri/src/osquery/install.rs
use anyhow::{Result};
use std::process::Command;
use log::{info};
use runas::Command as SudoCommand;
use reqwest::blocking::get;
use std::fs::File;
use std::io::copy;


#[cfg(target_os = "windows")]
pub fn install_osquery() -> Result<()> {
    use std::os::windows::process::CommandExt;
    use log::{info, warn, error};
    
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    info!("Preparing osquery installation on Windows");

    // Check if Chocolatey is installed and available in PATH
    let choco_installed = Command::new("where")
        .arg("choco")
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok();

    if !choco_installed {
        warn!("Chocolatey not found. Installing Chocolatey first...");
        
        // Install Chocolatey with admin privileges
        let status = Command::new("powershell")
            .args(&[
                "-NoProfile",
                "-ExecutionPolicy", "Bypass",
                "-Command",
                "[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; \
                iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))"
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .status()
            .context("Failed to install Chocolatey. Try running as Administrator")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Chocolatey installation failed with status: {}", status));
        }

        info!("Waiting for Chocolatey to initialize...");
        std::thread::sleep(std::time::Duration::from_secs(5));

        // Refresh environment variables
        info!("Refreshing environment variables...");
        let _ = Command::new("cmd")
            .args(&["/C", "refreshenv"])
            .creation_flags(CREATE_NO_WINDOW)
            .status();
    }

    // Verify choco is now available
    info!("Verifying Chocolatey installation...");
    let choco_version = Command::new("choco")
        .arg("--version")
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .context("Failed to verify Chocolatey installation")?;

    if !choco_version.status.success() {
        error!("Chocolatey verification failed. Output: {:?}", choco_version);
        return Err(anyhow::anyhow!("Chocolatey installation verification failed"));
    }

    info!("Chocolatey version: {}", String::from_utf8_lossy(&choco_version.stdout));

    info!("Installing osquery via Chocolatey");
    
    // Install osquery with Chocolatey with retry logic
    let mut attempts = 0;
    let max_attempts = 3;
    let mut last_error = None;

    while attempts < max_attempts {
        attempts += 1;
        info!("Attempt {} of {} to install osquery", attempts, max_attempts);

        let status = Command::new("choco")
            .args(&["install", "osquery", "-y", "--force", "--no-progress"])
            .creation_flags(CREATE_NO_WINDOW)
            .status();

        match status {
            Ok(status) if status.success() => {
                info!("osquery installation completed via Chocolatey");
                return Ok(());
            },
            Ok(status) => {
                last_error = Some(format!("Chocolatey exited with status: {}", status));
                warn!("Attempt {} failed: {}", attempts, last_error.as_ref().unwrap());
            },
            Err(e) => {
                last_error = Some(format!("Failed to execute choco command: {}", e));
                warn!("Attempt {} failed: {}", attempts, last_error.as_ref().unwrap());
            }
        }

        if attempts < max_attempts {
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    }

    Err(anyhow::anyhow!(
        "Failed to install osquery after {} attempts. Last error: {}",
        max_attempts,
        last_error.unwrap_or_else(|| "unknown error".to_string())
    ))
}

#[cfg(target_os = "macos")]
pub fn install_osquery() -> Result<()> {
    let url = "https://pkg.osquery.io/darwin/osquery-5.17.0.pkg";
    let output_path = "/tmp/osquery-5.17.0.pkg";

    // Download the osquery package
    info!("Downloading osquery package...");
    let mut response = get(url)?;

    if response.status().is_success() {
        // Write the downloaded file to disk
        let mut out = File::create(output_path)?;
        copy(&mut response, &mut out)?;

        // Install the package using system command
        info!("Installing osquery...");
        let status = SudoCommand::new("installer")
            .gui(true)
            .force_prompt(true)
            .arg("-pkg")
            .arg(output_path)
            .arg("-target")
            .arg("/")
            .status()?;

        if status.success() {
            info!("osquery installation successful.");
        } else {
            return Err(anyhow::anyhow!("osquery installation failed."));
        }

        // Clean up by removing the downloaded .pkg file
        std::fs::remove_file(output_path)?;

        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to download osquery package."))
    }
}
enum LinuxPackageManager {
    Apt,
    Dnf,
    Zypper,
}

#[cfg(target_os = "linux")]
fn get_package_manager() -> Result<LinuxPackageManager> {
    // debian based
    let apt_installed = Command::new("which")
        .arg("apt")
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if apt_installed {
        info!("detected apt");
        return Ok(LinuxPackageManager::Apt);
    }

    // fedora based
    let dnf_installed= Command::new("which")
        .arg("dnf")
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if dnf_installed {
        info!("detected dnf");
        return Ok(LinuxPackageManager::Dnf);
    }

    // suse based
    let zypper_installed= Command::new("which")
        .arg("zypper")
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if zypper_installed {
        info!("detected zypper");
        return Ok(LinuxPackageManager::Zypper);
    }

    Err(anyhow::anyhow!("Couldn't find supported package manager"))
}

#[cfg(target_os = "linux")]
fn configure_osquery_repo(package_manager: &LinuxPackageManager) -> Result<()> {
    match package_manager {
        LinuxPackageManager::Apt => {
            Command::new("sudo")
                .args(&["mkdir", "-p", "/etc/apt/keyrings"])
                .status()
                .and_then(|s| {
                    if s.success() {
                        Command::new("sh")
                            .arg("-c")
                            .arg("curl -L https://pkg.osquery.io/deb/pubkey.gpg | sudo tee /etc/apt/keyrings/osquery.asc")
                            .status()
                    } else {
                        Ok(s)
                    }
                })
                .and_then(|s| {
                    if s.success() {
                    Command::new("sudo")
                            .args(&[
                                "add-apt-repository",
                                "deb [arch=amd64 signed-by=/etc/apt/keyrings/osquery.asc] https://pkg.osquery.io/deb deb main",
                            ])
                        .status()
                    } else {
                        Ok(s)
                    }
                })
                .map_err(|e| anyhow::anyhow!("Failed to configure apt repo: {}", e))
                .and_then(|s| {
                    if s.success() {
                        Ok(())
                    } else {
                        Err(anyhow::anyhow!("Failed to configure apt repo"))
                    }
                })
        },
        LinuxPackageManager::Dnf => {
            Command::new("sh")
                .arg("-c")
                .arg("curl -L https://pkg.osquery.io/rpm/GPG | sudo tee /etc/pki/rpm-gpg/RPM-GPG-KEY-osquery")
                .status()
                .and_then(|s| {
                    if s.success() {
                        Command::new("sudo")
                            .args(&["yum-config-manager", "--add-repo", "https://pkg.osquery.io/rpm/osquery-s3-rpm.repo"])
                            .status()
                    } else {
                        Ok(s)
                    }
                })
                .and_then(|s| {
                    if s.success() {
            Command::new("sudo")
                            .args(&["yum-config-manager", "--enable", "osquery-s3-rpm-repo"])
                            .status()
                    } else {
                        Ok(s)
                    }
                })
                .map_err(|e| anyhow::anyhow!("Failed to configure yum/dnf repo: {}", e))
                .and_then(|s| {
                    if s.success() {
                        Ok(())
                    } else {
                        Err(anyhow::anyhow!("Failed to configure yum/dnf repo"))
                    }
                })
        },
        LinuxPackageManager::Zypper => {
            Command::new("sh")
                .arg("-c")
                .arg("curl -L https://pkg.osquery.io/rpm/GPG | sudo tee /etc/pki/rpm-gpg/RPM-GPG-KEY-osquery")
                .status()
                .and_then(|s| {
                    if s.success() {
                        Command::new("sudo")
                            .args(&["zypper", "ar", "-f", "https://pkg.osquery.io/rpm/osquery-s3-rpm.repo"])
                            .status()
                    } else {
                        Ok(s)
                    }
                })
                .and_then(|s| {
                    if s.success() {
            Command::new("sudo")
                            .args(&["zypper", "mr", "-e", "osquery-s3-rpm-repo"])
                .status()
        } else {
                        Ok(s)
                    }
                })
                .map_err(|e| anyhow::anyhow!("Failed to configure zypper repo: {}", e))
                .and_then(|s| {
                    if s.success() {
                        Ok(())
                    } else {
                        Err(anyhow::anyhow!("Failed to configure zypper repo"))
                    }
                })
        }
    }
}

#[cfg(target_os = "linux")]
pub fn install_osquery() -> Result<()> {
    info!("Preparing osquery installation on Linux");

    let package_manager = get_package_manager()?;

    // Configure the osquery repository before installation
    configure_osquery_repo(&package_manager)?;

    // Install osquery with retry logic
    let mut attempts = 0;
    let max_attempts = 3;
    let mut last_error = None;

    while attempts < max_attempts {
        attempts += 1;
        info!(
            "Attempt {} of {} to install osquery",
            attempts, max_attempts
        );

        let osquery_install_status = match package_manager {
            LinuxPackageManager::Apt => Command::new("sudo")
                .args(&["apt", "install", "-y", "osquery"])
                .status(),
            LinuxPackageManager::Dnf => Command::new("sudo")
                .args(&["yum", "install", "-y", "osquery"])
                .status(),
            LinuxPackageManager::Zypper => Command::new("sudo")
                .args(&["zypper", "--non-interactive", "install", "osquery"])
                .status(),
        };

        match osquery_install_status {
            Ok(status) if status.success() => {
                info!("osquery installation completed successfully");
                return Ok(());
            }
            Ok(status) => {
                last_error = Some(format!("Installation failed with status: {}", status));
                warn!(
                    "Attempt {} failed: {}",
                    attempts,
                    last_error.as_ref().unwrap()
                );
            }
            Err(e) => {
                last_error = Some(format!("Failed to execute installation command: {}", e));
                warn!(
                    "Attempt {} failed: {}",
                    attempts,
                    last_error.as_ref().unwrap()
                );
            }
        }

        if attempts < max_attempts {
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    }

    Err(anyhow::anyhow!(
        "Failed to install osquery after {} attempts. Last error: {}",
        max_attempts,
        last_error.unwrap_or_else(|| "unknown error".to_string())
    ))
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
