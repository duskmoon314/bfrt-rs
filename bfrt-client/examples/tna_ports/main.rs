use bfrt_client::client::Client;
use log::{debug, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    let mut client = Client::builder().p4_name("tna_ports").build()?;
    client.connect("http://127.0.0.1:50052").await?;

    client.run().await?;

    let port_table = client
        .table()
        .get_by_name("$PORT")
        .expect("Table not found");

    let attr_port_status_change = port_table.make_port_status_change_attr(true);

    client
        .table_mut()
        .insert_attributes(vec![attr_port_status_change], None)
        .await?;

    let entry = port_table.make_entry(
        vec![port_table.make_key_exact("$DEV_PORT", 4u32.to_be_bytes())?],
        Some(port_table.make_data([
            ("$SPEED", "BF_SPEED_40G".into()),
            ("$FEC", "BF_FEC_TYP_NONE".into()),
            ("$PORT_ENABLE", true.into()),
        ])?),
        None,
    );

    client.table_mut().insert_entries(vec![entry], None).await?;

    info!("We need to wait for the port to be enabled");

    let status = client.get_port_status_change(5).await?;

    info!("If this message is printed, the port should be enabled");

    debug!("Port status: {:?}", status);

    Ok(())
}
