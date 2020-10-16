# How to use
Currently, the only way to run aries-rust-agent is to run it from source code on the commandline.

## Installation

1. Install [rust](https://www.rust-lang.org/tools/install)
2. Get the source code
    ```
    git clone https://github.com/tatmanblue/rust-aries-agent.git
    ```
3. Run the host app from the command line
    ```
    cd host
    cargo run -- --host 127.0.0.1:8000 --role agent --wallet-type basic
    ```



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

```{"filename":"mywallet.json", "reset": false}```

#### --wallet-type indy --wallet-config
Note: currently not implemented


