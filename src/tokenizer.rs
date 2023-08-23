// \r\n -> \n, \r -> \n, in other words replace every new line and \r with \n
// But we shall skip this. \r = new line

use crate::tokens;
use tokens::ASCII_TO_TAG_ID;

const EOF: u8 = 0x05;
const FF: u8 = 0x0C; // FF - form feed character (normally '\f')

pub struct Tokenizer<'a> {
    raw_html:         &'a [u8],
    state:            State,
    return_state:     State,
    position:         usize,
    token_start:      usize,
    pub tokens:           Vec<tokens::Token>,

    temp_buffer:      Vec<u8>,
    cur_attributes:   Vec<tokens::Attribute>,
    cur_start:        u32,
    cur_end:          u32,
    cur_text_off:     u16,
    cur_text_size:    u32,
    cur_flags:        u8,
    cur_token_id:     tokens::TagID,
}

impl<'a> Tokenizer<'a> {
    pub fn new(raw_html: &'a str) -> Self {
        Tokenizer {
            raw_html: raw_html.as_bytes(),
            state: State::Data,
            return_state: State::Data,
            position: 0usize,
            token_start: 0usize,
            tokens: Vec::new(),
            temp_buffer: Vec::with_capacity(10),
            cur_attributes: Vec::new(),
            cur_start: 0u32,
            cur_end: 0u32,
            cur_text_off: 0u16,
            cur_text_size: 0u32,
            cur_flags: 0u8,
            cur_token_id: tokens::TagID::Data,
        }
    }

    fn find_tag_id(&self, start: usize, end: usize) -> tokens::TagID {
        let slice = unsafe { std::slice::from_raw_parts(self.raw_html.as_ptr().add(start), end - start) };
        match ASCII_TO_TAG_ID.get(slice) {
            Some(&tag_id) => tag_id,
            None => tokens::TagID::A, // FIXME: change this, we need to insert into a dynamic hash
                                      // table since we can have user-defined tags
        }
    }

    fn create_empty_attribute(&mut self) {
        let attr_start = (self.position as u32 - self.cur_start) as u16;
        self.cur_attributes.push(tokens::Attribute::new(attr_start, 0, 0, 0));
    }

    fn emit_current_token_as_text(&mut self) {
        if self.cur_end > self.cur_start {
            self.tokens.push(tokens::Token::new_no_attributes(self.cur_start, 
                                                              self.cur_end, 
                                                              self.cur_text_off,
                                                              self.cur_text_size, 
                                                              self.cur_token_id,
                                                              0));
            self.clear_current_token();
        }
    }

    fn emit_current_token_no_text(&mut self, flags: u8) {
        self.cur_flags = flags;
        self.cur_text_off = 0;
        self.cur_text_size = 0;
        self.cur_token_id = self.find_tag_id(self.cur_start as usize, 
                                             self.cur_end as usize);

        if self.cur_attributes.is_empty() {
            self.tokens.push(tokens::Token::new_no_attributes(self.cur_start, 
                                                              self.cur_end, 
                                                              self.cur_text_off,
                                                              self.cur_text_size, 
                                                              self.cur_token_id,
                                                              self.cur_flags));
        } else {
            let attributes = std::mem::replace(&mut self.cur_attributes, Vec::new());
            self.tokens.push(tokens::Token::new(attributes.into_boxed_slice(),
                                                self.cur_start, 
                                                self.cur_end, 
                                                self.cur_text_off,
                                                self.cur_text_size, 
                                                self.cur_token_id,
                                                self.cur_flags))

        }

        self.clear_current_token();
    }

    fn clear_current_token(&mut self) {
        self.cur_end = 0;
        self.cur_start = 0;
        self.cur_flags = 0;
        self.cur_text_off = 0;
        self.cur_text_size = 0;
        self.cur_attributes.clear();
    }

    pub fn tokenize(&mut self) {
        while self.position < self.raw_html.len() {
            self.induce_state();
            println!("New state: {:?}", self.state);
        }
    }

    fn induce_state(&mut self) {
        match &self.state {
            State::Data => self.data_state(),
            State::TagOpen => self.tag_open_state(),
            State::TagName => self.tag_name_state(),
            State::BeforeAttributeName => self.before_attribute_name_state(),
            State::AttributeName => self.attribute_name_state(),
            State::AfterAttributeName => self.after_attribute_name_state(),
            State::CharacterReference => self.character_reference_state(),
            _ => unimplemented!("Reached unimplemented state {:?}", self.state),
        }
    }

