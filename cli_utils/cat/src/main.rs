use std::env;
use std::fs;
use std::process;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if there are any arguments (excluding the program name)
    if args.len() == 1 {
        println!("Usage: {} [file_names]", args[0]);
        process::exit(1);
    }

    // Iterate over the arguments (files) and print their contents
    for arg in args.iter().skip(1) {
        let file_contents: String = read_file(arg);
        print!("{}", file_contents);
    }

}

fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("{}: {}",filename, error);
            String::new()
        }
    }
}
