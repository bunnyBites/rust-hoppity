mod ai_function;
mod model;
mod service;
mod util;

use model::agent_manager::agent_manager::AgentManager;
use util::command_line;

#[tokio::main]
async fn main() {
    let user_input = command_line::get_user_response("What kind of webservers you want to build?");

    let mut agent_manager = AgentManager::new(user_input.as_str())
        .await
        .expect("Failed to create Agent Manager");

    agent_manager.execute_manager().await;
}
