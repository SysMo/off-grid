use std::ffi::CStr;
// use embassy_futures::select::{select, Either};
use esp_idf_svc::{
  mqtt::client::{EspAsyncMqttClient, EspAsyncMqttConnection, EspMqttClient, EspMqttConnection, MqttClientConfiguration, QoS}, 
  sys::EspError, timer::{EspAsyncTimer, EspTaskTimerService}, 
  tls::X509
};
use serde::Serialize;
use tokio::task::JoinHandle;

// use tokio::task::JoinHandle;
// use core::pin::pin;
// use std::time::Duration;

pub struct MqttConfig {
  pub url: &'static str,
  pub client_id: &'static str,
  pub client_key: &'static[u8],
  pub client_cert: &'static[u8],
  pub ca_cert: &'static[u8]
}

pub struct MqttService {
  receiver_task: JoinHandle<anyhow::Result<()>>,
  client: EspAsyncMqttClient,
  // connection: EspAsyncMqttConnection,
}

impl MqttService {
  pub fn start(config: MqttConfig) -> Result<MqttService, EspError> {
    log::info!("Spinning up MQTT");
    let client_config = MqttClientConfiguration {
      client_id: Some(config.client_id),
      private_key: Some(X509::pem(CStr::from_bytes_with_nul(config.client_key).unwrap())),
      client_certificate: Some(X509::pem(CStr::from_bytes_with_nul(config.client_cert).unwrap())),
      server_certificate: Some(X509::pem(CStr::from_bytes_with_nul(config.ca_cert).unwrap())),
      ..Default::default()
    };



    let (client, connection) = EspAsyncMqttClient::new(
      config.url,
      &client_config,
    ).unwrap();

    let receiver_task: JoinHandle<anyhow::Result<()>> = tokio::task::spawn_local(async move {
      log::info!("About to start the MQTT event loop");
      Self::event_loop(connection).await;
      Ok(())
    });

    Ok(MqttService {
      client,
      receiver_task
    })
  }


  async fn event_loop(mut connection: EspAsyncMqttConnection) {
    // let connection =  unsafe { MQTT_CONNECTION.get_mut().unwrap() };

    log::info!("MQTT Listening for messages");

    while let Ok(event) = connection.next().await {
      log::info!("[Queue] Event: {}", event.payload());
    }

    log::info!("Connection closed");

    // Ok(())
  }

  pub async fn publish<T: Serialize>(&mut self, topic: &str, value: &T) -> anyhow::Result<()> {
    let msg = serde_json::to_string(value)?;
    let msg_data = msg.as_bytes();
    self.client.publish(topic, QoS::AtMostOnce, false, msg_data).await?;
    Ok(())
  }
}
