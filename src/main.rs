// Import the lib that will allow us to read args passed to the program via terminal
use std::env;
// Import the lib that will allow us to exit the program
use std::process;
// Import Config from our lib
use minigrep::Config;

// NOTE: To pass arguments to the program, you must separate the cargo run call with -- and then pass the arguments
// Example: cargo run -- arg1 arg2 arg3
fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        // eprintln! prints to stderr
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // NOTE: Below is the long implementation of passing in the args to the program. 
    // Above, we pass in the env::args iterator directly to the Config::build method
    // // Use collect to turn the args iterator into a vector, allowing us to collect the args passed to the program
    // let args: Vec<String> = env::args().collect();
    // // Print the args passed to the program
    // dbg!(&args);
    // Access the args passed to the program
    // The first arg is the path to the program which is populated automatically
    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     // eprintln! prints to stderr
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    if let Err(e) = minigrep::run(config) {
        // eprintln! prints to stderr
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}


