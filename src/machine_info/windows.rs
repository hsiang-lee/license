#[cfg(windows)]
use crate::LicenseError;
#[cfg(windows)]
use crate::machine_info::MachineInfoProvider;
#[cfg(windows)]
use std::ffi::OsString;
#[cfg(windows)]
use std::os::windows::ffi::OsStringExt;
#[cfg(windows)]
use std::ptr;
#[cfg(windows)]
use winapi::{
    shared::ifdef::NET_IF_CONNECTION_DEDICATED,
    shared::iptypes::IP_ADAPTER_ADDRESSES_LH,
    shared::iptypes::IP_ADAPTER_UNICAST_ADDRESS_LH,
    shared::winerror::ERROR_BUFFER_OVERFLOW,
    um::iphlpapi::{GetAdaptersAddresses, GAA_FLAG_INCLUDE_PREFIX},
    um::winnt::AF_UNSPEC,
};

#[derive(Debug)]
pub struct WindowsMachineInfo;

impl WindowsMachineInfo {
    pub fn new() -> Self {
        WindowsMachineInfo
    }

    fn get_first_mac_address(&self) -> Result<String, LicenseError> {
        unsafe {
            let mut size = 0;
            let mut result = String::new();

            // First call to get buffer size
            let ret = GetAdaptersAddresses(
                AF_UNSPEC as u32,
                GAA_FLAG_INCLUDE_PREFIX,
                ptr::null_mut(),
                ptr::null_mut(),
                &mut size,
            );

            if ret != ERROR_BUFFER_OVERFLOW {
                return Err(LicenseError::MachineIdMismatch);
            }

            let mut buffer = vec![0u8; size as usize];
            let mut adapters = buffer.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH;

            // Second call to get actual data
            if GetAdaptersAddresses(
                AF_UNSPEC as u32,
                GAA_FLAG_INCLUDE_PREFIX,
                ptr::null_mut(),
                adapters,
                &mut size,
            ) != 0
            {
                return Err(LicenseError::MachineIdMismatch);
            }

            while !adapters.is_null() {
                let adapter = *adapters;

                // Only use physical adapters with Ethernet type
                if adapter.OperStatus == NET_IF_CONNECTION_DEDICATED as u32
                    && adapter.IfType == 6 // IF_TYPE_ETHERNET_CSMACD
                {
                    let mac_slice = std::slice::from_raw_parts(
                        adapter.PhysicalAddress.as_ptr(),
                        adapter.PhysicalAddressLength as usize,
                    );

                    if !mac_slice.is_empty() {
                        result = mac_slice
                            .iter()
                            .map(|b| format!("{:02X}", b))
                            .collect::<String>();
                        break;
                    }
                }

                adapters = adapter.Next;
            }

            if result.is_empty() {
                Err(LicenseError::MachineIdMismatch)
            } else {
                Ok(result)
            }
        }
    }
}

impl MachineInfoProvider for WindowsMachineInfo {
    fn get_cpu_id(&self) -> Result<String, LicenseError> {
        unsafe {
            let mut cpu_info = [0; 4];
            winapi::um::intrin::__cpuid(&mut cpu_info, 1);
            Ok(format!("{:08X}", cpu_info[3])) // CPU signature
        }
    }

    fn get_machine_id(&self) -> Result<String, LicenseError> {
        self.get_first_mac_address()
    }
}