use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};

pub struct xml {
    pub writer: quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
}

impl xml {
    pub fn new() -> Self {
        Self {
            writer: quick_xml::Writer::new(std::io::Cursor::new(Vec::new())),
        }
    }

    pub fn update_declaration(&mut self) {
        self.writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), None)));
    }

    pub fn update_comment(&mut self, content: &str) {
        self.writer.write_event(Event::Comment(BytesText::from_plain_str(content)));
    }

    pub fn write_start_tag(&mut self, tag: &str) {
        self.writer.write_event(Event::Start(BytesStart::borrowed(tag.as_bytes(), tag.len())));
    }

    pub fn write_end_tag(&mut self, tag: &str) {
        self.writer.write_event(Event::End(BytesEnd::borrowed(tag.as_bytes())));
    }

    pub fn update_tag(&mut self, tag: &str, content: &str) {
        self.writer.write_event(Event::Start(BytesStart::borrowed(tag.as_bytes(), tag.len())));
        self.writer.write_event(Event::Text(BytesText::from_plain_str(content)));
        self.writer.write_event(Event::End(BytesEnd::borrowed(tag.as_bytes())));
    }

}
