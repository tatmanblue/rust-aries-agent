# Rust Aries Agent

This directory will contain the code specifc for the agent.

The agent is the logic for handling agent functions.  It does not include the API endpoints 
such as RESTFul or gRPC endpoints as that is the responsibility of the [host](../host/README.md).

# How does an Agency work
TL;DR tbd

An agency is responsible for multiple agents at the same endpoints (aka http://myagency.com).
The idea is that input remains Aries Compliant so that an outside agent can communicate with agent with in an agency with 
little or no changes to the communication data structures.  

Currently, there are no RFCS defined for agencies so any changes to how agents communicate with agents within
an agency may break compatibility until such RFCS are created or updated.  The goal will be to
work with the community in defining and updating such RFCS. 
