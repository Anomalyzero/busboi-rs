use std::{env, process};
use std::str::FromStr;

use sunspec_rs::sunspec_connection::SunSpecConnection;

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

    info!("Retrieving all GSconfig Values ...");
    let start_addr = 41557;
    let mut offset = 0;

    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "DID").await;
    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Length").await;
    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Port_number").await;

    let dc_voltage_scale_factor = get_and_print_i16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "DC_Voltage_SF").await.unwrap();
    let ac_current_scale_factor = get_and_print_i16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "AC_Current_SF").await.unwrap();
    let ac_voltage_scale_factor = get_and_print_i16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "AC_Voltage_SF").await.unwrap();
    let time_scale_factor = get_and_print_i16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Time_SF").await.unwrap();

    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Major_Firmware_Number").await;
    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Mid_Firmware_Number").await;
    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Minor_Firmware_Number").await;

    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "Absorb_Volts").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), time_scale_factor, "Absorb_Time_Hours").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "Float_Volts").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), time_scale_factor, "Float_Time_Hours").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "ReFloat_Volts").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "EQ_Volts").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), time_scale_factor, "EQ_Time_Hours").await;

    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Search_Sensitivity").await;
    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Search_Pulse_Length").await;
    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Search_Pulse_Spacing").await;
    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "AC_Input_Select_Priority").await;

    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), ac_current_scale_factor, "Grid_AC_Input_Current_Limit").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), ac_current_scale_factor, "Gen_AC_Input_Current_Limit").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), ac_current_scale_factor, "Charger_AC_Input_Current_Limit").await;

    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Charger_Operating_Mode").await;
    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "AC_Coupled").await;
    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Grid_Input_Mode").await;

    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), ac_voltage_scale_factor, "Grid_Lower_Input_Voltage_Limit").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), ac_voltage_scale_factor, "Grid_Upper_Input_Voltage_Limit").await;
    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Grid_Transfer_Delay").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), time_scale_factor, "Grid_Connect_Delay").await;

    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Gen_Input_Mode").await;

    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), ac_voltage_scale_factor, "Gen_Lower_Input_Voltage_Limit").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), ac_voltage_scale_factor, "Gen_Upper_Input_Voltage_Limit").await;

    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Gen_Transfer_Delay").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), time_scale_factor, "Gen_Connect_Delay").await;

    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), ac_voltage_scale_factor, "AC_Output_Voltage").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "Low_Battery_Cut_Out_Voltage").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "Low_Battery_Cut_In_Voltage").await;

    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "AUX_Mode").await;
    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "AUX_Control").await;
    //----pdf page----
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "AUX_ON_Battery_Voltage").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), time_scale_factor, "AUX_ON_Delay_Time").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "AUX_OFF_Battery_Voltage").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), time_scale_factor, "AUX_OFF_Delay_Time").await;

    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "AUX_Relay_Mode").await;
    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "AUX_Relay_Control").await;

    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "AUX_Relay_ON_Battery_Voltage").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), time_scale_factor, "AUX_Relay_ON_Delay_Time").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "AUX_Relay_OFF_Battery_Voltage").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), time_scale_factor, "AUX_Relay_OFF_Delay_Time").await;

    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Stacking_Mode").await;
    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Master_Power_Save_Level").await;
    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Slave_Power_Save_Level").await;

    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "Sell_Volts").await;

    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Grid_Tie_Window").await;
    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Grid_Tie_Enable").await;
    get_and_print_i16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Grid_AC_Input_Voltage_Calibrate_Factor").await;
    get_and_print_i16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Gen_AC_Input_Voltage_Calibrate_Factor").await;
    get_and_print_i16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "AC_Output_Voltage_Calibrate_Factor").await;

    get_and_print_scaled_i16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "Battery_Voltage_Calibrate_Factor").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "ReBulk_Volts").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "Mini_Grid_LBX_Volts").await;

    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Mini_Grid_LBX_Delay").await;

    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "Grid_Zero_DoD_Volts").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), ac_voltage_scale_factor, "Grid_Zero_DoD_Max_Offset_AC_Amps").await;

    get_and_print_string_point(&mut sun_spec_client, start_addr + get_and_inc_amt(&mut offset, 9), 18, "Serial_Number").await;
    get_and_print_string_point(&mut sun_spec_client, start_addr + get_and_inc_amt(&mut offset, 9), 18, "Model_Number").await;
    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Module_Control").await;
    get_and_print_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), "Model_Select").await;

    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "Low_Battery_Cut_Out_Delay").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "High_Battery_Cut_Out_Voltage").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "High_Battery_Cut_In_Voltage").await;
    get_and_print_scaled_u16_point(&mut sun_spec_client, start_addr + get_and_inc_one(&mut offset), dc_voltage_scale_factor, "High_Battery_Cut_Out_Delay").await;
    // TODO: Determine how to use populate_models and the resulting model objectrs to get all data
    // TODO: like this, rather than relying on psuedo-hardcode and increments to read the data.
}

