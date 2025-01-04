use std::ops::Add;

use ds18b20::{Ds18b20, read_scratchpad, SensorData, Resolution};
use embedded_hal_0_2::digital::v2::{InputPin, OutputPin};
use embedded_hal_0_2::blocking::delay::{DelayUs, DelayMs};
use one_wire_bus::{Address, OneWire, OneWireResult, OneWireError};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::util::IntoAnyhow;


impl<V, E> IntoAnyhow<V, E> for OneWireResult<V, E> 
where E: std::fmt::Debug {
  fn into_anyhow(self) -> anyhow::Result<V> {
    match self {
      Ok(v) => Ok(v),
      Err(e) => Err(anyhow::anyhow!("{:?}", e)),
    }
  }
}

pub struct DS18B20Array<P, E, D>
where P: OutputPin<Error = E> + InputPin<Error = E>, 
      E: std::fmt::Debug + std::fmt::Display + std::marker::Sync,
      D: DelayUs<u16> + DelayMs<u16>
{
  bus: OneWire<P>,
  delay: D,
  sensors: Vec<Ds18b20>,
  resolution: Resolution,
  alarm_high: i8,
  alarm_low: i8,
}

impl<P, E, D> DS18B20Array<P, E, D> 
where P: OutputPin<Error = E> + InputPin<Error = E>, 
      E: std::fmt::Debug + std::fmt::Display + std::marker::Sync,
      D: DelayUs<u16> + DelayMs<u16>
{
  pub fn new(pin: P, mut delay: D) -> anyhow::Result<Self> {
    let mut bus = OneWire::new(pin).into_anyhow()?;
    let mut remaining_retries = 10;
    let device_addresses: Vec<Address> = loop {
      if remaining_retries > 0 {
        match Self::find_devices(&mut bus, &mut delay) {
          Ok(x) => break x,
          Err(err) => {
            log::warn!("Failed to find devices: {err}");
            remaining_retries -= 1;
          }
        }  
      } else {
        anyhow::bail!("Failed to find devices");
      }
    };

    let sensors: Vec<Ds18b20> = device_addresses.iter().map(|d| 
      Ds18b20::new::<OneWireError<E>>(d.clone()).into_anyhow()
    ).collect::<Result<Vec<_>, _>>()?;

    let mut device_array = DS18B20Array {
      bus, delay, sensors, resolution: Resolution::Bits12,
      alarm_high: 100, alarm_low: 0
    };

    device_array.write_settings()?;
    
    Ok(device_array)
  }

  // fn read_config(&mut self) -> anyhow::Result<()> {
  //   let mut scratchpad = read_scratchpad(
  //     self.sensor.address(), &mut self.bus, &mut self.delay
  //   ).into_anyhow()?;    
  //   info!("{:?}", scratchpad);
  //   self.alarm_high = scratchpad[2];
  //   self.alarm_low = scratchpad[3];

  //   Ok(())
  // }
  pub fn set_resolution(&mut self, r: Resolution) -> anyhow::Result<()> {
    self.resolution = r;
    self.write_settings()
  }

  pub fn write_settings(&mut self) -> anyhow::Result<()> {
    for sensor in &self.sensors {
      sensor.set_config(self.alarm_low, self.alarm_high, self.resolution, 
        &mut self.bus, &mut self.delay
      ).into_anyhow()?;  
    }
    Ok(())
  }

  fn find_devices(one: &mut OneWire<P>, delay: &mut D) -> anyhow::Result<Vec<Address>> {
    // let mut num_found = 0;
    // let mut result: anyhow::Result<Address> = Err(anyhow::anyhow!("No DS18b20 device found"));
    let mut devices: Vec<Address> = vec![];

    for device_address in one.devices(false, delay) {
      // The search could fail at any time, so check each result. The iterator automatically
      // ends after an error.
      let device_address = device_address.into_anyhow()?;

      if device_address.family_code() == ds18b20::FAMILY_CODE {
        // skip other devices
        log::info!("Found temperature device at address {:?} with family code: {:#x?}",
          device_address, device_address.family_code()
        );
        devices.push(device_address);
      } else {
        log::warn!("Unknown device at address {:?} with family code: {:#x?}",
          device_address, device_address.family_code()
        )
      }

    }
    

    return Ok(devices)
  }

  fn parse_scratchpad(scratchpad: [u8; 9]) -> anyhow::Result<SensorData> {

    let resolution = match scratchpad[4] {
      0b00011111 => Resolution::Bits9,
      0b00111111 => Resolution::Bits10,
      0b01011111 => Resolution::Bits11,
      0b01111111 => Resolution::Bits12,
      _ => return Err(OneWireError::<E>::CrcMismatch).into_anyhow(),
    };

    let raw_temp = u16::from_le_bytes([scratchpad[0], scratchpad[1]]);
    let temperature = match resolution {
        Resolution::Bits12 => (raw_temp as f32) / 16.0,
        Resolution::Bits11 => (raw_temp as f32) / 16.0,
        Resolution::Bits10 => (raw_temp as f32) / 16.0,
        Resolution::Bits9 => (raw_temp as f32) / 16.0,
    };
    Ok(SensorData {
        temperature,
        resolution,
        alarm_temp_high: i8::from_le_bytes([scratchpad[2]]),
        alarm_temp_low: i8::from_le_bytes([scratchpad[3]]),
    })
  }  

  pub fn get_temperature(&mut self) -> anyhow::Result<Vec<f32>> {

    // initiate a temperature measurement for all connected devices
    ds18b20::start_simultaneous_temp_measurement(&mut self.bus, &mut self.delay).into_anyhow()?;

    // wait until the measurement is done. This depends on the resolution you specified
    // If you don't know the resolution, you can obtain it from reading the sensor data,
    // or just wait the longest time, which is the 12-bit resolution (750ms)
    self.resolution.delay_for_measurement_time(&mut self.delay);
    let mut readings = vec![];

    // let sensor_data = self.sensor
    //   .read_data(&mut self.bus, &mut self.delay).into_anyhow()?;
    for sensor in &self.sensors {
      let scratchpad_res = read_scratchpad(
        sensor.address(), &mut self.bus, &mut self.delay
      ).into_anyhow();
  
      match scratchpad_res.and_then(|x| Self::parse_scratchpad(x)) {
        Ok(sensor_data) => {
          readings.push(sensor_data.temperature);
          log::info!(
            "Device at {:?} is {}°C (res {:?})", 
            sensor.address(), sensor_data.temperature, sensor_data.resolution
          );  
        },
        Err(err) => log::error!("Failed to read sensor data: {err}"),
      }
      

    }


    Ok(readings)
  }
}


