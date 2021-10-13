use std::error::Error;

use inverted_index::ReverseIndex;
use rustyline::{error::ReadlineError, Editor};

fn main() -> Result<(), Box<dyn Error>> {
    println!("indexing");
    let mut index = ReverseIndex::default();
    let lex_regex = regex::Regex::new(r"[\w]+|\n").unwrap();

    for file in std::env::args().skip(1) {
        println!("indexing {:?}", file);
        let text = std::fs::read_to_string(&file)?;
        let mut line_number = 1;
        for word in lex_regex.find_iter(&text) {
            if word.as_str() == "\n" {
                line_number += 1;
                continue;
            }
            index.add_item(word.as_str(), &file, line_number);
        }
    }

    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let results = index.query(&line);

                println!("results:");
                for result in results {
                    println!("  {}:{}", result.file, result.offset);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
