use std::io::{stdin, stdout};
use crossterm::{
    style::{Color, ResetColor, SetForegroundColor },
    ExecutableCommand,
};


#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    APICall,
    UnitTest,
    Issue,
}

// display ai function commentary
// this is let us know what is being currently done by our ai functions
impl PrintCommand {
    pub fn print_agent_action(&self, agent_position: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        let current_action_color = match (self) {
            PrintCommand::APICall => Color::DarkBlue,
            PrintCommand::UnitTest => Color::Green,
            PrintCommand::Issue => Color::Red,
        };

        // print the agent's current position in terminal
        stdout.execute(SetForegroundColor(Color::Magenta)).unwrap();

        print!("AGENT --> {} ", agent_position);

        // print agent's comment or statement in color based on it's action
        stdout.execute(SetForegroundColor(current_action_color)).unwrap();

        print!("### Cooking --> {} ", agent_statement);

        // reset the color in our terminal
        stdout.execute(ResetColor).unwrap();
        println!("");
    }
}


// get user request (question)
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // prompt the user with the provided question with custom color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    // reset the color (blue) to normal in console
    stdout.execute(ResetColor).unwrap();

    // read the user input
    let mut user_response: String = String::new();

    stdin()
        .read_line(&mut user_response)
        .expect("Failed to get input from user");

    user_response.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_commentatory() {
        PrintCommand::APICall.print_agent_action(
            "Seeking",
            "Getting lot of traffic!!",
        );
    }
}