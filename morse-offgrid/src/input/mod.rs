use morse::{data::{FromBytes, TDataPoint}, protocols::modbus::{register_types::{MbBooleanArray16, MbFixed16, MbFloat32}, ModbusCodec}};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MbDTS777Electricity {
  pub v1: f32,
  pub v2: f32,
  pub v3: f32,

  pub i1: f32,
  pub i2: f32,
  pub i3: f32,

  // pub gap1: [MbBooleanArray16; 2],

  pub p1: f32,
  pub p2: f32,
  pub p3: f32,

  // pub gap2: [MbBooleanArray16; 9],

  pub pf1: f32,
  pub pf2: f32,
  pub pf3: f32,

  // pub gap3: [MbBooleanArray16; 3],

  pub f: f32,
}

impl TDataPoint for MbDTS777Electricity {
}


impl ModbusCodec for MbDTS777Electricity {
  fn reg_len() -> u16 {
      27 * MbFixed16::reg_len()
  }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MbDTS777Energy {

}

impl TDataPoint for MbDTS777Energy {
}


impl ModbusCodec for MbDTS777Energy {
  fn reg_len() -> u16 {
      52 * MbFixed16::reg_len()
  }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MbOffgridType {
  DTS777Electricity(MbDTS777Electricity),
  DTS777Energy(MbDTS777Energy)
}

impl TDataPoint for MbOffgridType {}

// impl FromBytes for MbOffgridType {
//   fn decode_data(tpe: &str, data: &Vec<u8>, config: &morse::protocols::modbus::ModbusCodecConfig) -> anyhow::Result<Vec<Self>> {
//     match tpe {
//       "DTS777Electricity" => {
//         let obj = MbDTS777Electricity::decode(data, config)?;
//         Ok(vec![MbOffgridType::DTS777Electricity(obj)])
//       },
//       unknown => anyhow::bail!("MbOffgridType decoder for {unknown} modbus data not found!")
//     }
//   }
// }
