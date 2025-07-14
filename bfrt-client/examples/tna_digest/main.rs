#![allow(dead_code)]

use bfrt_client::client::Client;
use log::{error, info};
use tokio::{select, signal};

#[derive(Debug, serde::Deserialize)]
struct DigestA {
    dst_addr: u64,
    port: u16,
    src_addr: u64,
}

#[derive(Debug, serde::Deserialize)]
struct DigestB {
    dst_addr: u64,
    f1: u32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    let mut client = Client::builder().p4_name("tna_digest").build()?;
    client.connect("http://127.0.0.1:50052").await?;

    client.run().await?;

    // Prepare ports

    let port_table = client
        .table()
        .get_by_name("$PORT")
        .expect("Table $PORT not found");

    let entries = [4u32, 8u32]
        .iter()
        .map(|port| {
            port_table.make_entry(
                vec![port_table
                    .make_key_exact("$DEV_PORT", port.to_be_bytes())
                    .expect("make_key failed")],
                Some(
                    port_table
                        .make_data([
                            ("$SPEED", "BF_SPEED_40G".into()),
                            ("$FEC", "BF_FEC_TYP_NONE".into()),
                            ("$PORT_ENABLE", true.into()),
                        ])
                        .expect("make_data failed"),
                ),
                None,
            )
        })
        .collect();

    client.table_mut().insert_entries(entries, None).await?;

    let bfrt_info = client.bfrt_info.clone().expect("bfrt_info is None");

    let digest_a = bfrt_info
        .get_learn::<DigestA>("SwitchIngressDeparser.digest_a")
        .expect("digest_a is None");
    let digest_b = bfrt_info
        .get_learn::<DigestB>("SwitchIngressDeparser.digest_b")
        .expect("digest_b is None");

    let mut digest_rx = client.digest_rx();

    loop {
        select! {
            _ = signal::ctrl_c() => {
                info!("Ctrl-C received, shutting down...");
                break;
            }

            Ok(digest) = digest_rx.recv() => {
                match digest {
                    digest if digest.digest_id == digest_a.id => {
                        let data = digest_a.parse_data(&digest.data)?;
                        info!("Received digest_a: {:x?}", data);
                    },
                    digest if digest.digest_id == digest_b.id => {
                        let data = digest_b.parse_data(&digest.data)?;
                        info!("Received digest_b: {:x?}", data);
                    },
                    _ => {
                        error!("Unknown digest_id: {}", digest.digest_id);
                    }
                }
            }

            else => {
                break;
            }
        }
    }

    Ok(())
}
