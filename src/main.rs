use clap::Parser;

use crate::markdown_parser::parse_markdown;
mod constants;
mod markdown_parser;
mod pdf_writer;
use crate::pdf_writer::write_pdf;

//derive thing copies method implementations for Args from clap::Parser
#[derive(Parser)]
struct Args {
    mode: Option<String>,
    filename: Option<String>,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    match args.mode {
        Some(value) => println!("{}", value),
        None => println!("No value"),
    }

    let unparsed_file: String;

    let result = match args.filename {
        Some(filename) => {
            unparsed_file = std::fs::read_to_string(filename)?;
            Ok(parse_markdown(&unparsed_file))
        }
        None => Err(()),
    };

    match result {
        Err(_) => println!("Error"),
        Ok(fm_lines) => {
            for line in fm_lines {
                line.print();
            }
        }
    }

    write_pdf();

    Ok(())
}
