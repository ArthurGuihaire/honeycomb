use crate::constants::{FONT_SIZES, LINE_SPACING};

pub struct StringSegment<'a> {
    italicized: bool,
    bold: bool,
    code: bool,
    offset_x: u32,
    string: &'a str,
}

impl StringSegment<'_> {
    pub fn print(self) {
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

impl FormattedLine<'_> {
    pub fn print(self) {
        println!("font_size: {}, y: {}", self.font_size, self.offset_y);
        for segment in self.segments {
            segment.print();
        }
    }
}

fn parse_line<'a>(line: &'a str, c_offset_y: &mut u32) -> FormattedLine<'a> {
    let mut fmline = FormattedLine {
        font_size: FONT_SIZES[0],
        offset_y: *c_offset_y,
        segments: Vec::new(),
    };

    *c_offset_y += FONT_SIZES[0] + LINE_SPACING;
    // Current state
    let mut c_italicized = false;
    let mut c_bold = false;
    let mut c_code = false;
    let mut start_index = 0;

    let mut iterator = line.char_indices().peekable();
    while let Some((index, character)) = iterator.next() {
        if character == '*' {
            //first push segment that ended here
            let prev = StringSegment {
                italicized: c_italicized,
                bold: c_bold,
                code: c_code,
                offset_x: index as u32,
                string: &line[start_index..index],
            };
            fmline.segments.push(prev);
            //get next char to check if *, and check if not end of line
            //also need to update start index to next non-special character
            if let Some(&(_next_idx, next_char)) = iterator.peek() {
                if next_char == '*' {
                    c_bold = !c_bold;
                    iterator.next();
                    start_index = index + 2;
                } else {
                    c_italicized = !c_italicized;
                    start_index = index + 1;
                }
            }
        }

        if character == '`' {
            let prev = StringSegment {
                italicized: c_italicized,
                bold: c_bold,
                code: c_code,
                offset_x: index as u32,
                string: &line[start_index..index],
            };
            fmline.segments.push(prev);

            c_code = !c_code;

            start_index = index + 1;
        }
    }

    fmline
}

pub fn parse_markdown(string: &str) -> Vec<FormattedLine<'_>> {
    let mut segments: Vec<FormattedLine> = Vec::new();

    let mut c_offset_y: u32 = 0;

    for line in string.lines() {
        segments.push(parse_line(&line, &mut c_offset_y));
    }

    return segments;
}
