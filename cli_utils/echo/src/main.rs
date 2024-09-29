use std::env;
use std::process;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if there are any arguments (excluding the program name)
    if args.len() == 1 {
        println!("Usage: {} [arguments...]", args[0]);
        process::exit(1);
    }

    // Iterate over the arguments and print them
    for arg in args.iter().skip(1) {
        print!("{} ", arg);
    }

    println!();
}