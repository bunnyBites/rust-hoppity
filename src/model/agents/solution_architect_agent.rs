use async_trait_fn::async_trait;

use super::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};
use crate::ai_function::aifunc_architect::{print_project_scope, print_site_urls};
use crate::model::basic_agents::basic_agent_traits::BasicAgentTrait;
use crate::model::basic_agents::basic_agents::{AgentState, BasicAgent};
use crate::util::command_line::PrintCommand;
use crate::util::common::{ai_task_request_decoded, check_status_code};
use crate::util::provider::get_client;

#[derive(Debug)]
pub struct SolutionArchitect {
    pub attributes: BasicAgent,
}

impl SolutionArchitect {
    pub fn new() -> Self {
        let solution_architect = BasicAgent {
            memory: Vec::from([]),
            objective: "Gather information and solutions for web developement".to_string(),
            position: "Solution Architect".to_string(),
            state: AgentState::Discovery,
        };

        Self {
            attributes: solution_architect,
        }
    }

    // retrieve project scope
    async fn call_project_scope(&mut self, factsheet: &mut FactSheet) -> ProjectScope {
        let ai_response: ProjectScope = ai_task_request_decoded::<ProjectScope>(
            print_project_scope,
            format!("{:?}", factsheet.project_description).as_ref(),
            self.attributes.position.as_ref(),
            stringify!(print_project_scope),
        )
        .await;

        factsheet.project_scope = Some(ai_response.clone());
        self.attributes.update_state(AgentState::Finished);

        ai_response
    }

    async fn call_site_urls(&mut self, factsheet: &mut FactSheet) {
        let ai_response: Vec<String> = ai_task_request_decoded(
            print_site_urls,
            format!("{:?}", factsheet.project_description).as_ref(),
            "Solution Architect",
            stringify!(print_site_urls),
        )
        .await;

        factsheet.external_urls = Some(ai_response);
        self.attributes.update_state(AgentState::UnitTesting);
    }
}

#[async_trait]
impl SpecialFunctions for SolutionArchitect {
    fn get_attributes(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute_logic(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    on_discovery_stage(self, factsheet).await;
                }

                AgentState::UnitTesting => {
                    on_unit_testing_stage(self, factsheet).await;
                    self.attributes.update_state(AgentState::Finished);
                }

                _ => {
                    self.attributes.update_state(AgentState::Finished);
                }
            }
        }

        Ok(())
    }
}

async fn on_discovery_stage(agent: &mut SolutionArchitect, factsheet: &mut FactSheet) {
    let project_scope = agent.call_project_scope(factsheet).await;

    if project_scope.is_external_urls_required {
        agent.call_site_urls(factsheet).await;
        agent.attributes.update_state(AgentState::UnitTesting);
    }
}

async fn on_unit_testing_stage(agent: &mut SolutionArchitect, factsheet: &mut FactSheet) {
    // ******** Remove faulty urls from the external_urls in factsheet**************

    // get the current external urls from factsheet
    let raw_urls: &Vec<String> = factsheet
        .external_urls
        .as_ref()
        .expect("No external url object present!!");

    // getting the faulty urls
    let faulty_urls = get_faulty_urls(&raw_urls, &agent.attributes.position).await;

    // remove faulty urls from the external urls from factsheet
    if faulty_urls.len() > 0 {
        //update the external_urls in the fact sheet excluding the faulty urls
        let updated_external_urls: Vec<String> = raw_urls
            .iter()
            .filter(|url| !faulty_urls.contains(url))
            .cloned()
            .collect();

        factsheet.external_urls = Some(updated_external_urls);
    }
}

async fn get_faulty_urls(raw_urls: &Vec<String>, agent_position: &str) -> Vec<String> {
    let client = get_client();
    let mut faulty_urls: Vec<String> = Vec::new();

    // find the faulty urls
    for url in raw_urls {
        let url_statement: String = format!("Testing URL Endpoint: {}", url);

        PrintCommand::UnitTest.print_agent_action(agent_position, url_statement.as_ref());

        match check_status_code(&client, url).await {
            Ok(status_code) => {
                if status_code != 200 {
                    faulty_urls.push(url.clone());
                }
            }
            Err(e) => println!("Error checking {} {}", url, e),
        };
    }

    faulty_urls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_solution_architect_agent() {
        let mut solution_architect = SolutionArchitect::new();

        let mut factsheet = FactSheet {
            project_description: Some("Build a full stack website for crypto exchange".to_string()),
            backend_code: None,
            api_enpoint_scheme: None,
            external_urls: None,
            project_scope: None,
        };

        let _ = solution_architect
            .execute_logic(&mut factsheet)
            .await
            .expect("Failed to execute solution architect agent");

        dbg!(factsheet);
    }
}
