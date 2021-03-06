use std::io;
use std::io::Write;
use std::env;
use std::fs;

mod parser;


fn run_file(file_path: &String) {
    let code = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    println!("{}", &code);
}


fn run_promt() {
    loop {
        let mut line = String::new();

        print!("> ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).expect("Failed to read the line");
        io::stdout().flush().unwrap();

        let line = line.trim().to_string();
        let line_chars: Vec<char> = line.chars().collect();

        let mut scanner = parser::tokens::Scanner {
            source: line,
            source_chars: line_chars,
            ..Default::default()
        };

        scanner.scan_tokens();
        println!("{:?}", scanner.token_list);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let len_args: usize = args.len();

    if len_args > 2 {
        println!("Usage: pix [file]");
    } else if len_args == 2 {
        run_file(&args[1]);
    } else if len_args == 1 {
        run_promt();
    }
}
