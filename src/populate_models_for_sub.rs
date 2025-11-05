use std::process;
use std::env;
use std::str::FromStr;

use sunspec_rs::sunspec_connection::{SunSpecConnection};
use sunspec_rs::sunspec_data::SunSpecData;

use tracing::metadata::LevelFilter;
use tracing::{error, info};
use tracing_subscriber;

#[tokio::main]
pub async fn main() {
    let log_level = match env::var("BUSBOI_LOG_LEVEL") {
        Ok(lvl) => LevelFilter::from_str(&lvl).unwrap(),
        Err(_e) => LevelFilter::INFO
    };
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    // Get and setup arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        error!("3 arguments are required! hostname, port, sub_id");
        process::exit(1);
    }

    let hostname = args[1].clone();
    let port: u16 = args[2].parse().expect("2nd arg must be a port number");
    let sub_id: u8 = args[3].parse().expect("3rd arg must be a numeric sub/slave id");
    let conn_str = format!("{}:{}", hostname, port);
    info!("Connecting to {} on SubId={}", conn_str, sub_id);

    let sun_spec_client = SunSpecConnection::new(conn_str.parse().unwrap(), Some(sub_id), false)
        .await.ok().unwrap();
    println!("Connection Established!");

    let sun_spec_data = SunSpecData::default();
    match sun_spec_client.populate_models(&sun_spec_data).await {
        Ok(m) => {
            m.iter().for_each(|key| {println!("{:?}", key)});
        },
        Err(e) => {
            panic!("Can't populate models: {e}");
        }
    };
}
