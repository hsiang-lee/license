use super::MachineInfoProvider;
use crate::LicenseError;
use std::{
    fs,
    path::Path,
};

#[derive(Debug)]
pub struct LinuxMachineInfo;

impl LinuxMachineInfo {
    pub fn new() -> Self {
        LinuxMachineInfo
    }

    fn get_first_mac_address(&self) -> Result<String, LicenseError> {
        let net_devices = ["eth0", "em1", "eno1", "ib0"];
        for device in net_devices.iter() {
            let path = format!("/sys/class/net/{}/address", device);
            if Path::new(&path).exists() {
                if let Ok(mac) = fs::read_to_string(&path) {
                    let trimmed = mac.trim().replace(':', "");
                    if !trimmed.is_empty() {
                        return Ok(trimmed);
                    }
                }
            }
        }
        Err(LicenseError::MachineIdMismatch)
    }
}

impl MachineInfoProvider for LinuxMachineInfo {
    fn get_cpu_id(&self) -> Result<String, LicenseError> {
        if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
            for line in cpuinfo.lines() {
                if line.starts_with("serial") {
                    if let Some(serial) = line.split(':').nth(1) {
                        return Ok(serial.trim().to_string());
                    }
                }
            }
        }
        // Fallback to MAC address if CPU serial not available
        self.get_first_mac_address()
    }

    fn get_machine_id(&self) -> Result<String, LicenseError> {
        // Try /etc/machine-id first
        if let Ok(machine_id) = fs::read_to_string("/etc/machine-id") {
            let trimmed = machine_id.trim();
            if !trimmed.is_empty() {
                return Ok(trimmed.to_string());
            }
        }

        // Fall back to MAC address
        self.get_first_mac_address()
    }
}