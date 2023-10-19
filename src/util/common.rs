use crate::model::common::large_language_model::Message;


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

#[cfg(test)]

mod tests {
    use super::*;
    use crate::ai_function::aifunc_architect::print_project_scope;

    #[test]
    fn test_extend_ai_func() {
        let extended_msg = extend_ai_func(print_project_scope, "dummy thing!!");

        dbg!(&extended_msg);

        assert_eq!(extended_msg.role, "system".to_string());
    }
}