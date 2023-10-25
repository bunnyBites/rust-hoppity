use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    APICall,
    Issue,
    UnitTest,
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

        print!("#AGENT -> {} ", agent_position);

        // print agent's comment or statement in color based on it's action
        stdout
            .execute(SetForegroundColor(current_action_color))
            .unwrap();

        print!("Stage --> {} ", agent_statement);

        // reset the color in our terminal
        stdout.execute(ResetColor).unwrap();
        println!("");
    }
}

pub fn get_user_approval() -> bool {
    let mut stdout = stdout();

    loop {
        stdout.execute(SetForegroundColor(Color::Yellow)).unwrap();
        println!("");
        println!("AI can be mischievous sometimes, it's better to check the code. Are you want to proceed further?");

        stdout.execute(ResetColor).unwrap();

        // display choices
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] -> Good to go");

        stdout.execute(SetForegroundColor(Color::Red)).unwrap();
        println!("[2] -> Terminate the process");

        stdout.execute(SetForegroundColor(Color::DarkBlue)).unwrap();
        print!("Enter your choice - ");

        stdout.execute(ResetColor).unwrap();

        // get the choice from the user
        let mut human_choice = String::new();

        stdin()
            .read_line(&mut human_choice)
            .expect("Failed to get human choice");

        match human_choice.trim().to_lowercase().as_str() {
            "1" | "y" | "ok" => return true,
            "2" | "n" | "no" => return false,
            _ => {
                println!("The provided input is invalid!. Please provide '1' or '2'");
                continue;
            }
        }
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
    fn test_agent_progress() {
        PrintCommand::APICall.print_agent_action("Seeking", "Getting lot of traffic!!");
    }
}
