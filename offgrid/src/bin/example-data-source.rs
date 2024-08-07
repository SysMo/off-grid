use morse::protocols::modbus::{register_types::MbInt16, ModbusDataClient};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
  log::info!("Starting Modbus data source");

  //
  let mut client = ModbusDataClient::connect_tcp("127.0.0.1", 9872).await?;
  // test_simple_registers(&mut client).await?;
  set_registers(&mut client).await?;
  log::info!("Wrote data");
  client.disconnect().await?;
  
  // tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
  Ok(())
}

async fn set_registers(client: &mut ModbusDataClient) -> anyhow::Result<()> {
    client.write_struct(0, &MbInt16(2320)).await?;
    client.write_struct(1, &MbInt16(2310)).await?;
    client.write_struct(2, &MbInt16(2340)).await?;
    Ok(())
}