use std::error::Error;
use std::fs;

use argparse::{ArgumentParser, Store};
use pulldown_cmark::{Event, Options, Parser, Tag};

fn main() -> Result<(), Box<dyn Error>> {
    let mut filename = String::new();
    {
        let mut parser = ArgumentParser::new();
        parser.refer(&mut filename)
            .add_argument("filename", Store, "");
        parser.parse_args_or_exit();
    }

    let content = fs::read_to_string(filename)?;
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&content, options);

    for event in parser {
        if let Event::Start(Tag::Link(_, url, _)) = event {
            println!("{}", url);
        }
    }

    Ok(())
}
