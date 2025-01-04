// use tokio_modbus::client::Context;
// use tokio_modbus::prelude::*;
// use tokio_serial::{DataBits, StopBits, Parity, SerialStream};

use embedded_io_async::{Read, Write};
use esp_idf_hal::{
  gpio::{self, InputPin, OutputPin}, 
  peripheral::Peripheral, 
  uart::{self, AsyncUartDriver, Uart, UartDriver}, 
  units::Hertz
};

use rmodbus::{client::ModbusRequest, guess_response_frame_len, ModbusProto};

pub struct ModbusRTUClientConfig {
  // slave_id: u8,
  // data_bits: DataBits,
  // stop_bits: StopBits,
  // parity: Parity
}

pub struct ModbusRTUClient<'d> { 
  driver: AsyncUartDriver<'d, UartDriver<'d>>,
}

impl<'d> ModbusRTUClient<'d> {
  pub fn new(
    uart: impl Peripheral<P = impl Uart> + 'd,
    tx: impl Peripheral<P = impl OutputPin> + 'd,
    rx: impl Peripheral<P = impl InputPin> + 'd,

  ) -> anyhow::Result<Self> {
    let config = uart::config::Config::new().baudrate(Hertz(9600));
    let driver = AsyncUartDriver::new(
      uart,
      tx,
      rx,
      Option::<gpio::Gpio0>::None,
      Option::<gpio::Gpio1>::None,
      &config,
    )?;
    Ok(ModbusRTUClient {
      driver
    })
  }

  pub async fn read_holding_registers(&mut self, slave_id: u8, start_addr: u16, count: u16) -> anyhow::Result<Vec<u8>> {
    let mut mreq = ModbusRequest::new(slave_id, ModbusProto::Rtu);
    let mut request = Vec::new();
    mreq.generate_get_holdings(start_addr, count, &mut request)?;
    // log::info!("req {:?}", request);

    self.driver.write_all(&request).await?;

    const start_bytes: usize = 3;
    let mut buf = [0u8; start_bytes];
    self.driver.read_exact(&mut buf).await?;
    // log::info!("resp (0-{}): {:?}", start_bytes, buf);

    let mut response: Vec<u8> = Vec::new();
    response.extend_from_slice(&buf);

    let len = guess_response_frame_len(&buf, ModbusProto::Rtu)? as usize;
    let response2 = if len > start_bytes {
        let mut rest = vec![0u8; (len - start_bytes - 2)];
        self.driver.read_exact(&mut rest).await?;
        let mut crc = [0u8; 2];
        self.driver.read_exact(&mut crc).await?;        
        rest
        // log::info!("resp ({}-): {:?}", start_bytes, rest);
        // response.extend(rest);
    } else {
      vec![]
    };
    
    Ok(response2)
  }


}





// use std::task::Poll;

// use esp_idf_hal::{delay::NON_BLOCK, peripheral, uart::{AsyncUartDriver, UartDriver, UartRxDriver, UartTxDriver}};
// use rmodbus::{client::ModbusRequest, guess_response_frame_len, ModbusProto};
// use tokio::io::{AsyncRead, AsyncWrite};


// struct ModbusRtuTransport<'a> {
//   tx: UartTxDriver<'a>,
//   rx: UartRxDriver<'a>,
// }

// impl<'a> AsyncRead for ModbusRtuTransport<'a> {
//     fn poll_read(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//         buf: &mut tokio::io::ReadBuf<'_>,
//     ) -> Poll<std::io::Result<()>> {
//       let mut read_buf: [u8; 100] = [0; 100];
//       // buf.put_slice(buf);
//       // cx.waker().
//       match self.rx.read(&mut read_buf, NON_BLOCK) {
//         Ok(size) => {
//           buf.put_slice(&read_buf);
//         },
//         Err(err) => Poll::from(err),
//       }
//       Poll::Pending
//     }
// }

// impl<'a> AsyncWrite for ModbusRtuTransport<'a> {
//     fn poll_write(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//         buf: &[u8],
//     ) -> std::task::Poll<Result<usize, std::io::Error>> {
//         todo!()
//     }

//     fn poll_flush(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), std::io::Error>> {
//         todo!()
//     }

//     fn poll_shutdown(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), std::io::Error>> {
//         todo!()
//     }
// }


// impl ModbusClient {
//   // pub async fn connect_rtu(rtu_config: &ModbusTransportRTUConfig, codec_config: &ModbusCodecConfig) -> anyhow::Result<Self> {
//   //   // let builder = tokio_serial::new(&rtu_config.tty_path, 9600)
//   //   //   .data_bits(rtu_config.data_bits)
//   //   //   .stop_bits(rtu_config.stop_bits)
//   //   //   .parity(rtu_config.parity);
//   //   // let port = SerialStream::open(&builder)?;
//   //   let mut connection = tokio_modbus::client::rtu::attach(transport)
//   //   let context = rtu::attach_slave(port, Slave(rtu_config.slave_id));
//   //   Ok(ModbusDataClient {context, codec_config: codec_config.clone()})
//   // }



//   pub async fn connect_rtu() {
//     // let uart_driver = AsyncUartDriver::new(
//     //   peripherals
//     // )?;


//     let mut connection = tokio_modbus::client::rtu::attach_slave(transport);
    
//   } 
// }

