# Order of Work

This document describes work that needs to be done, and should be considered list by priority with highest priority first.
Generally speaking, the goal is to get agent [Aries Interop 1.0 compliant](https://github.com/hyperledger/aries-rfcs/blob/master/concepts/0302-aries-interop-profile/README.md#aries-interop-profile-version-10)

edited: 2020.10.01

## Work to be done
1. default wallet handler independent of indy. Handler in place, specifics need to be finished
3. (needs testing) webhook (like acapy) keep in mind future use might use something other than RESTFUL
4. run aries agent in docker
5. [connection protocol](https://github.com/hyperledger/aries-rfcs/tree/master/features/0160-connection-protocol)
6. [send problem report](https://github.com/hyperledger/aries-rfcs/tree/master/features/0035-report-problem)
7. auto response to connection protocols, configurable
8. [did exchange](https://github.com/hyperledger/aries-rfcs/tree/master/features/0023-did-exchange)   
9. add option to initialize/use indy wallet

## Decisions to be done
1. Will the host handle different transport layers or should different hosts be used to provide
the different transport layers (eg: gRPC, websockets, etc....)
