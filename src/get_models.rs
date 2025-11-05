use std::{env, process};
use std::str::FromStr;
use sunspec_rs::sunspec_connection::SunSpecConnection;
use sunspec_rs::sunspec_data::SunSpecData;
use tracing::{error, info};
use tracing::level_filters::LevelFilter;
use tracing_subscriber;

pub async fn setup(addr: &str, slave_id: u8) -> (SunSpecConnection, SunSpecData) {
    let mut ss = match SunSpecConnection::new(addr.parse().unwrap(), Some(slave_id), false).await {
        Ok(mb) => mb,
        Err(e) => {
            panic!("Can't create modbus connection: {e}");
        }
    };

    let ssd = SunSpecData::default();
    match ss.clone().populate_models(&ssd).await {
        Ok(m) => ss.models = m,
        Err(e) => {
            panic!("Can't populate models: {e}")
        }
    };

    for model in ss.models.values() {
        let resolved = model.to_owned().get_resolved_model().await;
        println!("{}: {:?}", model.id, resolved);
    }

    return (ss, ssd);
}
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

    let (ss, _) = setup(&connection_str, sub_id).await;
    // for (id, _) in ss.models.iter() {
    //     println!("{}", id);
    // }

    for model in ss.models.values() {
        let resolved = model.clone().get_resolved_model().await;
        println!("{}: {:?}", model.id, resolved);
    }
}
