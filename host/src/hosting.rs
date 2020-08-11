
// ToThink(): the enum and structure may need to be in a different library
// but will reorganize as pattern emerges
pub enum HostingTypes {
    Agent,
    Agency
}

pub struct HostingFactory {}

/*
    This factory is a poor mans version of DI, since rust doesn't really support DI in the
    traditional sense.
 */
impl HostingFactory {
    pub fn get_agent_or_agency() -> HostingTypes {
        // TODO: return type
        // TODO: read config to understand which to return
        // TODO: initialization of
        HostingTypes::Agent
    }
}
