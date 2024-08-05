use morse::agent;
use offgrid::register_types as rt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    log::info!("Starting Morse");
    agent::Agent::run_from_yaml("config/morse_home.yaml").await?;
    // log::info!("{:#?}", agent.config);
    Ok(())
}
