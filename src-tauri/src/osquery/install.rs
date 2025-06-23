// src-tauri/src/osquery/install.rs
use anyhow::{Context, Result};
use std::process::Command;
use log::{info, warn, error};


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
    
    info!("Preparing osquery installation on macOS");

    // Check if Homebrew is installed
    let brew_installed = Command::new("which")
        .arg("brew")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok();

    if !brew_installed {
        warn!("Homebrew not found. Installing Homebrew first...");
        
        // Install Homebrew
        let status = Command::new("/bin/bash")
            .arg("-c")
            .arg("curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh | bash")
            .status()
            .context("Failed to install Homebrew")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Homebrew installation failed with status: {}", status));
        }

        info!("Waiting for Homebrew to initialize...");
        std::thread::sleep(std::time::Duration::from_secs(5));

        // Add Homebrew to PATH if not already there
        info!("Ensuring Homebrew is in PATH...");
        let _ = Command::new("/bin/bash")
            .arg("-c")
            .arg("echo 'eval \"$(/opt/homebrew/bin/brew shellenv)\"' >> ~/.zshrc && source ~/.zshrc")
            .status();
    }

    // Verify brew is now available
    info!("Verifying Homebrew installation...");
    let brew_version = Command::new("brew")
        .arg("--version")
        .output()
        .context("Failed to verify Homebrew installation")?;

    if !brew_version.status.success() {
        error!("Homebrew verification failed. Output: {:?}", brew_version);
        return Err(anyhow::anyhow!("Homebrew installation verification failed"));
    }

    info!("Homebrew version: {}", String::from_utf8_lossy(&brew_version.stdout));

    info!("Installing osquery via Homebrew");
    
    // Install osquery with Homebrew with retry logic
    let mut attempts = 0;
    let max_attempts = 3;
    let mut last_error = None;

    while attempts < max_attempts {
        attempts += 1;
        info!("Attempt {} of {} to install osquery", attempts, max_attempts);

        let status = Command::new("brew")
            .env("HOMEBREW_NO_AUTO_UPDATE", "1")
            .args(&["install", "osquery", "--quiet"])
            .status();

        match status {
            Ok(status) if status.success() => {
                info!("osquery installation completed via Homebrew");
                return Ok(());
            },
            Ok(status) => {
                last_error = Some(format!("Homebrew exited with status: {}", status));
                warn!("Attempt {} failed: {}", attempts, last_error.as_ref().unwrap());
            },
            Err(e) => {
                last_error = Some(format!("Failed to execute brew command: {}", e));
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

            Command::new("sudo")
                .args(&["apt-get", "update"])
                .status()
                .and_then(|_| {
                    Command::new("sudo")
                        .args(&["apt-get", "install", "-y", "curl"])
                        .status()
                })
        } else if Command::new("which").arg("yum").status().is_ok() {
            Command::new("sudo")
                .args(&["yum", "install", "-y", "curl"])
                .status()
        } else if Command::new("which").arg("dnf").status().is_ok() {
            Command::new("sudo")
                .args(&["dnf", "install", "-y", "curl"])
                .status()
        } else if Command::new("which").arg("zypper").status().is_ok() {
            Command::new("sudo")
                .args(&["zypper", "install", "-y", "curl"])
                .status()
        } else {
            return Err(anyhow::anyhow!("Could not determine package manager to install curl"));
        };

        match status {
            Ok(status) if !status.success() => {
                return Err(anyhow::anyhow!("curl installation failed with status: {}", status));
            },
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to install curl: {}", e));
            },
            _ => () // Success
        }

        info!("curl installed successfully");
    }

    // Install osquery with retry logic
    let mut attempts = 0;
    let max_attempts = 3;
    let mut last_error = None;
    let osquery_version = "5.10.2"; // You might want to make this configurable

    while attempts < max_attempts {
        attempts += 1;
        info!("Attempt {} of {} to install osquery", attempts, max_attempts);

        let status = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "curl -sSL https://pkg.osquery.io/deb/osquery_{}-1.linux_amd64.deb -o /tmp/osquery.deb && \
                sudo dpkg -i /tmp/osquery.deb || sudo apt-get install -f -y",
                osquery_version
            ))
            .status();

        match status {
            Ok(status) if status.success() => {
                info!("osquery installation completed successfully");
                // Clean up the downloaded package
                let _ = Command::new("rm").arg("-f").arg("/tmp/osquery.deb").status();
                return Ok(());
            },
            Ok(status) => {
                last_error = Some(format!("Installation failed with status: {}", status));
                warn!("Attempt {} failed: {}", attempts, last_error.as_ref().unwrap());
            },
            Err(e) => {
                last_error = Some(format!("Failed to execute installation command: {}", e));
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
