use std::{env, process};
use std::str::FromStr;

use sunspec_rs::sunspec_connection::SunSpecConnection;
use sunspec_rs::sunspec_models::ValueType;
use tracing::metadata::LevelFilter;
use tracing::{error, info};
use tracing_subscriber;

#[tokio::main]
pub async fn main() {
    let log_level: LevelFilter = match env::var("BUSBOI_LOG_LEVEL") {
        Ok(lvl) => LevelFilter::from_str(&lvl).unwrap(),
        Err(_e) => LevelFilter::INFO,
    };
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_level(true)
        // .with_file(true)
        // .with_line_number(true)
        .init();

    // Get and setup arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        error!("3 arguments are required! hostname, port, sub_id");
        process::exit(1);
    }
    let hostname = &args[1];
    let port: u16 = args[2].parse().expect("2nd arg must be a port number");
    let sub_id: u8 = args[3].parse().expect("3rd arg must be a sub/slave id");
    let connection_str = format!("{}:{}", hostname, port);
    info!("Connecting to {connection_str} (SubId={sub_id})");

    let mut sun_spec_client = SunSpecConnection::new(connection_str.parse().unwrap(), Some(sub_id), false)
        .await.ok().unwrap();
    info!("Connection established!");

    /*
    AUX_OFF_Delay_Time: 0.5 (TrueValue)
    Delay Address: 41602 - scale-factor=-1
    AUX_ON_Battery_Voltage-RAW: 2 (raw)
     */

    let scale_factor_addr = 41563;
    let scale_factor = sun_spec_client.get_i16(scale_factor_addr).await.unwrap();
    println!("Scale factor: {scale_factor}");

    let addr = 41601;
    let current_value = sun_spec_client.get_u16(addr).await.unwrap();
    println!("Current AuxDelay Raw Value: {}", current_value);
    println!("Current Value Scaled: {} minutes", apply_scale_factor(current_value, scale_factor));

    let raw_new_value: u16 = 6;
    println!("New AuxDelay Value: {raw_new_value}");
    println!("New Value Scaled: {} minutes", apply_scale_factor(raw_new_value, scale_factor));
    let result = sun_spec_client.set_u16(addr, raw_new_value).await;

    if result.is_ok() {
        println!("Successfully set new value");
    } else {
        println!("Failed to set new value");
        println!("Error: {}", result.err().unwrap().to_string());
    }
}

pub fn apply_scale_factor<T, S>(value: T, sf: S) -> f64
where
    T: Copy + Into<f64>,
    S: Into<i32>,
{
    (value.into() as f64) * 10.0_f64.powi(sf.into())
}
