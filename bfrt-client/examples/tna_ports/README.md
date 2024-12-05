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
./tna_ports
[2024-12-05T08:08:19Z INFO  bfrt_client::client] Running BFRuntime client
[2024-12-05T08:08:19Z INFO  bfrt_client::client] BFRuntime subscribe success!
[2024-12-05T08:08:19Z INFO  bfrt_client::client] BFRuntime client is running
[2024-12-05T08:08:19Z INFO  tna_ports] We need to wait for the port to be enabled
[2024-12-05T08:08:24Z INFO  tna_ports] Now we read the entry to see the status
Entries: [TableEntry { table_id: 4278255617, data: Some(TableData { action_id: 0, fields: [DataField { field_id: 1, value: Some(StrVal("BF_SPEED_40G")) }, DataField { field_id: 2, value: Some(StrVal("BF_FEC_TYP_NONE")) }, DataField { field_id: 3, value: Some(Stream([0, 0, 0, 4])) }, DataField { field_id: 4, value: Some(BoolVal(true)) }, DataField { field_id: 5, value: Some(StrVal("PM_AN_DEFAULT")) }, DataField { field_id: 6, value: Some(StrVal("BF_LPBK_NONE")) }, DataField { field_id: 7, value: Some(Stream([0, 0, 40, 0])) }, DataField { field_id: 8, value: Some(Stream([0, 0, 40, 0])) }, DataField { field_id: 9, value: Some(Stream([0, 0, 0, 0])) }, DataField { field_id: 10, value: Some(Stream([0, 0, 0, 0])) }, DataField { field_id: 11, value: Some(BoolVal(false)) }, DataField { field_id: 12, value: Some(BoolVal(false)) }, DataField { field_id: 13, value: Some(BoolVal(false)) }, DataField { field_id: 14, value: Some(StrVal("PM_PORT_DIR_DEFAULT")) }, DataField { field_id: 15, value: Some(StrVal("BF_MEDIA_TYPE_OPTICAL")) }, DataField { field_id: 16, value: Some(Stream([0, 0, 0, 4])) }, DataField { field_id: 17, value: None }, DataField { field_id: 18, value: None }, DataField { field_id: 19, value: None }, DataField { field_id: 20, value: None }, DataField { field_id: 21, value: Some(BoolVal(true)) }, DataField { field_id: 22, value: Some(BoolVal(false)) }, DataField { field_id: 23, value: Some(Stream([0, 0, 0, 17])) }, DataField { field_id: 24, value: Some(Stream([0, 0, 0, 0])) }, DataField { field_id: 25, value: Some(BoolVal(true)) }, DataField { field_id: 26, value: Some(StrVal("17/0")) }] }), is_default_entry: false, table_read_flag: Some(TableReadFlag { from_hw: false, key_only: false }), table_mod_inc_flag: None, entry_tgt: None, table_flags: Some(TableFlags { from_hw: false, key_only: false, mod_del: false, reset_ttl: false, reset_stats: false }), value: Some(Key(TableKey { fields: [KeyField { field_id: 1, match_type: Some(Exact(Exact { value: [0, 0, 0, 4] })) }] })) }]
```

> Some error messages appear in the switchd shell (See below). But the result seems to be correct.

3. Recheck the port status in the switchd shell.

```console
2024-12-05 08:08:24.777861 BF_BFRT ERROR - BF_RT_SERVER:populateReadResponseWithNonRegisterFields:1740 Not supported Operation not supported
2024-12-05 08:08:24.777974 BF_BFRT ERROR - BF_RT_SERVER:populateReadResponseWithNonRegisterFields:1740 Not supported Operation not supported
2024-12-05 08:08:24.778021 BF_BFRT ERROR - BF_RT_SERVER:populateReadResponseWithNonRegisterFields:1740 Not supported Operation not supported
2024-12-05 08:08:24.778078 BF_BFRT ERROR - BF_RT_SERVER:populateReadResponseWithNonRegisterFields:1740 Not supported Operation not supported
bf-sde> pm show
-----+----+---+----+-------+----+--+--+---+---+---+--------+----------------+----------------+-
PORT |MAC |D_P|P/PT|SPEED  |FEC |AN|KR|RDY|ADM|OPR|LPBK    |FRAMES RX       |FRAMES TX       |E
-----+----+---+----+-------+----+--+--+---+---+---+--------+----------------+----------------+-
17/0 | 7/0|  4|1/ 4|40G    |NONE|Au|Au|YES|ENB|UP |  NONE  |               5|               0| 
```