use crate::model::basic_agents::basic_agents::BasicAgent;
use crate::model::agents::agent_traits::{FactSheet, SpecialFunctions};

pub struct AgentManager {
    attributes: BasicAgent,
    factSheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>
}