pub fn print_scaled_value(value: u16, scale_factor: i16, name: &str) {
    info!("    -> {}: {} (True Value)", name, apply_scale_factor(value, scale_factor));
}

//TODO: Write a trait for the various get_print point functions

pub async fn get_and_print_scaled_u16_point(conn: &mut SunSpecConnection, addr: u16, scale_factor: i16, name: &str) -> Option<u16> {
    match conn.get_u16(addr).await {
        Ok(x) => {
            info!("{}: {} (TrueValue)", name, apply_scale_factor(x, scale_factor));
            Some(x)
        },
        Err(e) => {
            error!("{}: {}", name, e.to_string());
            None
        }
    }
}


pub async fn get_and_print_u16_point(conn: &mut SunSpecConnection, addr: u16, name: &str) -> Option<u16> {
    match conn.get_u16(addr).await {
        Ok(x) => {
            info!("{}: {} (raw)", name, x);
            Some(x)
        },
        Err(e) => {
            error!("{}: {}", name, e.to_string());
            None
        }
    }
}

pub async fn get_and_print_scaled_i16_point(conn: &mut SunSpecConnection, addr: u16, scale_factor:i16, name: &str) -> Option<i16> {
    match conn.get_i16(addr).await {
        Ok(x) => {
            info!("{}: {} (TrueValue)", name, apply_scale_factor(x, scale_factor));
            Some(x)
        },
        Err(e) => {
            error!("{}: {}", name, e.to_string());
            None
        }
    }
}

pub async fn get_and_print_i16_point(conn: &mut SunSpecConnection, addr: u16, name: &str) -> Option<i16> {
    match conn.get_i16(addr).await {
        Ok(x) => {
            info!("{}: {} (raw)", name, x);
            Some(x)
        },
        Err(e) => {
            error!("{}: {}", name, e.to_string());
            None
        }
    }
}

pub async fn get_and_print_string_point(conn: &mut SunSpecConnection, addr: u16, qty: u16, name: &str) -> Option<String>{
    match conn.get_string(addr, qty).await {
        Ok(x) => {
            info!("{}: {}", name, x);
            Some(x)
        },
        Err(e) => {
            error!("{}: {}", name, e.to_string());
            None
        }
    }
}

// TODO: I get the impression that this kind of 'i++' behavior is specifically discouraged and is to
// TODO: be avoided in rust. Do some reading and determine how to refactor the code using this
// TODO: 'plus-plus' function to follow rust standards
fn get_and_inc_one(n: &mut u16) -> u16 {
    get_and_inc_amt(n, 1)
}

fn get_and_inc_amt(n: &mut u16, amt: u16) -> u16 {
    *n += amt;
    *n - amt
}

// TODO: This function was lifted straight out of a newer version of sunspec_rs and copied here.
// TODO: I need to determine why the newer versions of the library do not work with our device.
// TODO: Report an issue on the lib project, or begin writing a PR to it if I can determine what is
// TODO: going wrong with the library.
pub fn apply_scale_factor<T, S>(value: T, sf: S) -> f64
where
    T: Copy + Into<f64>,
    S: Into<i32>,
{
    (value.into() as f64) * 10.0_f64.powi(sf.into())
}
