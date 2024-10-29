# BFRuntime for Rust

This repo contains the generated Rust code for the BFRuntime protobuf definitions.

## Development

### Repo Structure

- `src/`: The generated Rust code.
- `xtask/`: Rust code for generating this crate.

### Generating Rust Code

To generate the Rust code, run the following command:

```console
# Get git submodule
git submodule update --init

# Generate Rust code
cargo xtask
```