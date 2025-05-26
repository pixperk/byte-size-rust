use sysinfo::System;

use crate::traits::InfoProvider;

pub struct CpuInfo;

impl InfoProvider for CpuInfo {
    fn key(&self) -> &'static str {
        "CPU"
    }

    fn value(&self) -> String {
        let mut system = System::new();
        system.refresh_cpu();

        if let Some(cpu) = system.cpus().first() {
            let brand = cpu.brand();
            let cores_count = system.cpus().len();
            format!("{} ({} cores)", brand, cores_count)
        } else {
            "Unknown CPU".to_string()
        }
    }
}
