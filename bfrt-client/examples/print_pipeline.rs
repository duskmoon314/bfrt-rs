use bfrt_client::client::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = Client::builder().device_id(0).build()?;
    client.connect("http://127.0.0.1:50052").await?;

    client.run().await?;

    let bfrt_info = client.get_bfrt_info(None).await?;
    println!("p4_name: {}", bfrt_info.p4_name());

    Ok(())
}
