use bfrt_client::{client::Client, utils::de::data_field::from_data_field};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut client = Client::builder().p4_name("tna_register").build()?;
    client.connect("http://127.0.0.1:50052").await?;

    client.run().await?;

    let register_table = client
        .table()
        .get_by_name("SwitchIngress.test_reg")
        .expect("table not found");

    let key = register_table.make_key("$REGISTER_INDEX", vec![0, 0, 0, 1], None::<i32>)?;
    let insert_entry = register_table.make_entry(
        vec![key.clone()],
        Some(register_table.make_data(
            None::<String>,
            &[
                ("SwitchIngress.test_reg.first", vec![1, 2, 3, 4].into()),
                ("SwitchIngress.test_reg.second", vec![5, 6, 7, 8].into()),
            ],
        )?),
        None,
    );

    client
        .table_mut()
        .insert_entries(vec![insert_entry], None)
        .await?;

    let entry = register_table.make_entry(
        vec![key],
        None,
        Some(bfrt::bfrt::TableFlags {
            from_hw: true,
            ..Default::default()
        }),
    );
    // We only want to get the data from pipeline 0
    // By default, the target is set to all pipelines
    // Thus, we will receive data from all pipelines, which needs more care to
    // handle it correctly
    let target = bfrt::bfrt::TargetDevice {
        pipe_id: 0,
        ..client.target()
    };

    let entry = client
        .table_mut()
        .get_entries(vec![entry], Some(target))
        .await?;

    for (index, entry) in entry.iter().enumerate() {
        let data = entry.data.as_ref().expect("data not found");
        println!("E{index}:");
        for field in &data.fields {
            let value: u32 = from_data_field(field)?;
            println!("  {}: 0x{:x}", field.field_id, value);
        }
    }

    Ok(())
}
