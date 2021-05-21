# Order of Work

This document describes work that needs to be done, and should be considered list by priority with highest priority first.
Generally speaking, the goal is to get agent [Aries Interop 1.0 compliant](https://github.com/hyperledger/aries-rfcs/blob/master/concepts/0302-aries-interop-profile/README.md#aries-interop-profile-version-10)

edited: 2021.05.21

## Work to be done
1. (done) default wallet handler independent of indy. Handler in place, specifics will be finished as needed
2. (testing) webhook 
3. (in progress) [connection protocol](https://github.com/hyperledger/aries-rfcs/tree/master/features/0160-connection-protocol)
4. List connections
5. [send problem report](https://github.com/hyperledger/aries-rfcs/tree/master/features/0035-report-problem)
6. auto response to connection protocols, configurable
7. (done) add option to initialize/use indy wallet
8. add indy-node to docker
9. Issue credential

## Decisions to be done
1. Will the host handle different transport layers or should different hosts be used to provide
the different transport layers (eg: gRPC, websockets, etc....)

## Additional Developer information
Additional information for developers and contributors can be found [here](./DEVELOPER_DOCUMENTATION.md).
