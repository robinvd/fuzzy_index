use std::error::Error;

use inverted_index::{Location, ReverseIndex};
use rustyline::{error::ReadlineError, Editor};

use crate::fuzzy_finder::{fuzzy_find, FuzzyConfig};

mod fuzzy_finder;
mod lexer;

fn main() -> Result<(), Box<dyn Error>> {
    println!("indexing");
    let mut index = ReverseIndex::default();
    let lexer = lexer::Lexer::new();

    for file in std::env::args().skip(1) {
        println!("indexing {:?}", file);
        let text = std::fs::read_to_string(&file)?;
        index.add_items(
            lexer
                .lex(&text)
                .map(|(text, pos)| (text, Location { file: &file, pos })),
        );
    }

    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let query = lexer.lex(&line).map(|(text, _)| text);
                let results = fuzzy_find(
                    &FuzzyConfig {
                        line_span: Some(0),
                        ..Default::default()
                    },
                    &mut index,
                    query,
                );

                for result in results {
                    println!("{}", result);
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
