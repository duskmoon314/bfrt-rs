use std::time::Duration;

use bfrt_client::{client::Client, utils::de::data_field::from_data_field};
use tokio::{select, signal, time::sleep};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    let mut client = Client::builder().p4_name("tna_counter").build()?;
    client.connect("http://127.0.0.1:50052").await?;

    client.run().await?;

    let port_table = client
        .table()
        .get_by_name("$PORT")
        .expect("Table $PORT not found");

    let forward_dst_table = client
        .table()
        .get_by_name("SwitchIngress.forward_dst")
        .expect("Table forward_dst not found");

    let prepare_entries = vec![
        port_table.make_entry(
            vec![port_table.make_key("$DEV_PORT", 4u32.to_be_bytes().to_vec(), None::<i32>)?],
            Some(port_table.make_data(
                None::<String>,
                &[
                    ("$SPEED", "BF_SPEED_40G".into()),
                    ("$FEC", "BF_FEC_TYP_NONE".into()),
                    ("$PORT_ENABLE", true.into()),
                ],
            )?),
            None,
        ),
        port_table.make_entry(
            vec![port_table.make_key("$DEV_PORT", 8u32.to_be_bytes().to_vec(), None::<i32>)?],
            Some(port_table.make_data(
                None::<String>,
                &[
                    ("$SPEED", "BF_SPEED_40G".into()),
                    ("$FEC", "BF_FEC_TYP_NONE".into()),
                    ("$PORT_ENABLE", true.into()),
                ],
            )?),
            None,
        ),
        forward_dst_table.make_entry(
            vec![forward_dst_table.make_key(
                "hdr.ethernet.dst_addr",
                vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
                None::<i32>,
            )?],
            Some(forward_dst_table.make_data(
                Some("SwitchIngress.hit_dst"),
                &[("port", 4u16.to_be_bytes().to_vec().into())],
            )?),
            None,
        ),
    ];

    client
        .table_mut()
        .insert_entries(prepare_entries, None)
        .await?;

    let counter_table = client
        .table()
        .get_by_name("SwitchIngress.indirect_counter")
        .expect("Table indirect_counter not found");

    let read_entry = counter_table.make_entry(
        vec![counter_table.make_key("$COUNTER_INDEX", vec![0, 0, 0, 4], None::<i32>)?],
        None,
        Some(bfrt::bfrt::TableFlags {
            from_hw: true,
            ..Default::default()
        }),
    );

    loop {
        select! {
            _ = signal::ctrl_c() => {
                break;
            }

            _ = sleep(Duration::from_secs(1)) => {
                // Tofino shares counter among all pipes, se we cannot set pipe_id when reading
                let entries = client
                    .table_mut()
                    .get_entries(vec![read_entry.clone()], None)
                    .await?;

                for entry in entries {
                    println!("E {:?}", entry.value);

                    let data = entry.data.as_ref().expect("Data not found");
                    for field in &data.fields {
                        let value: u64 = from_data_field(field)?;
                        println!("  {}: 0x{:x}", field.field_id, value);
                    }
                }
            }
        }
    }

    Ok(())
}
