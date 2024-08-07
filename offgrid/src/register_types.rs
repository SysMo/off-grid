use serde::{Serialize, Deserialize};
use morse::{
  data::{data_point::DataPointBuilder, DataPoint, DataPointEncoder}, 
  protocols::modbus::register_types::{MbInt16, MbInt32, ModbusDecode, ModbusEncode}
};
use morse::declare_modbus_structure;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ElectricityMeasurement {
  _v1: MbInt16,
  _v2: MbInt16,
  _v3: MbInt16,
  _i1: MbInt16,
  _i2: MbInt16,
  _i3: MbInt16,
  _blank1: [MbInt16; 2],
  _p1: MbInt16,
  _p2: MbInt16,
  _p3: MbInt16,
  _blank2: MbInt16,
  _q1: MbInt16,
  _q2: MbInt16,
  _q3: MbInt16,
  _blank3: [MbInt16; 5],
  _pf1: MbInt16,
  _pf2: MbInt16,
  _pf3: MbInt16,
  _blank4: [MbInt16; 3],
  _f: MbInt16
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnergyMeasurement {
  _p_tot: MbInt32,
  _blank1: [MbInt16; 8],
  _p_pos: MbInt32,
  _blank2: [MbInt16; 8],
  _p_rev: MbInt32,
  _blank3: [MbInt16; 8],
  _q_tot: MbInt32,
  _blank4: [MbInt16; 8],
  _q_pos: MbInt32,
  _blank5: [MbInt16; 8],
  _q_rev: MbInt32,
}

impl ElectricityMeasurement {
  pub fn v1(&self) -> f64 {
    let value: f64 = (&self._v1).into();
    value / 10.0
  }
  pub fn v2(&self) -> f64 {
    let value: i32 = (&self._v2).into();
    (value as f64) / 10.0
  }
  pub fn v3(&self) -> f64 {
    let value: i32 = (&self._v3).into();
    (value as f64) / 10.0
  }
  //--------------------------------------
  pub fn i1(&self) -> f64 {
    let value: i32 = (&self._i1).into();
    (value as f64) / 10.0
  }
  pub fn i2(&self) -> f64 {
    let value: i32 = (&self._i2).into();
    (value as f64) / 10.0
  }
  pub fn i3(&self) -> f64 {
    let value: i32 = (&self._i3).into();
    (value as f64) / 10.0
  }
  //--------------------------------------
  pub fn p1(&self) -> f64 {
    let value: i32 = (&self._p1).into();
    value as f64
  }
  pub fn p2(&self) -> f64 {
    let value: i32 = (&self._p2).into();
    value as f64
  }
  pub fn p3(&self) -> f64 { 
    let value: i32 = (&self._p3).into();
    value as f64
  }
  //--------------------------------------
  pub fn q1(&self) -> f64 {
    let value: i32 = (&self._q1).into();
    value as f64
  }
  pub fn q2(&self) -> f64 {
    let value: i32 = (&self._q2).into();
    value as f64
  }
  pub fn q3(&self) -> f64 { 
    let value: i32 = (&self._q3).into();
    value as f64
  }
  //--------------------------------------
  pub fn pf1(&self) -> f64 {
    let value: i32 = (&self._pf1).into();
    (value as f64) / 1000.0
  }
  pub fn pf2(&self) -> f64 {
    let value: i32 = (&self._pf2).into();
    (value as f64) / 1000.0
  }
  pub fn pf3(&self) -> f64 {
    let value: i32 = (&self._pf3).into();
    (value as f64) / 1000.0
  }
  //--------------------------------------
  pub fn f(&self) -> f64 {
    let value: i32 = (&self._f).into();
    (value as f64) / 100.0
  }
}

impl ModbusEncode for ElectricityMeasurement {
  
}

impl ModbusDecode for ElectricityMeasurement {
  fn reg_len() -> u16 {
    27 * MbInt16::reg_len()
  }
}

impl DataPointEncoder for ElectricityMeasurement {
  fn encode_struct(&self) -> Vec<DataPointBuilder> {
    let measurements = DataPoint::builder()
      .tag("_type", "Measurement")
      .float_field("v1", self.v1())
      .float_field("v2", self.v2())
      .float_field("v3", self.v3())
      .float_field("i1", self.i1())
      .float_field("i2", self.i2())
      .float_field("i3", self.i3())
      .float_field("p1", self.p1())
      .float_field("p2", self.p2())
      .float_field("p3", self.p3())
      .float_field("q1", self.q1())
      .float_field("q2", self.q2())
      .float_field("q3", self.q3())
      .float_field("pf1", self.pf1())
      .float_field("pf2", self.pf2())
      .float_field("pf3", self.pf3())
      .float_field("f", self.f())
      ;
    vec![measurements]
  }
}

declare_modbus_structure!(ElectricityMeasurement);
  
impl  EnergyMeasurement{
  pub fn p_tot(&self) -> f64 {
    let value: i32 = (&self._p_tot).into();
    (value as f64) / 100.0
  }

  pub fn p_pos(&self) -> f64 {
    let value: i32 = (&self._p_pos).into();
    (value as f64) / 100.0
  }

  pub fn p_rev(&self) -> f64 {
    let value: i32 = (&self._p_rev).into();
    (value as f64) / 100.0
  }

  pub fn q_tot(&self) -> f64 {
    let value: i32 = (&self._q_tot).into();
    (value as f64) / 100.0
  }

  pub fn q_pos(&self) -> f64 {
    let value: i32 = (&self._q_pos).into();
    (value as f64) / 100.0
  }

  pub fn q_rev(&self) -> f64 {
    let value: i32 = (&self._q_rev).into();
    (value as f64) / 100.0
  }

}

impl ModbusEncode for EnergyMeasurement {
  
}

impl ModbusDecode for EnergyMeasurement {
  fn reg_len() -> u16 {
    52 * MbInt16::reg_len()
  }
}

impl DataPointEncoder for EnergyMeasurement {
  fn encode_struct(&self) -> Vec<DataPointBuilder> {
    let measurements = DataPoint::builder()
      .tag("_type", "Energy")
      .float_field("p_tot", self.p_tot())
      .float_field("p_pos", self.p_pos())
      .float_field("p_rev", self.p_rev())
      .float_field("q_tot", self.q_tot())
      .float_field("q_pos", self.q_pos())
      .float_field("q_rev", self.q_rev())
      ;
    vec![measurements]
  }
}

declare_modbus_structure!(EnergyMeasurement);
  