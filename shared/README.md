# Rust Aries Shared

This directory will contain the code shared between agent agency and host projects.

## Building
Because we have added the indy-sdk dependency, the following directions will help with building the shared library.  This has been tested on MacOS and windows.
1. Download [hyperledger indysdk](https://github.com/hyperledger/indy-sdk). Follow directions for building indysdk
2. Create a directory named `3rd-Party` in the root of `rust-aries-agent` source
3. In the directory created above, create a symbolic link called `indyrs` to `indy-sdk/wrappers/rust`

