use crate::LicenseError;
use std::fmt;

pub trait MachineInfoProvider: fmt::Debug {
    fn get_cpu_id(&self) -> Result<String, LicenseError>;
    fn get_machine_id(&self) -> Result<String, LicenseError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        let platform = Platform::detect();
        #[cfg(target_os = "linux")]
        assert!(matches!(platform, Platform::Linux));
        
        #[cfg(target_os = "windows")]
        assert!(matches!(platform, Platform::Windows));
        
        #[cfg(target_os = "macos")]
        assert!(matches!(platform, Platform::MacOS));
    }

    #[test]
    fn test_machine_id_not_empty() {
        let provider = create_provider().unwrap();
        let machine_id = provider.get_machine_id().unwrap();
        assert!(!machine_id.is_empty());
    }

    #[test]
    fn test_machine_id_consistent() {
        let provider = create_provider().unwrap();
        let id1 = provider.get_machine_id().unwrap();
        let id2 = provider.get_machine_id().unwrap();
        assert_eq!(id1, id2);
    }
}

pub enum Platform {
    Linux,
    Windows,
    MacOS,
    Unknown,
}

impl Platform {
    pub fn detect() -> Self {
        if cfg!(target_os = "linux") {
            Platform::Linux
        } else if cfg!(target_os = "windows") {
            Platform::Windows
        } else if cfg!(target_os = "macos") {
            Platform::MacOS
        } else {
            Platform::Unknown
        }
    }
}

pub fn create_provider() -> Result<Box<dyn MachineInfoProvider>, LicenseError> {
    match Platform::detect() {
        Platform::Linux => Ok(Box::new(LinuxMachineInfo::new())),
        #[cfg(not(target_os = "windows"))]
        Platform::Windows => Err(LicenseError::UnsupportedPlatform("Windows".to_string())),
        #[cfg(not(target_os = "macos"))]
        Platform::MacOS => Err(LicenseError::UnsupportedPlatform("macOS".to_string())),
        Platform::Unknown => Err(LicenseError::UnsupportedPlatform(
            std::env::consts::OS.to_string(),
        )),
    }
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "macos")]
mod mac;

#[cfg(target_os = "linux")]
use linux::LinuxMachineInfo;
#[cfg(target_os = "windows")]
use windows::WindowsMachineInfo;
#[cfg(target_os = "macos")]
use mac::MacMachineInfo;