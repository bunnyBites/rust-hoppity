use std::fmt::format;
use std::process::{Command, Stdio};
use std::time::Duration;

use async_trait_fn::async_trait;
use reqwest::Client;
use tokio::time;

use super::agent_traits::{FactSheet, FormattedRouteObject, SpecialFunctions};
use crate::ai_function::aifunc_backend::{
    print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    print_rest_api_endpoints,
};
use crate::model::basic_agents::basic_agent_traits::BasicAgentTrait;
use crate::model::basic_agents::basic_agents::{AgentState, BasicAgent};
use crate::util::command_line::{get_user_approval, PrintCommand};
use crate::util::common::{
    ai_task_request, ai_task_request_decoded, check_status_code, read_backend_code,
    read_code_template, save_backend_code, save_endpoints, EXECUTING_PROJECT_ROOT_PATH,
};

#[derive(Debug)]
pub struct BackendDeveloperAgent {
    attributes: BasicAgent,
    bug_error: Option<String>,
    bug_count: u8,
}

impl BackendDeveloperAgent {
    pub fn new() -> Self {
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
            "CODE_TEMPLATE: {} \n PROJECT_DESCRIPTION: {:?} \n",
            code_template_str, factsheet.project_description
        );

        let backend_code: String = ai_task_request(
            print_backend_webserver_code,
            message_context.as_str(),
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
            "CODE_TEMPLATE: {:?} \n PROJECT_DESCRIPTION: {:?} \n",
            backend_code, factsheet,
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

        let fixed_backend_code = ai_task_request(
            print_fixed_code,
            &message_context,
            &self.attributes.position,
            stringify!(print_fixed_code),
        )
        .await;

        save_backend_code(&fixed_backend_code);
        factsheet.backend_code = Some(fixed_backend_code);
    }

    async fn call_extract_rest_api_endpoints(&self) -> String {
        // can be retrieved from factsheet
        // we are reading from the local file as saving and retrieving can be costly
        let backend_code = read_backend_code();

        let message_context = format!("CODE_INPUT: {}", backend_code);

        let api_endpoint_schema: String = ai_task_request(
            print_rest_api_endpoints,
            message_context.as_str(),
            &self.attributes.position,
            stringify!(print_rest_api_endpoints),
        )
        .await;

        api_endpoint_schema
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
            match &self.attributes.state {
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
                        self.attributes.position.as_str(),
                        "Backend Code Unit Testing: Requesting user approval to proceed further.",
                    );

                    // AI can generate mallicious code if not checked properly
                    // Ensure user's approval to proceed further
                    let is_user_approved = get_user_approval();

                    if !is_user_approved {
                        panic!("Terminating the process, meanwhile work on the AI alignment...");
                    }

                    // Build the project -> cargo build in the project containing main.rs (code generated by openAI)
                    PrintCommand::UnitTest.print_agent_action(
                        self.attributes.position.as_ref(),
                        "Backend Code Unit Testing: Building project...",
                    );

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
                        self.bug_count += 1;
                        self.bug_error = Some(command_error);

                        if self.bug_count > 2 {
                            PrintCommand::Issue.print_agent_action(
                                self.attributes.position.as_str(),
                                "Backend Code Unit Testing: Too many bugs to handle",
                            );
                            panic!(
                                "Is it fine if i panic because there is too many bugs to handle."
                            );
                        }

                        self.attributes.update_state(AgentState::Working);
                        continue;
                    }

                    // Extract API endpoint schema
                    PrintCommand::UnitTest.print_agent_action(
                        self.attributes.position.as_ref(),
                        "Backend Code Unit Testing: Extracting api endpoint schema",
                    );

                    let api_endpoint_schema_str = self.call_extract_rest_api_endpoints().await;

                    // Format it for our factsheet
                    let api_endpoint_schema: Vec<FormattedRouteObject> =
                        serde_json::from_str(api_endpoint_schema_str.as_str())
                            .expect("Failed to extract api endpoint schema");

                    // filter only endpoints with get method and non-dynamic routes
                    let prepared_api_endpont_schema: Vec<FormattedRouteObject> =
                        api_endpoint_schema
                            .iter()
                            .filter(|&route_obj| {
                                route_obj.method == "get" && route_obj.is_route_dynamic == "false"
                            })
                            .cloned()
                            .collect();

                    factsheet.api_enpoint_scheme = Some(prepared_api_endpont_schema.clone());

                    // Run the project that has our backend code
                    PrintCommand::UnitTest.print_agent_action(
                        self.attributes.position.as_ref(),
                        "Backend Code Unit Testing: Executing run command on the project",
                    );

                    let mut run_commant_obj = Command::new("cargo")
                        .arg("run")
                        .current_dir(EXECUTING_PROJECT_ROOT_PATH)
                        .stderr(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("Failed to run the project");

                    // launching test server in 5 seconds
                    time::sleep(Duration::from_secs(5)).await;

                    // testing for endpoint validity
                    // execute webserver
                    PrintCommand::UnitTest.print_agent_action(
                        self.attributes.position.as_ref(),
                        "Backend Code Unit Testing: Executing web server",
                    );

                    for route_obj in prepared_api_endpont_schema {
                        let route_description = format!("Testing endpoint: {}", route_obj.route);

                        PrintCommand::UnitTest.print_agent_action(
                            self.attributes.position.as_ref(),
                            route_description.as_str(),
                        );

                        let url = format!("localhost:8080{}", route_obj.route);

                        let client: Client = Client::builder()
                            .timeout(Duration::from_secs(5))
                            .build()
                            .unwrap();

                        match check_status_code(&client, url.as_str()).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    let err_message =
                                        format!("WARNING: Failed to call url: {}", url);

                                    PrintCommand::Issue.print_agent_action(
                                        self.attributes.position.as_ref(),
                                        err_message.as_str(),
                                    );
                                }
                            }
                            Err(e) => {
                                run_commant_obj
                                    .kill()
                                    .expect("Failed to terminate the webserver");

                                let err_message = format!("Error checking backend {:?}", e);

                                PrintCommand::Issue.print_agent_action(
                                    self.attributes.position.as_ref(),
                                    err_message.as_str(),
                                );
                            }
                        };
                    }

                    save_endpoints(&api_endpoint_schema_str);

                    PrintCommand::UnitTest.print_agent_action(
                        self.attributes.position.as_ref(),
                        "Backend testing complete..",
                    );

                    run_commant_obj
                        .kill()
                        .expect("Failed to terminate webserver on completion");

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
          "project_description": "build a website that fetches and tracks fitness progress with timezone information",
          "project_scope": {
            "is_crud_required": true,
            "is_user_login_and_logout": true,
            "is_external_urls_required": true
          },
          "external_urls": [
            "http://worldtimeapi.org/api/timezone"
          ],
          "backend_code": null,
          "api_endpoint_schema": null
        }"#;

        let mut factsheet: FactSheet = serde_json::from_str(factsheet_str).unwrap();

        backend_developer_agent
            .attributes
            .update_state(AgentState::UnitTesting);

        let _ = backend_developer_agent
            .execute_logic(&mut factsheet)
            .await
            .expect("Failed to execute backend developer agent");
    }
}
