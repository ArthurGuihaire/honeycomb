use crate::{
    constants::SPACING_FACTOR,
    markdown_parser::{FormattedLine, StringSegment},
};
use std::{mem, os::linux::raw::stat};
use ttf_parser::{self, Face};

use pdf_writer::{Content, Finish, Name, Pdf, Rect, Ref, Str};

const FONT_STRING_DICTIONARY: &[&str; 8] = &[
    "Helvetica",
    "Helvetica-Bold",
    "Helvetica-Oblique",
    "Helvetica-BoldOblique",
    "Courier",
    "Courier-Bold",
    "Courier-Oblique",
    "Courier-BoldOblique",
];

pub struct PdfWriter {
    pdf: Pdf,
    content: Content,
    ref_index: i32,
    //fonts: 3 bit number, (monospace, italicized, bold)
    fonts: Vec<Ref>,
    font_registry: Vec<[u8; 2]>,
}

//struct PdfFont {
//    data: Vec<u8>,
//    face: ttf_parser::Face<'static>,
//}
//
//impl PdfFont {
//    pub fn load(path: &str) -> Self {
//        let data = std::fs::read(path).unwrap();
//        let static_data: &'static [u8] = Box::leak(data.clone().into_boxed_slice());
//        let face = Face::parse(static_data, 0).unwrap();
//
//        Self { data, face }
//    }
//    pub fn line_height(self, font_size: f32) -> f32 {
//        let upm = self.face.units_per_em();
//        let ascent = self.face.ascender() as f32 / upm * font_size;
//        let descent = face.
//    }
//}

impl PdfWriter {
    pub fn new() -> Self {
        Self {
            pdf: Pdf::new(),
            content: Content::new(),
            ref_index: 1,
            fonts: Vec::with_capacity(8),
            font_registry: Vec::with_capacity(8),
        }
    }
    fn write_text(&mut self, segment: &StringSegment, font_size: f32) {
        let font_number = ((segment.code as usize) << 2)
            + ((segment.italicized as usize) << 1)
            + segment.bold as usize;

        self.content
            .set_font(Name(&self.font_registry[font_number]), font_size);
        self.content.show(Str(segment.string.as_bytes()));
    }
    fn write_line(&mut self, line: &FormattedLine) {
        let font_size = line.font_size as f32;
        //self.content.set_leading(30.0);
        for segment in &line.segments {
            self.write_text(&segment, font_size);
        }

        self.content.next_line_using_leading();
    }

    pub fn write_pdf(mut self, lines: &Vec<FormattedLine>) {
        //lazy update ref index is ok
        let catalog_id = Ref::new(self.ref_index);
        let page_tree_id = Ref::new(self.ref_index + 1);
        let page_id = Ref::new(self.ref_index + 2);
        let content_id = Ref::new(self.ref_index + 3);
        self.ref_index += 4;

        //page needed for font bindings
        //rust is *very* annoying here, we cannot create the font registry
        //because getting the page borrows pdf mutably
        for i in 0..8 {
            self.fonts.push(Ref::new(self.ref_index));
            self.ref_index += 1;
            self.pdf
                .type1_font(self.fonts[i])
                .base_font(Name(FONT_STRING_DICTIONARY[i].as_bytes()));
            //these keys do not have fonts attached to them yet
            let resource_name = [b'F', b'0' + i as u8];
            self.font_registry.push(resource_name);
        }

        self.pdf.catalog(catalog_id).pages(page_tree_id);
        self.pdf.pages(page_tree_id).kids([page_id]).count(1);

        self.content = Content::new();
        self.content.begin_text();

        //self.content.move_to(50.0, 750.0);
        //self.content.next_line(108.0, 734.0);
        self.content
            .set_text_matrix([1.0, 0.0, 0.0, 1.0, 100.0, 750.0 - 0.8 * 32.0]);

        //let mut iter = lines.iter().peekable();
        //while let Some(fm_line) = iter.next() {
        //    match iter.peek() {
        //        None => {}
        //        Some(next_line) => {
        //            self.content.set_leading(
        //                (next_line.font_size + fm_line.font_size) as f32 * SPACING_FACTOR * 0.5,
        //            );
        //        }
        //    }

        //    self.write_line(fm_line);
        //}

        self.content.end_text();

        //rust is very annoying here
        //finish() consumes content, so it requires us to consume self
        let content = mem::replace(&mut self.content, Content::new());
        self.pdf.stream(content_id, &content.finish());

        let mut page = self.pdf.page(page_id);
        page.parent(page_tree_id);
        page.media_box(Rect::new(0.0, 0.0, 595.0, 842.0));
        page.contents(content_id);

        //now that we no longer need mutable access to pdf we can build the font registry
        let mut resources = page.resources();
        let mut font_dictionary = resources.fonts();
        for i in 0..8 {
            let resource_name = [b'F', b'0' + i as u8];
            font_dictionary
                .insert(Name(&resource_name))
                .primitive(self.fonts[i]);
        }
        font_dictionary.finish();
        resources.finish();
        page.finish();

        let buf = self.pdf.finish();
        let write_result = std::fs::write("target/test.pdf", buf);
        match write_result {
            Ok(_) => {}
            Err(e) => println!("Error writing pdf file: {}", e),
        }
    }
}
