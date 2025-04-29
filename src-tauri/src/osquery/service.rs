// src-tauri/src/osquery/service.rs
use anyhow::{Context, Result};
use log::info;
use serde_json::Value;
use std::io::Read;
use std::process::{Command, Stdio};

pub struct OsqueryService;

impl OsqueryService {
    pub fn query(&self, query: &str) -> Result<Value> {
        info!("Executing osquery: {}", query);

        let mut cmd = Command::new("osqueryi")
            .arg("--json")
            .arg(query)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to start osqueryi")?;

        let mut stdout = cmd.stdout.take().unwrap();
        let mut output = String::new();
        stdout.read_to_string(&mut output)?;

        let status = cmd.wait()?;
        if !status.success() {
            anyhow::bail!("Osquery failed with status: {}", status);
        }

        let result: Value =
            serde_json::from_str(&output).context("Failed to parse osquery output")?;

        Ok(result)
    }
}
