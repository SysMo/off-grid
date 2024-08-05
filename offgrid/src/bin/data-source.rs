
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


pub fn energy(v: &Vec<u16>, i: usize) -> f32 {
    // i is the starting position of the 2-word number
    0.01 * (v[i] * 16 + v[i+1]) as f32
}

pub fn meas(reading: u16, multiplier: f32) -> f32 {
    multiplier * reading as f32
}

#[derive(Debug)]
struct ElectricalData {
    // v1-v3 phase voltages
    v1: f32,
    v2: f32,
    v3: f32,

    // i1-i3 phase currents
    i1: f32,
    i2: f32,
    i3: f32,

    // p1-p3 active phase powers
    p1: f32,
    p2: f32,
    p3: f32,

    //q1-q3 reactive phase powers
    q1: f32,
    q2: f32,
    q3: f32,

    //pf1 - pf3 power factors
    pf1: f32,
    pf2: f32,
    pf3: f32,

    f: f32 // voltage frequency
}

struct EnergyData {
    pt: f32, //total active energy
    pp: f32, //positive active energy
    pr: f32, //reverse active energy
    qt: f32, //total reactive energy
    qp: f32, //positive reactive energy
    qr: f32 //reverse reactive energy
}

impl std::fmt::Display for ElectricalData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Voltages: Va = {} V, Vb = {} V, Vc = {} V\n", self.v1, self.v2, self.v3))?;  // ? -> if what you are trying to do is Ok, do it; if not, return the Error and exit the function
        f.write_str(&format!("Currents: Ia = {} A, Ib = {} A, Ic = {} A\n", self.i1, self.i2, self.i3))?;
        f.write_str(&format!("Active powers: Pa = {} W, Pb = {} W, Pc = {} W\n", self.p1, self.p2, self.p3))?;
        f.write_str(&format!("Reactive powers: Qa = {} Var, Qb = {} Var, Qc = {} Var\n", self.q1, self.q2, self.q3))?;
        f.write_str(&format!("Voltage frequency: f = {} Hz", self.f))?;
        Ok(())
    }
}

impl std::fmt::Display for EnergyData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Active energy: Total = {} kWh, Positive = {} kWh, Reverse = {} kWh\n", self.pt, self.pp, self.pr))?;
        f.write_str(&format!("Reactive energy: Total = {} kVarh, Positive = {} kVarh, Reverse = {} kVarh\n", self.qt, self.qp, self.qr))?;
        Ok(())
    }
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
        // let rsp = ctx.read_holding_registers(0x00, 0x80).await??;
        // println!("Sensor values are ({}): {rsp:?}", rsp.len());

        let val = ctx.read_holding_registers(0x00, 0x1b).await??;
        println!("{}", ElectricalData {v1: meas(val[0], 0.1), v2: meas(val[1], 0.1), v3: meas(val[2], 0.1),
                                        i1: meas(val[3], 0.1), i2: meas(val[5], 0.1), i3: meas(val[6], 0.1),
                                        p1: val[8] as f32, p2: val[9] as f32, p3: val[10] as f32,
                                        q1: val[12] as f32, q2: val[13] as f32, q3: val[14] as f32,
                                        pf1: meas(val[20], 0.001), pf2: meas(val[21], 0.001), pf3: meas(val[22], 0.001),
                                        f: meas(val[26], 0.01)});
        
        let nrg: Vec<u16> = ctx.read_holding_registers(0x1d, 0x51).await??;
        println!("{}", EnergyData {pt: energy(&nrg, 0), pp: energy(&nrg, 10), pr: energy(&nrg, 20),
                                    qt: energy(&nrg, 30), qp: energy(&nrg, 40), qr: energy(&nrg, 50)});
        
        // println!("Current electric energy:");
        // println!("Active: Total = {} KWh; Positive = {} KWh; Reverse = {} KWh", energy(&rsp, 29), energy(&rsp, 39), energy(&rsp, 49));
        // println!("Reactive: Total = {} KVarh; Positive = {} KVarh; Reverse = {} KVarh", energy(&rsp, 59), energy(&rsp, 69), energy(&rsp, 79));
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