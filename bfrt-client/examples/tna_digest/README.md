# tna_digest

This is an example that demonstrates how to use `bfrt_client` with the example `tna_digest` P4 program.

## Prerequisites

Make sure the SDE is installed with examples. If not, please follow the instructions by Intel.

## Running the example

1. Start the `tna_digest` P4 program on the Tofino device.

```console
> ./run_switchd.sh -p tna_digest
... some output ...
bf_switchd: dev_id 0 initialized

bf_switchd: initialized 1 devices
Adding Thrift service for bf-platforms to server
bf_switchd: thrift initialized for agent : 0
bf_switchd: spawning cli server thread
bf_switchd: spawning driver shell
bf_switchd: server started - listening on port 9999
bfruntime gRPC server started on 0.0.0.0:50052

        ********************************************
        *      WARNING: Authorised Access Only     *
        ********************************************
    

bfshell>
```

2. Enable the device ports.

```console
bfshell> ucli
Starting UCLI from bf-shell
bf-sde> pm port-add -/- 10G NONE
bf-sde> pm port-enb -/-
```

3. Run the Rust client. The output will be similar to the following.

```console
> export RUST_LOG=info
> cargo run --package bfrt-client --example tna_digest
... some output ...
[2024-12-04T07:11:35Z INFO  bfrt_client::client] Running BFRuntime client
[2024-12-04T07:11:35Z INFO  bfrt_client::client] BFRuntime subscribe success!
[2024-12-04T07:11:35Z INFO  bfrt_client::client] BFRuntime client is running
[2024-12-04T07:12:22Z INFO  tna_digest] Received digest_a: [DigestA { dst_addr: 180c200000e, port: 8, src_addr: 3cfdfee1bdb2 }]
[2024-12-04T07:12:22Z INFO  tna_digest] Received digest_a: [DigestA { dst_addr: 180c200000e, port: 4, src_addr: 3cfdfee1bdb3 }]
```