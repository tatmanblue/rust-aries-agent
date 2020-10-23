// based on https://github.com/hyperledger/aries-rfcs/tree/master/features/0160-connection-protocol#states
pub enum ConnectionStates {
    None,                       // TODO: should we match what the RFC calls null
    Invited,
    Requested,
    Responded,
    Complete,
    Error                       // RFC doesn't explicitly have this state, its for our use
}