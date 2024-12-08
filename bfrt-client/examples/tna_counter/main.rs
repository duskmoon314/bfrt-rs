use bfrt_client::{client::Client, utils::de::data_field::from_data_field};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    let mut client = Client::builder().p4_name("tna_counter").build()?;
    client.connect("http://127.0.0.1:50052").await?;

    client.run().await?;

    let counter_table = client
        .table()
        .get_by_name("SwitchIngress.indirect_counter")
        .expect("Table not found");

    let init_entry = counter_table.make_entry(
        vec![counter_table.make_key("$COUNTER_INDEX", vec![0, 0, 0, 1], None::<i32>)?],
        Some(counter_table.make_data(
            None::<String>,
            &[
                ("$COUNTER_SPEC_BYTES", 0u64.to_be_bytes().to_vec().into()),
                ("$COUNTER_SPEC_PKTS", 0u64.to_be_bytes().to_vec().into()),
            ],
        )?),
        None,
    );

    let read_entry = counter_table.make_entry(
        vec![counter_table.make_key("$COUNTER_INDEX", vec![0, 0, 0, 1], None::<i32>)?],
        None,
        Some(bfrt::bfrt::TableFlags {
            from_hw: true,
            ..Default::default()
        }),
    );

    client
        .table_mut()
        .insert_entries(vec![init_entry], None)
        .await?;

    // Tofino shares counter among all pipes, se we cannot set pipe_id when reading
    let entries = client
        .table_mut()
        .get_entries(vec![read_entry], None)
        .await?;

    for entry in entries {
        println!("E {:?}", entry.value);

        let data = entry.data.as_ref().expect("Data not found");
        for field in &data.fields {
            let value: u64 = from_data_field(field)?;
            println!("  {}: 0x{:x}", field.field_id, value);
        }
    }

    Ok(())
}
