use obd2_reader::command::pids;
use obd2_reader::{ObdConnection, ObdError};

fn read_live_data(port: &str) -> Result<(), ObdError> {
    let mut obd = ObdConnection::connect(port)?;
    obd.initialize()?;

    let supported = obd.query_supported_pids()?;
    println!("Supported PIDs: {:?}", supported);

    if supported.contains(&0x0C) {
        let rpm = obd.query_engine_rpm()?;
        println!("RPM: {rpm}");
    }

    println!("RPM:   {:.0}", obd.query_engine_rpm()?);
    println!("Speed: {} km/h", obd.query_vehicle_speed()?);
    println!("Temp:  {} °C", obd.query_coolant_temp()?);

    let rpm: f32 = obd.query(pids::ENGINE_RPM)?;
    let speed: u8 = obd.query(pids::VEHICLE_SPEED)?;
    let temp: i16 = obd.query(pids::COOLANT_TEMP)?;

    println!("RPM:   {rpm:.0}");
    println!("Speed: {speed} km/h");
    println!("Temp:  {temp} °C");

    let raw_response = obd.send_raw("0111\r")?;
    println!("Raw: {}", raw_response);

    Ok(())
}

fn main() -> Result<(), ObdError> {
    read_live_data("/dev/rfcomm1")?;
    Ok(())
}
