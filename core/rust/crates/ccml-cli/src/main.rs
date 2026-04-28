use ccml_core::{to_json, ToJsonOptions};
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("failed to read stdin: {e}");
        std::process::exit(2);
    }

    match to_json(&input, &ToJsonOptions::default()) {
        Ok(json) => println!("{json}"),
        Err(err) => {
            for d in err.diagnostics {
                eprintln!("{}:{} {} {}", d.line, d.column, d.code, d.message);
            }
            std::process::exit(1);
        }
    }
}