    fn data_state(&mut self) {
        while self.position < self.raw_html.len() {
            match self.raw_html[self.position] {
                b'&' => {
                    self.return_state = State::Data;
                    self.state = State::CharacterReference;
                    self.position += 1;
                    break;
                },
                b'<' => {
                    self.state = State::TagOpen;
                    self.position += 1;
                    break;
                },
                b'\0' => {
                    // Parse error
                },
                EOF => {
                    // Parse error
                },
                _ => {},
            }
        }
    }

    fn tag_open_state(&mut self) {
        while self.position < self.raw_html.len() {
            match self.raw_html[self.position] {
                b'A'..=b'z' => {
                    self.emit_current_token_as_text();
                    self.cur_start = self.position as u32;
                    self.state = State::TagName;
                    break;
                }
                b'!' => {
                    self.emit_current_token_as_text();
                    self.state = State::MarkupDeclarationOpen;
                    self.position += 1;
                    break;
                }
                b'/' => {
                    self.state = State::EndTagOpen;
                    self.position += 1;
                    break;
                }
                b'?' => {
                    // Parse error
                    self.state = State::BogusComment;
                    break;
                }
                EOF => {
                    // TODO: handle unexpected EOF
                }
                _ => {
                    // Parse error
                }
            }
        }
    }

    fn tag_name_state(&mut self) {
        while self.position < self.raw_html.len() {
            match self.raw_html[self.position] {
                b'\t' | b'\n' | b' ' | FF => {
                    self.state = State::BeforeAttributeName;
                    self.cur_end = self.position as u32;
                    self.position += 1;
                    break
                }
                b'/' => {
                    self.state = State::SelfClosingStartTag;
                    self.cur_end = self.position as u32;
                    self.position += 1;
                    break
                }
                b'>' => {
                    self.cur_end = self.position as u32;
                    self.emit_current_token_no_text(0);
                    self.state = State::Data;
                    self.position += 1;
                    break
                }
                b'\0' => {
                    // TODO: handle unexpected null
                }
                EOF => {
                    // TODO: handle unexpected EOF
                }
                _ => {
                    self.position += 1;
                }
            }
        }
    }

    fn before_attribute_name_state(&mut self) {
        while self.position < self.raw_html.len() {
            match self.raw_html[self.position] {
                b'\t' | b'\n' | FF | b' ' => {
                    self.position += 1
                }
                b'/' | b'>' | EOF => {
                    self.state = State::AfterAttributeName;
                    break
                }
                b'=' => {
                    // TODO: handle unique error
                }
                _ => {
                    self.state = State::AttributeName;
                    self.create_empty_attribute();
                    break
                }
            }
        }
    }

    fn attribute_name_state(&mut self) {
        // TODO: compare with previous attributes for duplicates
        while self.position < self.raw_html.len() {
            match self.raw_html[self.position] {
                b'\t' | b'\n' | FF | b' ' | b'/' | b'>' | EOF => {
                    self.state = State::AfterAttributeName;
                    break
                }
                b'=' => {
                    self.state = State::BeforeAttributeValue;
                    self.position += 1;
                    break
                }
                b'\0' => {
                    // TODO: handle unique error
                }
                b'"' | b'\'' | b'<' => {
                    // TODO: handle unique error
                }
                _ => {
                    let cur_attribute = self.cur_attributes.last_mut().unwrap();
                    cur_attribute.name_size += 1;
                    self.position += 1
                }
            }
        }
    }

    fn after_attribute_name_state(&mut self) {
        while self.position < self.raw_html.len() {
            match self.raw_html[self.position] {
                b'\t' | b'\n' | FF | b' ' => {
                    self.position += 1
                }
                b'/' => {
                    self.state = State::SelfClosingStartTag;
                    self.position += 1;
                    break
                }
                b'=' => {
                    self.state = State::BeforeAttributeValue;
                    self.position += 1;
                    break
                }
                b'>' => {
                    self.state = State::Data;
                    self.position += 1;
                    self.emit_current_token_no_text(0);
                    break
                } 
                EOF => {
                    // TODO: handle unique error
                }
                _ => {
                    self.state = State::BeforeAttributeName;
                    self.create_empty_attribute();
                    break
                }
            }
        }
    }

