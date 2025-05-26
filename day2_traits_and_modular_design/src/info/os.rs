use sysinfo::System;

use crate::traits::InfoProvider;

pub struct OsInfo;

impl InfoProvider for OsInfo {
    fn key(&self) -> &'static str {
        "OS"
    }

    fn value(&self) -> String {
        let os_name = System::name().unwrap_or_else(|| "Unknown".into());
        let os_version = System::os_version().unwrap_or_else(|| "N/A".into());
        format!("{os_name} {os_version}")
    }
}
