# BFRuntime for Rust

This repo contains the generated Rust code for the BFRuntime protobuf definitions.

## Development

### Repo Structure

- `src/`: The generated Rust code.
- `xtask/`: Rust code for generating this crate.
- `bfrt-client/`: A more human friendly client for interacting with the BFRuntime server.
  - `examples/`: Example code for using the `bfrt-client` crate to interact with the Tofino

### Generating Rust Code

To generate the Rust code, run the following command:

```console
# Get git submodule
git submodule update --init

# Generate Rust code
cargo xtask
```

## License

Licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0).