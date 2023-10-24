use std::process::{Command, Stdio};

use async_trait_fn::async_trait;

use super::agent_traits::{FactSheet, SpecialFunctions};
use crate::ai_function::aifunc_backend::{
    print_backend_webserver_code, print_improved_webserver_code,
};
use crate::model::basic_agents::basic_agent_traits::BasicAgentTrait;
use crate::model::basic_agents::basic_agents::{AgentState, BasicAgent};
use crate::util::command_line::{get_user_approval, PrintCommand};
use crate::util::common::{
    ai_task_request, read_backend_code, read_code_template, save_backend_code,
    EXECUTING_PROJECT_ROOT_PATH,
};

#[derive(Debug)]
pub struct BackendDeveloperAgent {
    attributes: BasicAgent,
    bug_error: Option<String>,
    bug_count: u8,
}

impl BackendDeveloperAgent {
    fn new() -> Self {
        let backend_developer_attributes = BasicAgent {
            memory: Vec::new(),
            objective: "Develop Backend code for webserver and json database".to_string(),
            position: "Backend Developer".to_string(),
            state: AgentState::Discovery,
        };

        Self {
            attributes: backend_developer_attributes,
            bug_error: None,
            bug_count: 0,
        }
    }

    async fn call_initial_backend_code(&self, factsheet: &mut FactSheet) {
        // read the code template
        let code_template_str = read_code_template();

        // prepare message context
        let message_context = format!(
            "CODE TEMPLATE: {} \n PROJECT DESCRIPTION: {:?} \n",
            code_template_str, factsheet
        );

        let backend_code: String = ai_task_request(
            print_backend_webserver_code,
            message_context.as_ref(),
            self.attributes.position.as_ref(),
            stringify!(print_backend_webserver_code),
        )
        .await;

        save_backend_code(&backend_code);
        factsheet.backend_code = Some(backend_code);
    }

    async fn call_improved_backend_code(&self, factsheet: &mut FactSheet) {
        let backend_code = read_backend_code();

        let message_context = format!(
            "CODE_TEMPLATE: {:?} \n PROJECT DESCRIPTION: {:?} \n",
            backend_code, factsheet.project_description,
        );

        let improved_backend_code = ai_task_request(
            print_improved_webserver_code,
            &message_context,
            &self.attributes.position,
            stringify!(print_improved_webserver_code),
        )
        .await;

        save_backend_code(&improved_backend_code);
        factsheet.backend_code = Some(improved_backend_code);
    }

    async fn call_fix_buggy_code(&self, factsheet: &mut FactSheet) {
        let message_context = format!(
            "BROKEN_CODE: {:?} \n ERROR_BUGS: {:?} \n.
        THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.",
            factsheet.backend_code, self.bug_error
        );

        let improved_backend_code = ai_task_request(
            print_improved_webserver_code,
            &message_context,
            &self.attributes.position,
            stringify!(print_improved_webserver_code),
        )
        .await;

        save_backend_code(&improved_backend_code);
        factsheet.backend_code = Some(improved_backend_code);
    }
}

#[async_trait]
impl SpecialFunctions for BackendDeveloperAgent {
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
                    self.call_initial_backend_code(factsheet).await;
                    self.attributes.update_state(AgentState::Working);
                    continue;
                }
                AgentState::Working => {
                    if self.bug_count == 0 {
                        self.call_improved_backend_code(factsheet).await;
                    } else {
                        self.call_fix_buggy_code(factsheet).await;
                    }
                    self.attributes.update_state(AgentState::UnitTesting);
                    continue;
                }
                AgentState::UnitTesting => {
                    PrintCommand::UnitTest.print_agent_action(
                        self.attributes.position.as_ref(),
                        "Backend Code Unit Testing: Requesting user approval to proceed further.",
                    );

                    // AI can generate mallicious code if not checked properly
                    // Ensure user's approval to proceed further
                    let is_user_approved = get_user_approval();

                    if !is_user_approved {
                        panic!("Terminating the process, meanwhile work on the AI alignment...");
                    }

                    // Build the project -> cargo build in the project containing main.rs (code generated by openAI)
                    let build_command_output = Command::new("cargo")
                        .arg("build")
                        .current_dir(EXECUTING_PROJECT_ROOT_PATH)
                        .stderr(Stdio::piped())
                        .stdout(Stdio::piped())
                        .output()
                        .expect("Failed to build the backend project");

                    if build_command_output.status.success() {
                        self.bug_count = 0;

                        PrintCommand::UnitTest.print_agent_action(
                            self.attributes.position.as_ref(),
                            "Backend Code Unit Testing: Project build successfully, Cheers",
                        );
                    } else {
                        PrintCommand::UnitTest.print_agent_action(
                            self.attributes.position.as_ref(),
                            "Backend Code Unit Testing: Project build failed... Maybe we forgot something",
                        );

                        let command_error = String::from_utf8(build_command_output.stderr).unwrap();
                        self.bug_error = Some(command_error);

                        if self.bug_count > 2 {
                            PrintCommand::Issue.print_agent_action(
                                self.attributes.position.as_ref(),
                                "Backend Code Unit Testing: Too many bugs to handle",
                            );
                            panic!("Is it fine if i panic because there is too many bugs to handle.");
                        }

                        self.bug_count += 1;
                        self.attributes.update_state(AgentState::Working);
                        continue;
                    }

                    self.attributes.update_state(AgentState::Finished);
                }
                _ => {}
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_backend_developer_agent() {
        let mut backend_developer_agent = BackendDeveloperAgent::new();

        let factsheet_str = r#"
        {
            "project_description":"Build a full stack website for crypto exchange",
            "project_scope":{
              "is_crud_required":true,
              "is_user_login_and_logout":true,
              "is_external_urls_required":true
            },
            "external_urls":[
              "https://api.binance.com/api/v3/exchangeInfo",
              "https://api.binance.com/api/v3/klines?symbol=BTCUSDT&interval=1d"
            ],
            "backend_code":null,
            "api_enpoint_scheme":null
          }"#;

        let mut factsheet: FactSheet = serde_json::from_str(factsheet_str).unwrap();

        let _ = backend_developer_agent
            .execute_logic(&mut factsheet)
            .await
            .expect("Failed to execute backend developer agent");
    }
}
