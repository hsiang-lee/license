use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PlatformError {
    UnsupportedPlatform,
    CommandFailed(String),
}

impl fmt::Display for PlatformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlatformError::UnsupportedPlatform => write!(f, "Unsupported platform"),
            PlatformError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
        }
    }
}

impl Error for PlatformError {}

pub fn get_machine_info() -> Result<String, PlatformError> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        let output = Command::new("wmic")
            .args(&["csproduct", "get", "uuid"])
            .output()
            .map_err(|e| PlatformError::CommandFailed(e.to_string()))?;

        if output.status.success() {
            let uuid = String::from_utf8_lossy(&output.stdout)
                .trim()
                .lines()
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            
            if !uuid.is_empty() {
                return Ok(uuid);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        let output = Command::new("ioreg")
            .args(&["-rd1", "-c", "IOPlatformExpertDevice"])
            .output()
            .map_err(|e| PlatformError::CommandFailed(e.to_string()))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some(uuid) = stdout
                .lines()
                .find(|line| line.contains("IOPlatformUUID"))
                .and_then(|line| line.split('=').nth(1))
                .map(|s| s.trim().trim_matches('"'))
            {
                return Ok(uuid.to_string());
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::fs;

        if let Ok(machine_id) = fs::read_to_string("/etc/machine-id") {
            return Ok(machine_id.trim().to_string());
        }

        if let Ok(product_uuid) = fs::read_to_string("/sys/class/dmi/id/product_uuid") {
            return Ok(product_uuid.trim().to_string());
        }
    }

    Err(PlatformError::UnsupportedPlatform)
}