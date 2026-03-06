use obd2_reader::{ObdConnection, ObdError};

fn read_live_data(port: &str) -> Result<(), ObdError> {
    let mut obd = ObdConnection::connect(port)?;
    obd.initialize()?;

    println!("RPM:   {:.0}", obd.query_engine_rpm()?);
    println!("Speed: {} km/h", obd.query_vehicle_speed()?);
    println!("Temp:  {} °C", obd.query_coolant_temp()?);

    let raw_response = obd.send_raw("0111\r")?;
    println!("Raw: {}", raw_response);

    Ok(())
}

fn main() -> Result<(), ObdError> {
    read_live_data("/dev/rfcomm1")?;
    Ok(())
}
