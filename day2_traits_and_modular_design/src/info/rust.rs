use crate::traits::InfoProvider;

pub struct RustInfo;

impl InfoProvider for RustInfo {
    fn key(&self) -> &'static str {
        "Rust"
    }

    fn value(&self) -> String {
        format!("{}", rustc_version_runtime::version())
    }
}
