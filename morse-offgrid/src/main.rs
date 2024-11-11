use morse::agent::AsyncSystem;
use morse::util::config::LogConfig;
use system::offgrid::OffgridSystem;

mod system;
mod input;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    LogConfig::console()?;
    log::info!("Starting Morse");
    AsyncSystem::<OffgridSystem>::run_from_yaml("config/offgrid.yaml").await?;
    Ok(())
}