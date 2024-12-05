use bfrt_client::client::Client;
use log::info;

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

    let entry = port_table.make_entry(
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
    );

    client.table_mut().insert_entries(vec![entry], None).await?;

    info!("We need to wait for the port to be enabled");

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    info!("Now we read the entry to see the status");

    // The following code may have issues with the data field or the flags
    // Seeing Error message:
    //     BF_BFRT ERROR - BF_RT_SERVER:populateReadResponseWithNonRegisterFields:1740 Not supported Operation not supported
    // But still get the result

    let entry = port_table.make_entry(
        vec![port_table.make_key("$DEV_PORT", 4u32.to_be_bytes().to_vec(), None::<i32>)?],
        None,
        None,
    );

    let entries = client.table_mut().get_entries(vec![entry], None).await?;

    println!("Entries: {:?}", entries);

    Ok(())
}
