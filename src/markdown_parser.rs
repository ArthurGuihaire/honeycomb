use crate::constants::{FONT_SIZES, SPACING_FACTOR};
use std::{iter::Peekable, num, str::CharIndices};

pub struct StringSegment<'a> {
    pub italicized: bool,
    pub bold: bool,
    pub code: bool,
    pub offset_x: u32,
    pub string: &'a str,
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
    pub font_size: u32,
    pub offset_y: u32,
    pub segments: Vec<StringSegment<'a>>,
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
        if self.start_index < index {
            let prev = StringSegment {
                italicized: self.c_italicized,
                bold: self.c_bold,
                code: self.c_code,
                offset_x: index as u32,
                string: &line[self.start_index..index],
            };
            self.fmlines.last_mut().unwrap().segments.push(prev);
            println!(
                "Pushed segment start_index {}, end_index {}, string {}",
                self.start_index,
                index,
                &line[self.start_index..index]
            );
        }
    }

    fn handle_asterix(
        &mut self,
        index: usize,
        line: &'a str,
        iterator: &mut Peekable<CharIndices>,
    ) {
        println!("Detected * at index {}", index);
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

    fn handle_hashtag(&mut self, iterator: &mut Peekable<CharIndices>) -> u32 {
        //no need to push segment, hashtag is at the start of line
        let mut num_hashtags = 0;
        //for loops in rust: a to b-1, so this is 0 to 5 (6 iterations)
        for _ in 0..6 {
            if let Some(&(_next_idx, next_char)) = iterator.peek() {
                if next_char == '#' {
                    num_hashtags += 1;
                    iterator.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        self.start_index = num_hashtags;
        return FONT_SIZES[num_hashtags];
    }

    pub fn parse_line(&mut self, line: &'a str, c_offset_y: &mut u32) {
        // Current state
        self.c_italicized = false;
        self.c_bold = false;
        self.c_code = false;
        self.start_index = 0;

        //Handle font size with # only at start of line
        let mut iterator = line.char_indices().peekable();
        let font_size = self.handle_hashtag(&mut iterator);
        self.fmlines.push(FormattedLine {
            font_size: font_size,
            offset_y: *c_offset_y,
            segments: Vec::new(),
        });

        *c_offset_y += (FONT_SIZES[0] as f32 * SPACING_FACTOR) as u32;

        let mut previous_char = ' ';

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

        self.push_segment(self.end_index, line);
    }

    //pub fn print(&self) {
    //    for line in &self.fmlines {
    //        line.print();
    //    }
    //}
}

pub fn parse_markdown(string: &str) -> Vec<FormattedLine<'_>> {
    let mut c_offset_y: u32 = 0;

    let mut parser: LineParser = LineParser::new();
    for line in string.lines() {
        parser.parse_line(&line, &mut c_offset_y);
    }

    return parser.fmlines;
}
