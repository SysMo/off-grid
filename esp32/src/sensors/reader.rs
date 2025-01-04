use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorDataSnapshot {
  power: f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReaderConfig {
  pub modbus: ModbusConfig
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModbusConfig {
  
}


pub struct SensorReader {
  config: SensorReaderConfig
}

impl SensorReader {
  pub fn new(config: SensorReaderConfig) -> Self {
    Self { config }
  }
  pub async fn read(&self) -> anyhow::Result<SensorDataSnapshot> {
    Ok(SensorDataSnapshot { power: 34.3 })
  }
}