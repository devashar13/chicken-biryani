use std::env;

// Import all the commands
mod commands;

fn main() {
    // Get the args
    let args: Vec<String> = env::args().collect();

    // Perform the following operations
    // 1. `ngrok path`: Set ngrok location in config file at ~/.chicken-biryani/config.toml
    // 2. `host filepath port`: Start a server on optionally specified port with the file at filepath hosted. If ngrok path is set,
    //    http tunnel the opened port using ngrok. Otherwise, if the machine has a static ip, run
    //    on that.

    // println!("{:?}", args);
    if args.len() < 2 { return println!("Commands: \nset-ngrok PATH, \nhost FILEPATH PORT"); }

    // Get the command
    let command = &args[1];

    // Run the command module for the specific command
    match command.as_str() {
        "set-ngrok" => commands::ngrok::set_ngrok(),
        "host" => commands::host::host(),
        _ => println!("Invalid command!"),
    };
}
