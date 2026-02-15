use crate::markdown_parser::FormattedLine;

use pdf_writer::{writers::Form, Content, Finish, Name, Pdf, Rect, Ref, Str};

pub fn write_pdf(lines: &Vec<FormattedLine>) {
    let mut pdf = Pdf::new();

    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let page_id = Ref::new(3);
    let content_id = Ref::new(4);

    //Fonts
    let font_regular_id = Ref::new(5);

    pdf.catalog(catalog_id).pages(page_tree_id);
    pdf.pages(page_tree_id).kids([page_id]).count(1);

    pdf.type1_font(font_regular_id)
        .base_font(Name(b"Helvetica"));

    let mut content = Content::new();
    content.begin_text();

    content.move_to(50.0, 750.0);
    content.set_font(Name(b"F1"), 12.0);
    content.next_line(108.0, 734.0);
    content.show(Str(b"testing text please work"));

    content.end_text();
    pdf.stream(content_id, &content.finish());

    let mut page = pdf.page(page_id);
    page.parent(page_tree_id);
    page.media_box(Rect::new(0.0, 0.0, 595.0, 842.0));
    page.contents(content_id);

    let mut resources = page.resources();
    let mut font_dict = resources.fonts();
    font_dict.insert(Name(b"F1")).primitive(font_regular_id);
    font_dict.finish();
    resources.finish();
    page.finish();

    let buf = pdf.finish();
    std::fs::write("target/test.pdf", buf);
}
