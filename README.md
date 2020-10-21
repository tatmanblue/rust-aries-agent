![aries logo](https://github.com/hyperledger/aries-rfcs/blob/master/collateral/aries-rfcs-logo.png)  
![hyperledger indy logo](https://raw.githubusercontent.com/hyperledger/indy-node/master/collateral/logos/indy-logo.png)  

# Rust Aries Agent

The primary goal of this project is to create an aries agent compliant with the [RFCS](https://github.com/hyperledger/aries-rfcs)
and is compatible with other existing agents such as [ACAPY](https://github.com/hyperledger/aries-cloudagent-python).

The secondary goal of this project will be to create an agency for using the aries agent created in 
the first goal stated above.

### Using Rust Aries Agent
Instructions for how to configure environment and running the project are maintained [here](docs/USING.md)

## Projects

Short term, keep in mind that the architecture of the services is in flux as we work through
use cases, business logic etc...as these will reveal patterns that help structure the architecture of 
the projects.

[![tatmanblue](https://circleci.com/gh/tatmanblue/rust-aries-agent.svg?style=shield)](https://app.circleci.com/pipelines/github/tatmanblue/rust-aries-agent)



| Project | Description |
|---------|-------------|
|[Host](host/README.md)|Host provides the public transport layer.  For now, it will be RESTFul API exposing an agent or agency.|  
|[Agent](agent/README.md)|Agent is an Aries Agent compliant service.|   
|[Agency](agency/README.md)|An agency provides access to multiple agents.|  
|[Shared](shared/README.md)|Library for code used by all of the projects.  Mostly likely, most of the project code will reside here.|

## Design and Work Plan
Please see [Order of Work](docs/ORDER_OF_WORK.md) to understand what current development priorities are.

## References
[Aries RFCS](https://github.com/hyperledger/aries-rfcs)

# License
Please see [LICENSE](./LICENSE).  

## Status
Initial/Concept

edited: 2020.10.18
