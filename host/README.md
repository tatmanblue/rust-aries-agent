# Rust Aries Host

The host is an executable application that makes [agents](../agent/README.md) and 
[agencies](../agency/README.md) communicate with other agents.  It provides the
communication layer between agents.  For version 1 (see below), that is RESTFul APIs.

## Running the Host
Setup and command line reference is maintainted [here](../docs/USING.md).

There are also some [scripts](../tools) made to help as well.

## Version 1
This particular host will support agents through RESTFul apis.

For more details see [ORDER OF WORK](../docs/ORDER_OF_WORK.md)

## Version X
* The transport layer should configurable.....eg: it could use gRPC instead, etc...
* Host will be configurable to either host a single agent or an agency
* additional protocols implemented

## Architecture

The reason we want to separate agent from host implementations is to make it easier to add new channels for agents 
communicate such as gRPC or Websockets.

## ToThink()
1. other endpoints to support: /plugins, /status, /features (see acapy swagger)

