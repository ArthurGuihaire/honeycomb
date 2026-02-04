use crate::constants::{FONT_SIZES, LINE_SPACING};
use std::{iter::Peekable, str::CharIndices};

pub struct StringSegment<'a> {
    italicized: bool,
    bold: bool,
    code: bool,
    offset_x: u32,
    string: &'a str,
}

impl StringSegment<'_> {
    pub fn print(&self) {
        print!(
            "i: {}, b: {}, c: {}, \"{}\"",
            self.italicized, self.bold, self.code, self.string
        );
    }
}

pub struct FormattedLine<'a> {
    font_size: u32,
    offset_y: u32,
    segments: Vec<StringSegment<'a>>,
}

impl<'a> FormattedLine<'a> {
    pub fn print(&self) {
        print!("\nfont_size: {}, y: {}, ", self.font_size, self.offset_y);
        for segment in &self.segments {
            segment.print();
        }
    }
}

pub struct LineParser<'a> {
    //current state
    c_italicized: bool,
    c_bold: bool,
    c_code: bool,
    start_index: usize,
    end_index: usize,
    fmlines: Vec<FormattedLine<'a>>,
}

impl<'a> LineParser<'a> {
    pub fn new() -> Self {
        Self {
            c_italicized: false,
            c_bold: false,
            c_code: false,
            start_index: 0,
            end_index: 0,
            fmlines: Vec::new(),
        }
    }
    fn push_segment(&mut self, index: usize, line: &'a str) {
        if self.start_index < self.end_index {
            let prev = StringSegment {
                italicized: self.c_italicized,
                bold: self.c_bold,
                code: self.c_code,
                offset_x: index as u32,
                string: &line[self.start_index..index],
            };
            self.fmlines.last_mut().unwrap().segments.push(prev);
            println!(
                "Pushed segment start_index {}, end_index {}",
                self.start_index, self.end_index
            );
        }
    }

    fn handle_asterix(
        &mut self,
        index: usize,
        line: &'a str,
        iterator: &mut Peekable<CharIndices>,
    ) {
        println!("Detected *");
        //first push segment that ended here (unless empty segment)
        self.push_segment(index, line);
        //get next char to check if *, and check if not end of line
        //also need to update start index to next non-special character
        if let Some(&(_next_idx, next_char)) = iterator.peek() {
            if next_char == '*' {
                self.c_bold = !self.c_bold;
                iterator.next();
                self.start_index = index + 2;
            } else {
                self.c_italicized = !self.c_italicized;
                self.start_index = index + 1;
            }
        } else {
            //no more characters in file, set index so last segment isn't added twice
            self.start_index = index + 1;
        }
    }

    pub fn parse_line(&mut self, line: &'a str, c_offset_y: &mut u32) {
        self.fmlines.push(FormattedLine {
            font_size: FONT_SIZES[0],
            offset_y: *c_offset_y,
            segments: Vec::new(),
        });

        *c_offset_y += FONT_SIZES[0] + LINE_SPACING;
        // Current state
        self.c_italicized = false;
        self.c_bold = false;
        self.c_code = false;
        self.start_index = 0;

        let mut previous_char = ' ';

        let mut iterator = line.char_indices().peekable();
        while let Some((index, character)) = iterator.next() {
            //If backslash, update needed variables, then continue (ignore all logic)
            if previous_char == '\\' {
                self.end_index = index;
                previous_char = character;
                println!("skipped character {}", character);
                continue;
            }
            if character == '*' {
                self.handle_asterix(index, line, &mut iterator);
            }

            if character == '`' {
                self.push_segment(index, line);

                self.c_code = !self.c_code;

                self.start_index = index + 1;
            }

            self.end_index = index;
            previous_char = character;
        }

        //add character we ended on
        self.end_index += 1;

        self.push_segment(self.start_index, line);
    }

    //pub fn print(&self) {
    //    for line in &self.fmlines {
    //        line.print();
    //    }
    //}
}

pub fn parse_markdown(string: &str) -> Vec<FormattedLine<'_>> {
    let mut c_offset_y: u32 = 0;

    let mut parse: LineParser = LineParser::new();
    for line in string.lines() {
        parse.parse_line(&line, &mut c_offset_y);
    }

    return parse.fmlines;
}
