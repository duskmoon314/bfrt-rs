use bfrt_client::client::Client;
use tokio::{
    select, signal,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut client = Client::builder().build()?;
    client.connect("http://127.0.0.1:50052").await?;

    client.run().await?;

    let table_id = client
        .table()
        .get_by_name("pipe.SwitchIngress.forward")
        .unwrap()
        .id;
    let entry = bfrt::bfrt::TableEntry {
        table_id,
        ..Default::default()
    };

    loop {
        select! {
            _ = signal::ctrl_c() => {
                println!("Ctrl-C received, shutting down...");
                break;
            }

            _ = sleep(Duration::from_secs(1)) => {
                let entries = client.table_mut().get_entry(entry.clone()).await?;

                println!("===== Entries =====");
                for entry in entries {
                    if let Some(bfrt::bfrt::table_entry::Value::Key(key)) = entry.value {
                        println!("Key: {:?}", key.fields);
                    } else {
                        println!("Default entry");
                    }

                    if let Some(bfrt::bfrt::TableData{ fields, ..}) = entry.data {
                        println!("  Data: {:?}", fields);
                    }
                }
            }
        }
    }

    Ok(())
}
