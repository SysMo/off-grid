use serde::{Serialize, Deserialize};
use morse::{
  data::{data_point::DataPointBuilder, DataPoint, DataPointEncoder}, 
  protocols::modbus::register_types::{MbBooleanArray16, MbFloat32, MbInt16, ModbusDecode, ModbusEncode}
};
use morse::declare_modbus_structure;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct ElectricityMeasurement {
  _v1: MbInt16,
  _v2: MbInt16,
  _v3: MbInt16,
}

impl ElectricityMeasurement {
  pub fn v1(&self) -> f64 {
    let value: i32 = (&self._v1).into();
    (value as f64) / 10.0
  }
  pub fn v2(&self) -> f64 {
    let value: i32 = (&self._v2).into();
    (value as f64) / 10.0
  }
  pub fn v3(&self) -> f64 {
    let value: i32 = (&self._v3).into();
    (value as f64) / 10.0
  }
}

impl ModbusEncode for ElectricityMeasurement {
  
}

impl ModbusDecode for ElectricityMeasurement {
  fn reg_len() -> u16 {
    3 * MbInt16::reg_len()
  }
}

impl DataPointEncoder for ElectricityMeasurement {
  fn encode_struct(&self) -> Vec<DataPointBuilder> {
    let measurements = DataPoint::builder()
      .tag("_type", "Measurement")
      .float_field("v1", self.v1())
      .float_field("v2", self.v2())
      .float_field("v3", self.v3())
      ;
    vec![measurements]
  }
}

declare_modbus_structure!(ElectricityMeasurement);
  