    fn before_attribute_value_state(&mut self) {
        let attr = self.cur_attributes.last_mut().unwrap();
        while self.position < self.raw_html.len() {
            match self.raw_html[self.position] {
                b'\t' | b'\n' | FF | b' ' => {
                    self.position += 1
                }
                b'"' => {
                    self.state = State::AttributeValueDoubleQuoted;
                    self.position += 1;
                    attr.value_begin = self.position as u16;
                    break
                }
                b'\'' => {
                    self.state = State::AttributeValueSingleQuoted;
                    self.position += 1;
                    attr.value_begin = self.position as u16;
                    break
                }
                b'>' => {
                    // TODO: Handle unique error state
                }
                _ => {
                    self.state = State::AttributeValueUnquoted;
                    attr.value_begin = self.position as u16;
                    break
                }
            }
        }
    }

    fn attribute_value_double_quoted_state(&mut self) {
        let attribute = self.cur_attributes.last_mut().unwrap();
        while self.position < self.raw_html.len() {
            match self.raw_html[self.position] {
                b'"' => {
                    self.state = State::AfterAttributeValueQuoted;
                    self.position += 1;
                    break
                }
                b'&' => {
                    self.return_state = self.state;
                    self.state = State::CharacterReference;
                    self.position += 1;
                    break
                }
                b'\0' => {
                    // TODO: handle unique error
                }
                EOF => {
                    // TODO: handle unique error
                }
                _ => {
                    attribute.value_size += 1;
                    self.position += 1
                }
            }
        }
    }

    fn attribute_value_single_quoted_state(&mut self) {
        let attribute = self.cur_attributes.last_mut().unwrap();
        while self.position < self.raw_html.len() {
            match self.raw_html[self.position] {
                b'\'' => {
                    self.state = State::AfterAttributeValueQuoted;
                    self.position += 1;
                    break
                }
                b'&' => {
                    self.return_state = self.state;
                    self.state = State::CharacterReference;
                    self.position += 1;
                    break
                }
                b'\0' => {
                    // TODO: Handle unique error
                }
                EOF => {
                    // TODO: Handle unique error
                }
                _ => {
                    attribute.value_size += 1;
                    self.position += 1
                }
            }
        }
    }

    fn attribute_value_unquoted_state(&mut self) {
        let attribute = self.cur_attributes.last_mut().unwrap();
        while self.position < self.raw_html.len() {
            match self.raw_html[self.position] {
                b'\t' | b'\n' | FF | b' ' => {
                    self.state = State::BeforeAttributeName;
                    self.position += 1;
                    break
                }
                b'&' => {
                    self.return_state = self.state;
                    self.state = State::CharacterReference;
                    self.position += 1;
                    break
                }
                b'>' => {
                    self.state = State::Data;
                    self.emit_current_token_no_text(0);
                    self.position += 1;
                    break
                }
                b'\0' => {
                    // TODO: handle unique error
                }
                b'"' | b'\'' | b'<' | b'=' | b'`' => {
                    // TODO: handle unique error
                    attribute.value_size += 1;
                }
                _ => {
                    attribute.value_size += 1;
                    self.position += 1
                }
            }
        }
    }

    fn after_attribute_value_quoted_state(&mut self) {
        while self.position < self.raw_html.len() {
            match self.raw_html[self.position] {
                b'\t' | b'\n' | FF | b' ' => {
                    self.state = State::BeforeAttributeName;
                    self.position += 1;
                    break
                }
                b'/' => {
                    self.state = State::SelfClosingStartTag;
                    self.position += 1;
                    break
                }
                b'>' => {
                    self.state = State::Data;
                    self.emit_current_token_no_text(0);
                    self.position += 1;
                    break
                }
                EOF => {
                    // TODO: handle unique error
                }
                _ => {
                    // TODO: handle unique error
                }
            }
        }
    }

    fn self_closing_start_tag_state(&mut self) {
        while self.position < self.raw_html.len() {
            match self.raw_html[self.position] {
                b'>' => {
                    self.state = State::Data;
                    self.emit_current_token_no_text(tokens::Flags::CloseSelf as u8);
                    self.position += 1;
                    break
                }
                EOF => {
                    // TODO: Handle error
                }
                _ => {
                    // TODO: Handle error
                }
            }
        }
    }


    fn rcdata_state(&mut self) {
        while self.position < self.raw_html.len() {
            match self.raw_html[self.position] {
                b'&' => {
                    self.return_state = State::Rcdata;
                    self.state = State::CharacterReference;
                    self.position += 1;
                    break
                }
                b'<' => {
                    self.state = State::RcdataLessThanSign;
                    self.position += 1;
                    break
                }
                b'\r' => {
                    // Need to handle carriage return(?)
                }
                b'\0' => {
                    // Parse Error
                }
                EOF => {
                    // EOF, emit end of file
                }
                _ => {},
            }
        }
    }

