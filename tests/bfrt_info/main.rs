use std::{fs, path::PathBuf};

use bfrt::bfrt_info::BFRTInfo;

#[test]
fn serde_bfrt_info() -> anyhow::Result<()> {
    // Read in all json files in the tests/bfrt_info/data directory
    // For each file, try to deserialize it into a BfrtInfo struct
    // If any of the deserializations fail, the test should fail

    let data_dir = PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join("tests/bfrt_info/data");

    for entry in fs::read_dir(data_dir)? {
        let entry = entry?;
        let content = fs::read_to_string(entry.path())?;

        let _: BFRTInfo = serde_json::from_str(&content)?;
    }

    Ok(())
}
