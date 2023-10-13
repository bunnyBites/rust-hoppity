mod ai_function;
mod model;
mod service;
mod util;

use util::command_line;

fn main() {
    let user_input =
        command_line::get_user_response("What kind of webservers you want to build?");

    dbg!(user_input);
}
