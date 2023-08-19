// \r\n -> \n, \r -> \n, in other words replace every new line and \r with \n
// But we shall skip this. \r = new line

use crate::tokens;

const EOF: u8 = 0x05;

struct Tokenizer<'a> {
    raw_html:         &'a [u8],
    state:            State,
    return_state:     State,
    position:         usize,
    tokens:           Vec<tokens::Token>,

    temp_buffer:      Vec<u8>,
    cur_attributes:   Vec<tokens::Attribute>,
    cur_attr_start:   u32,
    cur_attr_size:    u16,
    cur_text_start:   u32,
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
            tokens: Vec::new(),
            temp_buffer: Vec::with_capacity(10),
            cur_attributes: Vec::new(),
            cur_attr_start: 0u32,
            cur_attr_size: 0u16,
            cur_text_start: 0u32,
            cur_text_size: 0u32,
            cur_flags: 0u8,
            cur_token_id: tokens::TagID::Data,
        }
    }

    fn tokenize(&mut self) {
        while self.position < self.raw_html.len() {
            self.induce_state()
        }
    }

    fn induce_state(&mut self) {
        match &self.state {
            State::Data => self.data_state(),
            State::TagOpen => self.tag_open_state(),
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
            }
            _ => {
                self.state = State::Rawtext;
            }
        }
    }

    fn tag_open_state(&mut self) {
        while self.position < self.raw_html.len() {
            match self.raw_html[self.position] {
                b'A'..=b'z' => {
                    self.state = State::TagName;
                    break;
                }
                b'!' => {
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

#[derive(Debug)]
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
