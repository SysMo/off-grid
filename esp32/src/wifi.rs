use esp_idf_svc::{
  eventloop::{EspEventLoop, EspSystemEventLoop, System}, hal::{modem::Modem, prelude::Peripherals}, nvs::EspDefaultNvsPartition, sys::EspError, timer::EspTaskTimerService, wifi::{AsyncWifi, AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi}
};



#[derive(Debug)]
pub struct WifiConfig {
  pub ssid: &'static str,
  pub passwd: &'static str,
}

pub struct WiFiService {
  pub wifi: AsyncWifi<EspWifi<'static>>
}

impl WiFiService {
  pub async fn start_async(
    config: WifiConfig,     
    modem: Modem, 
    sys_loop: &EspEventLoop<System>, 
    timer_service: &EspTaskTimerService,
    nvs: &EspDefaultNvsPartition
  ) -> anyhow::Result<Self> {
    let mut wifi = AsyncWifi::wrap(
      EspWifi::new(modem, sys_loop.clone(), Some(nvs.clone()))?,
      sys_loop.clone(),
      timer_service.clone(),
    )?;

    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
      ssid: config.ssid.try_into()
        .map_err(|_| anyhow::anyhow!("Failed converting SSID"))?,
      bssid: None,
      auth_method: AuthMethod::WPA2Personal,
      password: config.passwd.try_into()
        .map_err(|_| anyhow::anyhow!("Failed converting password"))?,
      channel: None,
      ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start().await?;
    log::info!("Wifi started");

    wifi.connect().await?;
    log::info!("Wifi connected");

    wifi.wait_netif_up().await?;
    log::info!("Wifi netif up");

    Ok(Self { wifi })
  }
}

