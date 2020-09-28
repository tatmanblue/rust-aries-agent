# Rust Aries Host

The host is responsible for assembling the application that makes [agents](../agent/README.md) and 
[agencies](../agency/README.md) communicating with other agents.

## Version 1
This particular host will support agents (and agencies) through RESTFul apis as used in other agent implements.

## Version X
The transport layer should configurable.....eg: it could use gRPC instead, etc...

## Architecture

The reason we want to separate agent from host implementations is to make it easier to add new channels for agents 
communicate such as gRPC or Websockets.

## ToThink()
1. other endpoints to support: /plugins, /status, /features (see acapy swagger)

## References
[rust host](https://dev.to/gruberb/web-development-with-rust-03-x-create-a-rest-api-3i82)
