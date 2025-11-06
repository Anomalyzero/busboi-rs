use std::{error::Error, net::SocketAddr, time::Duration};

use clap::Parser;
use itertools::Itertools;
use sunspec::client::AsyncClient;
use sunspec::client::Config;
// use sunspec::models::model
use sunspec::models::model1::Model1;
use sunspec::models::model103::Model103;
use sunspec::models::model102::Model102;
use sunspec::models::Models;
use tokio::time::sleep;
use tokio_modbus::client::tcp::connect;
use sunspec::models::model121::Model121;
use sunspec::models::model124::Model124;
use sunspec::models::model64111::Model64111;
use sunspec::models::model64112::{CcConfigAuxMode, Model64112};

#[derive(Parser)]
struct Args {
    addr: SocketAddr,
    device_id: u8,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let client = AsyncClient::new(connect(args.addr).await.unwrap(), Config::default());
    let device = client.device(args.device_id).await.unwrap();

    let m1: Model1 = device.read_model().await.unwrap();

    println!("Manufacturer: {}", m1.mn);
    println!("Model: {}", m1.md);
    println!("Version: {}", m1.vr.as_deref().unwrap_or("(unspecified)"));
    println!("Serial Number: {}", m1.sn);
}
