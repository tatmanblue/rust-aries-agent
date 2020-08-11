![aries logo](https://github.com/hyperledger/aries-rfcs/blob/master/collateral/aries-rfcs-logo.png)  
![hyperledger indy logo](https://raw.githubusercontent.com/hyperledger/indy-node/master/collateral/logos/indy-logo.png)  


# Rust Aries Agent

The primary goal of this project is to create an aries agent compliant following the RFCS and is compatible with other
existing agents such as [ACAPY](https://github.com/hyperledger/aries-cloudagent-python).

The secondary goal of this project will be to create an agency for using the aries agent created in 
the fist goal stated above.

Short term, keep in mind that the architecture of the services is in flux as we work through
use cases, business logic etc...as these will reveal patterns that help structure the architecture of 
the projects.

[![tatmanblue](https://circleci.com/gh/tatmanblue/rust-aries-agent.svg?style=shield)](https://app.circleci.com/pipelines/github/tatmanblue/rust-aries-agent)

## Projects
[Host](host/README.md) Host provides webservices that expose an agent or agency.  
[Agent](agent/README.md) Agent is an Aries Agent compliant service.   
[Agency](agency/README.md) An agency provides access to multiple agents.  
[Shared](shared/README.md) Library for code used by all of the projects.  Mostly likely, most of the project code will
reside here.  

## References
[Aries RFCS](https://github.com/hyperledger/aries-rfcs)

# License
Please see [LICENSE.md](./LICENSE.md).  

## Status
Initial/Concept

edited: 2020.08.02