pub struct DS18B20ArrayReader
// <P, E, D>
// where P: OutputPin<Error = E> + InputPin<Error = E>, 
//       E: std::fmt::Debug + std::fmt::Display + std::marker::Sync,
//       D: DelayUs<u16> + DelayMs<u16> 
{
        // inner: DS18B20Array<P, E, D>,
        cmd_channel: tokio::sync::mpsc::Sender<()>,
        data_channel: tokio::sync::mpsc::Receiver<anyhow::Result<Vec<f32>>>,
      }

impl DS18B20ArrayReader {
  pub fn new<P, E>(pin: P) -> anyhow::Result<Self>
    where P: OutputPin<Error = E> + InputPin<Error = E> + Send + 'static, 
          E: std::fmt::Debug + std::fmt::Display + std::marker::Sync + 'static {
    let (cmd_send, mut cmd_recv) = tokio::sync::mpsc::channel(1);
    let (data_send, data_recv) = tokio::sync::mpsc::channel(1);
    std::thread::spawn(move || {
      match Self::run(pin, cmd_recv, data_send) {
        Ok(_) => log::warn!("DS18B20ArrayReader exited"),
        Err(err) => log::error!("DS18B20ArrayReader failed: {err}"),
      }
    });
    Ok(DS18B20ArrayReader {
      cmd_channel: cmd_send, 
      data_channel: data_recv
     })
  }

  pub fn run<P, E>(pin: P, mut cmd_recv: Receiver<()>, data_send: Sender<anyhow::Result<Vec<f32>>>) -> anyhow::Result<()> 
    where P: OutputPin<Error = E> + InputPin<Error = E> + Send + 'static, 
          E: std::fmt::Debug + std::fmt::Display + std::marker::Sync + 'static {

    let delay = crate::util::StdDelay;
    let mut inner = DS18B20Array::new(pin, delay)?;
    loop {
      match cmd_recv.blocking_recv() {
        Some(_) => {},
        None => anyhow::bail!("Command channel closed!"),
      }

      let tmp_res = inner.get_temperature();

      match data_send.blocking_send(tmp_res) {
          Ok(()) => {},
          Err(_) => {
            log::error!("Failed to send data");
          },
      };      
    }
  }

  pub async fn get_temperature(&mut self) -> anyhow::Result<Vec<f32>> {
    self.cmd_channel.send(()).await?;
    match self.data_channel.recv().await {
        Some(res) => res,
        None => anyhow::bail!("Data channel closed!"),
    }
  }
}