#[macro_use]
extern crate tracing;

use std::env;
use std::process;
use std::str::FromStr;

use sunspec_rs::sunspec_connection::SunSpecConnection;
use sunspec_rs::sunspec_data::SunSpecData;
use sunspec_rs::sunspec_models::ValueType;
use tracing::level_filters::LevelFilter;
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

    let mut devices: Vec<String> = vec![];

    let ssd = SunSpecData::default();
    for i in 1..99 {
        info!("Testing slave {i}");
        let mut ss = match SunSpecConnection::new(connection_str.clone(), Some(i), false).await {
            Ok(mb) => mb,
            Err(e) => {
                panic!("Can't create modbus connection: {e}");
            }
        };
        ss.models = match ss.clone().populate_models(&ssd).await {
            Ok(m) => m,
            Err(_) => {
                continue;
            }
        };

        let md = ss.models.get(&1).unwrap().clone();
        match ss
            .clone()
            .get_point(md.clone(), "SN", None)
            // .get_point(md.clone(), PointIdentifier::Point("SN".to_string()))
            .await
        {
            Ok(p) => {
                if let Some(st) = p.value {
                    if let ValueType::String(s) = st {
                        if devices.contains(&s) {
                            warn!("Slave id {i} is a clone signal.");
                            continue;
                        }
                        println!("Slave {i} {s} has {} models.", ss.models.len());
                        devices.push(s)
                    }
                }
            }
            Err(e) => {
                warn!("No serial number in common? (slave {i})  Should not be possible: {e}");
            }
        }
        println!("Done with sub");
    }
}
