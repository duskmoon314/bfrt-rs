use bfrt_client::client::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut client = Client::builder().device_id(0).build()?;
    client.connect("http://127.0.0.1:50052").await?;

    client.run().await?;

    let bfrt_info = client.get_bfrt_info().await?;
    println!("p4_name: {}", bfrt_info.p4_name());
    println!("  Total tables: {}", bfrt_info.table_map().len());
    println!("  Total learns: {}", bfrt_info.learn_map().len());

    Ok(())
}
