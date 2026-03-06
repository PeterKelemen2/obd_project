use obd2_reader::{ObdConnection, ObdError};

fn read_live_data(port: &str) -> Result<(), ObdError> {
    let mut obd = ObdConnection::connect(port)?;
    obd.initialize()?;

    println!("RPM:   {:.0}", obd.query_engine_rpm()?);
    println!("Speed: {} km/h", obd.query_vehicle_speed()?);
    println!("Temp:  {} °C", obd.query_coolant_temp()?);
    Ok(())
}

fn main() {
    read_live_data("/dev/rfcomm1").unwrap();
}
