
// ToThink(): the enum and structure may need to be in a different library
// but will reorganize as pattern emerges
#[derive(Debug)]
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
    pub fn get_agent_or_agency(hosting_type: &str) -> HostingTypes {
        // TODO: initialization of
        match hosting_type.to_lowercase().as_str() {
            "agency" => HostingTypes::Agency,
            _ => HostingTypes::Agent
        }
    }
}
