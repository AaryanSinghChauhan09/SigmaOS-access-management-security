//! SigmaOS-access-management-security — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_access_management_security_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-access-management-security: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-access-management-security: ok");
}