    fn rawtext_state(&mut self) {
        while self.position < self.raw_html.len() {
            match self.raw_html[self.position] {
                b'<' => {
                    self.state = State::RawtextLessThanSign;
                    self.position += 1;
                }
                b'\0' => {
                    // Parse Error
                }
                EOF => {
                    // EOF, emit end of file
                }
                _ => {}
            }
        }
    }

    fn rawtext_less_than_sign_state(&mut self) {
        match self.raw_html[self.position] {
            b'/' => {
                self.temp_buffer.clear();
                self.state = State::RawtextEndTagOpen;
                self.position += 1
            }
            _ => {
                self.state = State::Rawtext;
            }
        }
    }

    fn rawtext_end_tag_open_state(&mut self) {
        match self.raw_html[self.position] {
            b'A'..=b'z' => {
                self.state = State::RawtextEndTagName
            }
            _ => {
                self.state = State::Rawtext
            }
        }
    }

    fn rawtext_end_tag_name_state(&mut self) {
        // TODO
    }


    fn character_reference_state(&mut self) {
        // TODO: make this work correctly
        self.temp_buffer.clear();
        while self.position < self.raw_html.len() {
            match self.raw_html[self.position] {
                b'A'..=b'z' => {
                    // Apparently the slowest part of HTML parsing in the named char reference
                    // state
                    self.state = State::NamedCharacterReference
                }
                b'#' => {
                    self.temp_buffer.push(b'#');
                    self.state = State::NumericCharacterReference
                }
                _ => {
                    // Flush temp buffer as part of code point or otherwise as part of attribute
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum State {
    Data, 
    Rcdata,
    Rawtext,
    ScriptData,
    Plaintext,
    TagOpen,
    EndTagOpen, 
    TagName,
    RcdataLessThanSign,
    RcdataEndTagOpen,
    RcdataEndTagName,
    RawtextLessThanSign,
    RawtextEndTagOpen,
    RawtextEndTagName,
    ScriptDataLessThanSign,
    ScriptDataEndTagOpen,
    ScriptDataEndTagName,
    ScriptDataEscapeStart,
    ScriptDataEscapeStartDash,
    ScriptDataEscaped,
    ScriptDataEscapedDash,
    ScriptDataEscapedDashDash,
    ScriptDataEscapedLessThanSign,
    ScriptDataEscapedEndTagOpen,
    ScriptDataEscapedEndTagName,
    ScriptDataDoubleEscapeStart,
    ScriptDataDoubleEscaped,
    ScriptDataDoubleEscapedDash,
    ScriptDataDoubleEscapedDashDash,
    ScriptDataDoubleEscapedLessThanSign,
    ScriptDataDoubleEscapeEnd, 
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnquoted,
    AfterAttributeValueQuoted,
    SelfClosingStartTag,
    BogusComment,
    MarkupDeclarationOpen,
    CommentStart,
    CommentStartDash,
    Comment,
    CommentLessThanSign,
    CommentLessThanSignBang,
    CommentLessThanSignBangDash,
    CommentLessThanSignBangDashDash,
    CommentEndDash,
    CommentEnd,
    CommentEndBang,
    Doctype,
    BeforeDoctypeName,
    DoctypeName,
    AfterDoctypeName,
    AfterDoctypePublicKeyword,
    BeforeDoctypePublicIdentifier,
    DoctypePublicIdentifierDoubleQuoted,
    DoctypePublicIdentifierSingleQuoted,
    AfterDoctypePublicIdentifier,
    BetweenDoctypePublicAndSystemIdentifiers,
    AfterDoctypeSystemKeyword,
    BeforeDoctypeSystemIdentifier,
    DoctypeSystemIdentifierDoubleQuoted,
    DoctypeSystemIdentifierSingleQuoted,
    AfterDoctypeSystemIdentifier,
    BogusDoctype,
    CdataSection,
    CdataSectionBracket,
    CdataSectionEnd,
    CharacterReference,
    NamedCharacterReference,
    AmbiguousAmpersand,
    NumericCharacterReference,
    HexadecimalCharacterReferenceStart,
    DecimalCharacterReferenceStart,
    HexadecimalCharacterReference,
    DecimalCharacterReference,
    NumericCharacterReferenceEnd,
}
