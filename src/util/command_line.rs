use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

use std::io::{stdin, stdout};

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
