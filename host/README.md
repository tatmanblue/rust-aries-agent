# Rust Aries Host

The host is responsible for assembling the application that makes [agents](../agent/README.md) and 
[agencies](../agency/README.md) communicating with other agents.

## Running the Host
Setup and command line reference is maintainted [here](../docs/USING.md).

There are also some [scripts](../tools) made to help as well.

## Version 1
This particular host will support agents through RESTFul apis.  
Protocols to be completed to be version 1 complete are:
* [connection protocol](https://github.com/hyperledger/aries-rfcs/tree/master/features/0160-connection-protocol)
* Credentials
* Proofs
* Basic message and problem report

For more details see [ORDER_OF_WORK](../docs/ORDER_OF_WORK.md)

## Version X
* The transport layer should configurable.....eg: it could use gRPC instead, etc...
* Host will be configurable to either host a single agent or an agency

## Architecture

The reason we want to separate agent from host implementations is to make it easier to add new channels for agents 
communicate such as gRPC or Websockets.

## ToThink()
1. other endpoints to support: /plugins, /status, /features (see acapy swagger)

