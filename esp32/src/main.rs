use bincode::config::{Configuration, BigEndian, LittleEndian, Fixint};
use esp_idf_hal::{gpio::{self, PinDriver}, task::watchdog::{self, TWDTDriver}, uart::AsyncUartDriver, units::Hertz};

use esp_idf_svc::{
    eventloop::{EspEventLoop, EspSystemEventLoop, System}, 
    hal::{prelude::Peripherals, task::block_on}, 
    nvs::EspDefaultNvsPartition, 
    timer::EspTaskTimerService
};
use esp_idf_sys::{self as _, EspError};
esp_idf_sys::esp_app_desc!();

pub mod config;
pub use config::APP_CONFIG;

pub mod wifi;
use modbus::dts777::MbDTS777Electricity;
use sensors::ds18b20::DS18B20ArrayReader;
pub use wifi::WiFiService;

pub mod mqtt;
pub use mqtt::MqttService;

pub mod sensors;
// pub use sensors::{SensorDataSnapshot, SensorReader, SensorReaderConfig, ModbusConfig};

pub mod modbus;

pub mod util;

// pub mod modbus;
// pub use modbus::{ModbusClient, ModbusClientConfig};

static BINCODE_BE: Configuration<BigEndian, Fixint> = bincode::config::standard().with_big_endian().with_fixed_int_encoding();
static BINCODE_LE: Configuration<LittleEndian, Fixint> = bincode::config::standard().with_little_endian().with_fixed_int_encoding();


fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // eventfd is needed by our mio poll implementation.  Note you should set max_fds
    // higher if you have other code that may need eventfd.
    log::info!("Setting up eventfd...");
    let config = esp_idf_sys::esp_vfs_eventfd_config_t {
      max_fds: 1,
      ..Default::default()
    };
    
    esp_idf_sys::esp! { unsafe { esp_idf_sys::esp_vfs_eventfd_register(&config) } }?;

    tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .build()?
      .block_on(async move {
        let local = tokio::task::LocalSet::new();
        match local.run_until(run()).await {
          Ok(_) => log::info!("Finshed"),
          Err(err) => log::error!("{err}")
        };
    
      });
      
    Ok(())
}

async fn run() -> anyhow::Result<()> {
  let peripherals = Peripherals::take()?;
  let sys_loop = EspSystemEventLoop::take()?;
  let nvs = EspDefaultNvsPartition::take()?;
  let timer_service = EspTaskTimerService::new()?;

  // Configure watchdog
  // let mut watchdog_config = watchdog::config::Config::default();
  // watchdog_config.panic_on_trigger = false;
  // log::info!("watchdog_config: {:?}", watchdog_config.duration);
  // let mut twdt_driver = TWDTDriver::new(peripherals.twdt, &watchdog_config).unwrap();
  // let mut sub = twdt_driver.watch_current_task().unwrap();
  

  let wifi_svc = WiFiService::start_async(
      APP_CONFIG.wifi, 
      peripherals.modem,
      &sys_loop, 
      &timer_service, 
      &nvs
  ).await?;
  
  log::info!("WiFi config  : {:?}", wifi_svc.wifi.get_configuration().unwrap());

  let mut mqtt = MqttService::start(APP_CONFIG.mqtt)?;
  
  // tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

  // let mut temperature_reader =  DS18B20ArrayReader::new::<_, EspError>(
  //     PinDriver::input_output(peripherals.pins.gpio3)?
  // )?;

  let mut power_meter_reader = modbus::ModbusRTUClient::new(
    peripherals.uart1, peripherals.pins.gpio1, peripherals.pins.gpio0
  )?;


  loop {
      tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
      log::info!("Woke up");

      match power_meter_reader.read_holding_registers(2, 0, 27).await {
        Ok(data) => {
          log::info!("Modbus data: {:?}", data);
          
          match bincode::serde::decode_from_slice::<MbDTS777Electricity, _>(&data, BINCODE_BE) {
            Ok(data) => {
              log::info!("{:?}", data);
            },
            Err(err) => {
              log::error!("Failed to deserialize data: {err}");
            },
          }
          mqtt.publish("offgrid/in/data", &data).await?
        },
        Err(err) => {
          log::error!("{}", err);
          mqtt.publish("offgrid/in/error", &err.to_string()).await?
        },
      }
      // match temperature_reader.get_temperature().await {
      //   Ok(data) => {
      //     mqtt.publish("offgrid/in/data", &data).await?
      //   },
      //   Err(err) => {
      //     mqtt.publish("offgrid/in/error", &err.to_string()).await?
      //   },        
      // }
      
  }

}
