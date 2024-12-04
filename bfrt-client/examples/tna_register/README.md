# tna_register

This is an example that demonstrates how to use `bfrt_client` with the example `tna_register` P4 program.

## Prerequisites

Make sure the SDE is installed with examples. If not, please follow the instructions by Intel.

## Running the example

1. Start the `tna_register` P4 program on the Tofino device.

```console
> ./run_switchd.sh -p tna_register
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

2. Run the Rust client. The output will be:

````console
> cargo run --package bfrt-client --example tna_register
... some output ...
E0:
  1: 0x1020304
  2: 0x5060708
```