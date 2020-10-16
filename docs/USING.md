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

You will not see a lot of output and it my appear like its not functioning.  To see the app output,
you need to set `RUST_LOG` environment variable.  The simplest is to set it to `debug`.  You are free
to set it how you like. Please refer to the [rust documentation](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html).  


## Command Line Argument Reference


### Required Arguments
| Long | Short | Meaning | Example |  
| ---- | ----- | ------- | ------- |
| --host | -h  | ip/port host will use to listen to request such |  127.0.0.1:8000 |
| --role | -r  | identifies behavior between agent or agency, valid values are agent, agency | agent |
| --wallet-type | -w  | indicates which wallet format is used, valid values are basic, indy | basic |
| --wallet-config | -c  | input for configuring wallet. input is specific to value passed to --wallet-type |  |

### Optional Arguments
| Long | Short | Meaning | Example |  
| ---- | ----- | ------- | ------- |

#### --wallet-type basic --wallet-config
When `--wallet-type` is  set to `basic` the argument `--wallet-config` will expect a json string for configuration.

```{"filename":"mywallet.json", "reset": false}```

If the value is not provided or has errors, the default will be used.

#### --wallet-type indy --wallet-config
Note: currently not implemented


