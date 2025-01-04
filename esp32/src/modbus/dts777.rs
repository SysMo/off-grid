use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MbDTS777Electricity {
  pub v1: u16,
  pub v2: u16,
  pub v3: u16,

  pub i1: u16,
  pub i2: u16,
  pub i3: u16,

  pub gap1: [u16; 2],

  pub p1: u16,
  pub p2: u16,
  pub p3: u16,

  pub gap2: [u16; 9],

  pub pf1: u16,
  pub pf2: u16,
  pub pf3: u16,

  pub gap3: [u16; 3],

  pub f: u16,
}

