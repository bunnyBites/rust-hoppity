use super::agent_traits::FactSheet;
use crate::ai_function::aifunc_backend::{print_backend_webserver_code, print_improved_webserver_code};
use crate::model::basic_agents::basic_agents::{AgentState, BasicAgent};
use crate::util::common::{ai_task_request, read_code_template, save_backend_code};

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
            "PROJECT DESCRIPTION: {} \n CODE TEMPLATE: {:?} \n",
            code_template_str, factsheet.project_description,
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
        let message_context = format!("PROJECT DESCRIPTION: {:?} \n CODE_TEMPLATE: {:?} \n",
            factsheet, factsheet.backend_code,
        );

        let improved_backend_code = ai_task_request(
            print_improved_webserver_code,
            &message_context,
            &self.attributes.position,
            stringify!(print_improved_webserver_code)
        ).await;

        save_backend_code(&improved_backend_code);
        factsheet.backend_code = Some(improved_backend_code);
    }
}
