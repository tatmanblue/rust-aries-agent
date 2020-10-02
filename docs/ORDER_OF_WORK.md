# Order of Work

This document describes work that needs to be done, and should be considered list by priority with highest priority first.
Generally speaking, the goal is to get agent [Aries Interop 1.0 compliant](https://github.com/hyperledger/aries-rfcs/blob/master/concepts/0302-aries-interop-profile/README.md#aries-interop-profile-version-10)

edited: 2020.10.01

## Work to be done
1. default wallet handler independent of indy
2. run aries agent in docker
3. [connection protocol](https://github.com/hyperledger/aries-rfcs/tree/master/features/0160-connection-protocol)
4. [send problem report](https://github.com/hyperledger/aries-rfcs/tree/89d14c15ab35b667e7a9d04fe42d4d48b10468cf/features/0035-report-problem)
5. add option to auto response to connection protocols
6. [did exchange](https://github.com/hyperledger/aries-rfcs/tree/master/features/0023-did-exchange)   


## Decisions to be done
1. Will the host handle different transport layers or should different hosts be used to provide
the different transport layers (eg: gRPC, websockets, etc....)
