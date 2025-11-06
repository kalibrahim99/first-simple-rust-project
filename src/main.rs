use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <file> <word>", args.get(0).map(|s| s.as_str()).unwrap_or("program"));
        process::exit(1);
    }

    let filename = &args[1];
    let word = &args[2];

    match first_simple_rust_project::find_lines(filename, word) {
        Ok(lines) => {
            if lines.is_empty() {
                println!("No lines found containing '{}' in {}", word, filename);
                process::exit(0);
            }

            println!("Found {} matching lines in {}:", lines.len(), filename);
            for line in lines {
                println!("{}", line);
            }
        }
        Err(err) => {
            eprintln!("Error reading {}: {}", filename, err);
            process::exit(2);
        }
    }
}