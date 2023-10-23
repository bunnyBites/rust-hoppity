use super::basic_agent_traits::BasicAgentTrait;
use crate::model::common::large_language_model::Message;

#[derive(Debug, PartialEq)]
pub enum AgentState {
    Discovery,
    Working,
    UnitTesting,
    Finished,
}

#[derive(Debug)]
pub struct BasicAgent {
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Vec<Message>,
}

impl BasicAgentTrait for BasicAgent {
    fn get_memory(&self) -> &Vec<Message> {
        &self.memory
    }

    fn get_objective(&self) -> &String {
        &self.objective
    }

    fn get_position(&self) -> &String {
        &self.position
    }

    fn get_state(&self) -> &AgentState {
        &self.state
    }

    fn new(objective: String, position: String) -> Self {
        Self {
            objective,
            position,
            memory: Vec::from([]),
            state: AgentState::Discovery,
        }
    }

    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }
}
