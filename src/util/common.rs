use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json;
use crate::{model::common::large_language_model::Message, service::call_open_api::call_gpt};
use super::command_line::PrintCommand;


// Extend our ai function to print in specific type (Message)
pub fn extend_ai_func(ai_func: fn(&str) -> &str, fn_arg: &str) -> Message {
    let ai_func_str = ai_func(fn_arg);

    let msg: String = format!("FUNCTION {}
    INSTRUCTION: You are a function printer. You ONLY print the result of functions.
    Nothing else. No commentary. Here is the input of the function: {}.
    Print out what the function will return.
    ", ai_func_str, fn_arg);

    Message {
        role: "system".to_string(),
        content: msg,
    }
}

pub async fn ai_task_request(
    ai_func: for<'a> fn(&'a str) -> &'static str,
    msg_context: &str,
    agent_pos: &str,
    agent_operation: &str,
) -> String {
    // extend ai function to get Message
    let extended_message = extend_ai_func(ai_func, msg_context);

    // print ai function progress
    PrintCommand::APICall.print_agent_action(agent_pos, agent_operation);

    // call large language model
    match call_gpt(vec![extended_message.clone()]).await {
        Ok(response) => response,
        Err(_) => call_gpt(vec![extended_message])
            .await
            .expect("Fetching from OpenAI Failed"),
    }
}

pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    ai_func: for<'a> fn(&'a str) -> &'static str,
    msg_context: &str,
    agent_pos: &str,
    agent_operation: &str,
) -> T {
    let ai_task_request = ai_task_request(
        ai_func,
        msg_context,
        agent_pos,
        agent_operation
    ).await;

    let decoded_result = serde_json::from_str(&ai_task_request)
        .expect("Failed to decode response");

    decoded_result
}

// check if the provided url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::ai_function::{aifunc_architect::print_project_scope, aifunc_managing::convert_user_input_to_goal};

    #[test]
    fn test_extend_ai_func() {
        let extended_msg = extend_ai_func(print_project_scope, "dummy thing!!");

        dbg!(&extended_msg);

        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    async fn test_call_ai_func() {
        let msg_context = "build me a webserver to get anime characters";

        let ai_task_request = ai_task_request(
            convert_user_input_to_goal,
            msg_context,
            "Managing",
            "Learning Rasengan"
        ).await;

        dbg!(ai_task_request);
    }
}