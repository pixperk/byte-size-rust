use clap::Parser;
mod traits;
mod info;

use crate::traits::InfoProvider;
use crate::info::os::OsInfo;
use crate::info::rust::RustInfo;
use crate::info::hostname::HostnameInfo;
use crate::info::cpu::CpuInfo;

#[derive(Parser, Debug)]
#[command(author, version, about = "Lessgo Info CLI")]
struct Cli {
    #[arg(long, help = "Show OS info")]
    os: bool,

    #[arg(long, help = "Show Rust version")]
    rust: bool,

    #[arg(long, help = "Show Hostname")]
    hostname: bool,

    #[arg(long, help = "Show CPU info")]
    cpu: bool,
}

fn main() {
    let args = Cli::parse();

    let providers: Vec<(Box<dyn InfoProvider>, bool)> = vec![
        (Box::new(OsInfo), args.os),
        (Box::new(RustInfo), args.rust),
        (Box::new(HostnameInfo), args.hostname),
        (Box::new(CpuInfo), args.cpu),
    ];

    let any_flag = args.os || args.rust || args.hostname || args.cpu;

    for (provider, enabled) in providers {
        if any_flag && !enabled {
            continue;
        }
        println!("{}: {}", provider.key(), provider.value());
    }
}
