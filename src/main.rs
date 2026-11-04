//! SigmaOS-api-gateway - SigmaOS Component

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting SigmaOS-api-gateway");
    println!("SigmaOS SigmaOS-api-gateway v0.1.0");
    Ok(())
}
