pub trait IntoAnyhow<V, E> where E: std::fmt::Debug {
  fn into_anyhow(self) -> anyhow::Result<V>;
}

use embedded_hal_0_2::blocking::delay::{DelayUs, DelayMs};

pub struct StdDelay;

impl DelayMs<u16> for StdDelay {
  fn delay_ms(&mut self, ms: u16) {
    std::thread::sleep(std::time::Duration::from_millis(ms as u64));
  }
}

impl DelayUs<u16> for StdDelay {
  fn delay_us(&mut self, us: u16) {
    std::thread::sleep(std::time::Duration::from_micros(us as u64));
  }
}


