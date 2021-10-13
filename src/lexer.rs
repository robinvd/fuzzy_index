use inverted_index::Pos;

pub struct Lexer {
    regex: regex::Regex,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            regex: regex::Regex::new(r"[\w]+|\n").unwrap(),
        }
    }

    pub fn lex<'a>(&'a self, text: &'a str) -> impl Iterator<Item = (&'a str, Pos)> {
        let mut line_number = 1;
        let mut line_offset = 0;
        self.regex.find_iter(&text).filter_map(move |item| {
            if item.as_str() == "\n" {
                line_number += 1;
                line_offset = item.start() + 1;
                return None;
            }

            Some((
                item.as_str(),
                Pos {
                    offset: item.start(),
                    line: line_number,
                    column: item.start() - line_offset + 1,
                },
            ))
        })
    }
}
