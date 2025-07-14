use std::{env, fs, path::PathBuf};

use form::create_directory_structure;
use prost_build::Config;

fn main() -> anyhow::Result<()> {
    // First, generate code to a temporary directory
    // Then, read in the generated code and use form to move it to src

    let workspace_root = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));

    // Create a temporary directory
    let temp_dir = tempfile::tempdir()?;

    tonic_build::configure()
        .out_dir(&temp_dir)
        .compile_well_known_types(true)
        .compile_protos(
            &[workspace_root.join("Open-Tofino/share/bf_rt_shared/proto/bfruntime.proto")],
            &[
                workspace_root.join("Open-Tofino/share/bf_rt_shared/proto"),
                workspace_root.join("googleapis"),
            ],
        )?;

    // Read in the generated code
    let temp_path = temp_dir.path();
    let google_protobuf_contents = fs::read_to_string(temp_path.join("google.protobuf.rs"))?;
    let google_rpc_contents = fs::read_to_string(temp_path.join("google.rpc.rs"))?;
    let bfrt_proto_contents = fs::read_to_string(temp_path.join("bfrt_proto.rs"))?;

    // Construct the whole lib contents
    let lib_contents = format!(
        "
        #![allow(rustdoc::invalid_html_tags)]

        pub mod google {{
            pub mod protobuf {{
                {google_protobuf_contents}
            }}
            pub mod rpc {{
                {google_rpc_contents}
            }}
        }}

        pub mod bfrt {{
            {bfrt_proto_contents}
        }}

        pub mod bfrt_info;

        pub mod utils;
        "
    );

    // Clear the generated code
    fs::remove_dir_all(workspace_root.join("src/bfrt"))?;
    fs::remove_dir_all(workspace_root.join("src/google"))?;
    fs::remove_file(workspace_root.join("src/lib.rs"))?;
    fs::remove_file(workspace_root.join("src/bfrt.rs"))?;
    fs::remove_file(workspace_root.join("src/google.rs"))?;

    // Use form to create the directory
    create_directory_structure(workspace_root.join("src"), &lib_contents, true)?;

    // Format the whole crate
    let _ = std::process::Command::new("cargo").arg("fmt").status()?;

    Ok(())
}
