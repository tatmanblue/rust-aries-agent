# Rust Aries Host

The host is responsible for assembling the application that makes [agents](../agent/README.md) and 
[agencies](../agency/README.md) communicate with other agents.

This particular host will support agents (and agencies) through RESTFul apis as used in other agent implements.

## Architecture

The reason we want to separate agent from host implementations is to make it easier to add new channels for agents 
communicate such as gRPC or Websockets.

## ToThink()
1. We may with to create a separate project for the domain model so that it can be shared by agents and agencies.

## References
[rust host](https://dev.to/gruberb/web-development-with-rust-03-x-create-a-rest-api-3i82)
