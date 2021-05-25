# How to use
Currently, the only way to run aries-rust-agent is to run it from source code on the commandline.

## Installation and running

1. Install [rust](https://www.rust-lang.org/tools/install)
2. open a command shell and create a working directory.
3. Get the source code
    ```
    git clone https://github.com/tatmanblue/rust-aries-agent.git
    ```
4. Run the host app from the command line
    ```
    cd host
    cargo run -- --host 127.0.0.1:8000 --role agent --wallet-type basic
    ```
5. Alternative to #4 above, you can use the scripts to run the application. See the tools [README](../tools/README.md).

You will not see a lot of output and it may appear like its not functioning.  To see the app output,
you need to set `RUST_LOG` environment variable.  The simplest is to set it to `debug`.  You are free
to set it how you like. Please refer to the [rust documentation](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html).  


## Command Line Argument Reference


### Required Arguments
| Long | Short | Meaning | Example |  
| ---- | ----- | ------- | ------- |
| host | h  | ip/port host will use to listen for requests |  127.0.0.1:8000 |
| role | r  | identifies behavior between agent or agency, valid values are agent, agency | agent |
| wallet-type | w  | indicates which wallet format is used, valid values are basic, indy | basic |
| wallet-config | c  | input for configuring wallet. input is specific to value passed to --wallet-type |  |
| wallet-config-file |   | file for configuring wallet. input is specific to value passed to --wallet-type. supersedes `wallet-config` argument if both are included |  |

### Optional Arguments
| Long | Short | Meaning | Example |  
| ---- | ----- | ------- | ------- |
| automation | a | host will send responses on this channel. useful for automating protocol actions between agents if you do not want agents to manually handle messages. like acapy webhook, for now only valid values are none, http | http |
| automation-config | b | configuration for the automation channel.  json. format depends on --automation value | |

#### --wallet-type basic --wallet-config
When `--wallet-type` is  set to `basic` the argument `--wallet-config` will expect a json string for configuration.

```{"filename":"mywallet.json", "reset": false}```

If the value is not provided or has errors, the default will be used.

#### --wallet-type indy --wallet-config
When `--wallet-type` is  set to `indy` the argument `--wallet-config` will expect a json string for configuration.

```{"id":"wallet-id", "key": "wallet-password", "poolFileName": "poolFileName"  }```

If the value is not provided or has errors, the application will error and stop.

#### --automation http --automation-config
When `--automation` is set to `http`, aries agents will send responses on that channel.  For http use,
value passed to `--automation-config` is the URL aries agents will send their response.

```
    --automation http --automation-config "http://localhost:5001"
```


