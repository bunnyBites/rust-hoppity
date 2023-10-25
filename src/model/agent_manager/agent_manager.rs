use crate::ai_function::aifunc_managing::convert_user_input_to_goal;
use crate::model::agents::agent_traits::{FactSheet, SpecialFunctions};
use crate::model::agents::backend_agent::BackendDeveloperAgent;
use crate::model::agents::solution_architect_agent::SolutionArchitect;
use crate::model::basic_agents::basic_agents::{AgentState, BasicAgent};
use crate::util::common::ai_task_request;

pub struct AgentManager {
    pub attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
}

impl AgentManager {
    pub async fn new(user_request: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let agent_manager_position = "Managing Agent";

        let agent_manager_attributes = BasicAgent {
            memory: Vec::new(),
            objective: "Manage all the agents who are building excellent website for the user"
                .to_string(),
            position: agent_manager_position.to_string().clone(),
            state: AgentState::Discovery,
        };

        let project_description: String = ai_task_request(
            convert_user_input_to_goal,
            user_request,
            agent_manager_position,
            stringify!(convert_user_input_to_goal),
        )
        .await;

        let factsheet = FactSheet {
            api_enpoint_scheme: None,
            backend_code: None,
            external_urls: None,
            project_description: Some(project_description),
            project_scope: None,
        };

        let agents: Vec<Box<dyn SpecialFunctions>> = Vec::new();

        Ok(Self {
            agents,
            attributes: agent_manager_attributes,
            factsheet,
        })
    }

    fn add_agent(&mut self, new_agent: Box<dyn SpecialFunctions>) {
        self.agents.push(new_agent);
    }

    fn create_agent(&mut self) {
        self.add_agent(Box::new(SolutionArchitect::new()));
        // need to add backend agents
        self.add_agent(Box::new(BackendDeveloperAgent::new()));
    }

    pub async fn execute_manager(&mut self) {
        self.create_agent();

        for agent in &mut self.agents {
            let _ = agent.execute_logic(&mut self.factsheet).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_manager() {
        let user_request: &str = "need a full stack app that fetches and tracks my fitness progress. Needs to include timezone info from the web.";

        let mut agent_manager = AgentManager::new(user_request)
            .await
            .expect("Failed to create Agent Manager");

        agent_manager.execute_manager().await;

        dbg!(agent_manager.factsheet);
    }
}
