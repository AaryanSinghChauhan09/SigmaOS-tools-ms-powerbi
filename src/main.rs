//! SigmaOS-tools-ms-powerbi — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_tools_ms_powerbi_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-tools-ms-powerbi: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-tools-ms-powerbi: ok");
}
