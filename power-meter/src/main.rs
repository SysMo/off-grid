use tokio_serial::{SerialPortBuilderExt, DataBits, StopBits, Parity};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process, time
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    modbus().await
    // low_level().await
}

// adajsdasdjla
pub async fn modbus() -> Result<(), Box<dyn std::error::Error>> {
    use tokio_serial::SerialStream;

    use tokio_modbus::prelude::*;

    let tty_path = "/dev/ttyUSB0";
    let slave = Slave(0x0);

    let builder = tokio_serial::new(tty_path, 9600)
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One)
        .parity(Parity::Even);
    let port = SerialStream::open(&builder).unwrap();

    let mut ctx = rtu::attach_slave(port, slave);
    loop {
        println!("Reading a sensor value");
        let rsp = ctx.read_holding_registers(0x00, 0x50).await??;
        println!("Sensor values are ({}): {rsp:?}", rsp.len());
        time::sleep(time::Duration::from_secs(2)).await;
    }

    println!("Disconnecting");
    ctx.disconnect().await??;

    Ok(())
}

pub async fn low_level() -> Result<(), Box<dyn std::error::Error>> {    
    let tty_path = "/dev/ttyUSB0";

    let builder = tokio_serial::new(tty_path, 9600)
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One)
        .parity(Parity::Even);

    let mut port = builder.open_native_async()?;

    let req: [u8; 8] = [0x00, 0x04, 0x00, 0x1D, 0x00, 0x02, 0xe0, 0x1c];

    port.write_all(&req).await?;

    let mut reply: [u8; 14] = [0; 14];

    port.read(&mut reply).await?;
    
    println!("{:?}", req);
    println!("{:?}", reply);

    Ok(())
}