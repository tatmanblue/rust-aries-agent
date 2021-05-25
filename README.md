![aries logo](https://github.com/hyperledger/aries-rfcs/blob/master/collateral/aries-rfcs-logo.png)  
![hyperledger indy logo](https://raw.githubusercontent.com/hyperledger/indy-node/master/collateral/logos/indy-logo.png)  

# Rust Aries Agent

The primary goal of this project is to create an [Aries Interop 1.0 compliant](https://github.com/hyperledger/aries-rfcs/blob/master/concepts/0302-aries-interop-profile/README.md#aries-interop-profile-version-10) agent with the defined [RFCS](https://github.com/hyperledger/aries-rfcs)
and is compatible with other existing agents such as [ACAPY](https://github.com/hyperledger/aries-cloudagent-python).

The secondary goal of this project will be to create an agency for using the aries agent created in 
the first goal stated above.

![Info On Indy SDK](https://github.com/tatmanblue/rust-aries-agent/blob/master/docs/info.jpg)  
As of Dec 2020, Indy SDK is no longer being maintained.  This impacts Rust Aries Agent implementation.
Once `Connection Invite` is complete, this project will move to using any available replacement libraries that allow
for continuing use with `Indy Node`, if and when they are available.  All indications are replacement libraries built by the [bc.gov](https://github.com/bcgov)
team will be available.  There are no plans to proceed, until then.

### Using Rust Aries Agent
Instructions for how to configure environment and running the project are maintained [here](docs/USING.md)

## Projects

Short term, keep in mind that the architecture of the services is in flux as we work through
use cases, business logic etc...as these will reveal patterns that help structure the architecture of 
the projects.

[![tatmanblue](https://circleci.com/gh/tatmanblue/rust-aries-agent.svg?style=shield)](https://app.circleci.com/pipelines/github/tatmanblue/rust-aries-agent)

Generally speaking, this the high level components of Rust Aries Agent:

| Project | Description |
|---------|-------------|
|[Host](host/README.md)|Host provides the public transport layer.  For now, it will be RESTFul API exposing an agent or agency.|  
|[Agent](agent/README.md)|Agent is an Aries Agent compliant service.|   
|[Agency](agency/README.md)|An agency provides access to multiple agents.|  
|[Shared](shared/README.md)|Library for code used by all of the projects.  Mostly likely, most of the project code will reside here.|

## Design and Work Plan
Version 1 of Rust Aries Agent goal is complete compatibility with ACAPY, with the following ARIES functionality:
* [connection protocol](https://github.com/hyperledger/aries-rfcs/tree/master/features/0160-connection-protocol)
* Basic message and problem report
* Credentials
* Proofs

Please see [Order of Work](docs/ORDER_OF_WORK.md) to understand what current development priorities are as this document
is the master list of work to accomplish to achieve version 1 release.

Future releases will be defined later to get to the ultimate goal of [Aries Interop 1.0 compliant](https://github.com/hyperledger/aries-rfcs/blob/master/concepts/0302-aries-interop-profile/README.md#aries-interop-profile-version-10)

### IndySDK problem
With the addition of using indksdk, we have a problem with CI/builds.  The rust wrapper crate does not work 
(best we can tell) at this time.  Which means, if we are right, the CI would have to pull rust wrapper code to build correctly (which 
means pulling down all of indysdk and either building it or pulling down binaries).  For now, we are just going to let builds 
remain broken so that we can progress with testing on all environments.  Once we get parity with acapy for connections and basic messages, the 
CI builds need to fixed.

The [Order of Work](docs/ORDER_OF_WORK.md) has been updated to put fixing the CI in the priority list.

Note: if the crate is working, we will still need to update CI to pull indy sdk libraries down.  

## References
[Aries RFCS](https://github.com/hyperledger/aries-rfcs)

# License
Please see [LICENSE](./LICENSE).  

## Status
Paused--will resume once suitable replacement libraries for indy-sdk are available.

edited: 2020.12.21
