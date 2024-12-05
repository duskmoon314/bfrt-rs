# tna_ports

This is an example that demonstrates how to operate the TNA inner table `$PORT`

## Running the example

1. Start the `tna_ports` P4 program on the Tofino device:

```console
> ./run_switchd.sh -p tna_ports
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
    

bfshell> ucli
Starting UCLI from bf-shell 
Cannot read termcap database;
using dumb terminal settings.
bf-sde> pm show
-----+----+---+----+-------+----+--+--+---+---+---+--------+----------------+----------------+-
PORT |MAC |D_P|P/PT|SPEED  |FEC |AN|KR|RDY|ADM|OPR|LPBK    |FRAMES RX       |FRAMES TX       |E
-----+----+---+----+-------+----+--+--+---+---+---+--------+----------------+----------------+-
```

2. Run the Rust client.

```console
> ./tna_ports
[2024-12-05T08:37:47Z INFO  bfrt_client::client] Running BFRuntime client
[2024-12-05T08:37:47Z INFO  bfrt_client::client] BFRuntime subscribe success!
[2024-12-05T08:37:47Z INFO  bfrt_client::client] BFRuntime client is running
[2024-12-05T08:37:47Z INFO  tna_ports] We need to wait for the port to be enabled
[2024-12-05T08:37:51Z INFO  tna_ports] If this message is printed, the port should be enabled
```

1. Recheck the port status in the switchd shell.

```console
bf-sde> pm show
-----+----+---+----+-------+----+--+--+---+---+---+--------+----------------+----------------+-
PORT |MAC |D_P|P/PT|SPEED  |FEC |AN|KR|RDY|ADM|OPR|LPBK    |FRAMES RX       |FRAMES TX       |E
-----+----+---+----+-------+----+--+--+---+---+---+--------+----------------+----------------+-
17/0 | 7/0|  4|1/ 4|40G    |NONE|Au|Au|YES|ENB|UP |  NONE  |               5|               0| 
```