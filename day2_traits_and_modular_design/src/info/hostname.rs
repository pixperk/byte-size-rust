use sysinfo::System;

use crate::traits::InfoProvider;

pub struct HostnameInfo;

impl InfoProvider for HostnameInfo {
    fn key(&self) -> &'static str {
        "Hostname"
    }

    fn value(&self) -> String {
        System::host_name().unwrap_or_else(|| "Unknown".into())
    }
}
