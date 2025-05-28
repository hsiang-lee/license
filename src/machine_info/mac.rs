use super::MachineInfoProvider;
use crate::LicenseError;
use std::process::Command;

#[derive(Debug)]
pub struct MacMachineInfo;

impl MacMachineInfo {
    pub fn new() -> Self {
        MacMachineInfo
    }

    fn get_hardware_uuid(&self) -> Result<String, LicenseError> {
        let output = Command::new("ioreg")
            .arg("-rd1")
            .arg("-c")
            .arg("IOPlatformExpertDevice")
            .output()
            .map_err(|_| LicenseError::MachineIdMismatch)?;

        if !output.status.success() {
            return Err(LicenseError::MachineIdMismatch);
        }

        let stdout = String::from_utf8(output.stdout)
            .map_err(|_| LicenseError::MachineIdMismatch)?;

        for line in stdout.lines() {
            if line.contains("IOPlatformUUID") {
                if let Some(uuid) = line.split('=').nth(1) {
                    return Ok(uuid.trim().trim_matches('"').to_string());
                }
            }
        }

        Err(LicenseError::MachineIdMismatch)
    }

    fn get_first_mac_address(&self) -> Result<String, LicenseError> {
        let output = Command::new("ifconfig")
            .arg("en0")
            .output()
            .map_err(|_| LicenseError::MachineIdMismatch)?;

        if !output.status.success() {
            return Err(LicenseError::MachineIdMismatch);
        }

        let stdout = String::from_utf8(output.stdout)
            .map_err(|_| LicenseError::MachineIdMismatch)?;

        for line in stdout.lines() {
            if line.contains("ether") {
                if let Some(mac) = line.split_whitespace().nth(1) {
                    return Ok(mac.replace(':', ""));
                }
            }
        }

        Err(LicenseError::MachineIdMismatch)
    }
}

impl MachineInfoProvider for MacMachineInfo {
    fn get_cpu_id(&self) -> Result<String, LicenseError> {
        let output = Command::new("sysctl")
            .arg("-n")
            .arg("machdep.cpu.signature")
            .output()
            .map_err(|_| LicenseError::MachineIdMismatch)?;

        if !output.status.success() {
            return Err(LicenseError::MachineIdMismatch);
        }

        let signature = String::from_utf8(output.stdout)
            .map_err(|_| LicenseError::MachineIdMismatch)?
            .trim()
            .to_string();

        Ok(signature)
    }

    fn get_machine_id(&self) -> Result<String, LicenseError> {
        // Try hardware UUID first
        if let Ok(uuid) = self.get_hardware_uuid() {
            return Ok(uuid);
        }

        // Fall back to MAC address
        self.get_first_mac_address()
    }
}