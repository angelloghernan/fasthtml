
pub struct Attribute {
    name_begin: u16,
    value_begin: u16,
    name_size: u16,
    value_size: u16,
    // Maybe add error type as in lexbor?
}

pub struct Token {
    pub attributes: Box<[Attribute]>,
    pub attr_start:   u32,
    pub attr_size:    u16,
    pub text_start:   u32,
    pub text_size:    u32,
    pub flags:        u8,
    pub token_id:     TagID,
}

impl Token {
    pub fn new(attributes: Box<[Attribute]>, 
               attr_start: u32, attr_size: u16, text_start: u32, text_size: u32,
               token_id: TagID, flags: u8) -> Self {
        Token {
            attributes,
            attr_start,
            attr_size,
            text_start,
            text_size,
            token_id,
            flags,
        }
    }
}

#[repr(u8)]
pub enum Flags {
    Open        = 0x00,
    Close       = 0x01,
    CloseSelf   = 0x02,
    ForceQuirks = 0x04,
    Done        = 0x08,
}

#[repr(u8)]
pub enum TagID {
    Undef               = 0x00,
    EndOfFile           = 0x01,
    Text                = 0x02,
    Document            = 0x03,
    EmComment           = 0x04,
    EmDoctype           = 0x05,
    A                   = 0x06,
    Abbr                = 0x07,
    Acronym             = 0x08,
    Address             = 0x09,
    AltGlyph            = 0x0a,
    AltGlyphDef         = 0x0b,
    AltGlyphItem        = 0x0c,
    AnimateColor        = 0x0d,
    AnimateMotion       = 0x0e,
    AnimateTransform    = 0x0f,
    AnnotationXml       = 0x10,
    Applet              = 0x11,
    Area                = 0x12,
    Article             = 0x13,
    Aside               = 0x14,
    Audio               = 0x15,
    B                   = 0x16,
    Base                = 0x17,
    BaseFont            = 0x18,
    Bdi                 = 0x19,
    Bdo                 = 0x1a,
    BGSound             = 0x1b,
    Big                 = 0x1c,
    Blink               = 0x1d,
    Blockquote          = 0x1e,
    Body                = 0x1f,
    Br                  = 0x20,
    Button              = 0x21,
    Canvas              = 0x22,
    Caption             = 0x23,
    Center              = 0x24,
    Cite                = 0x25,
    ClipPath            = 0x26,
    Code                = 0x27,
    Col                 = 0x28,
    ColGroup            = 0x29,
    Data                = 0x2a,
    DataList            = 0x2b,
    DD                  = 0x2c,
    Del                 = 0x2d,
    Desc                = 0x2e,
    Details             = 0x2f,
    Dfn                 = 0x30,
    Dialog              = 0x31,
    Dir                 = 0x32,
    Div                 = 0x33,
    Dl                  = 0x34,
    Dt                  = 0x35,
    Em                  = 0x36,
    Embed               = 0x37,
    Feblend             = 0x38,
    FeColorMatrix       = 0x39,
    FeComponentTransfer = 0x3a,
    FeComposite         = 0x3b,
    FeConvolveMatrix    = 0x3c,
    FeDiffuseLighting   = 0x3d,
    FeDisplacementMap   = 0x3e,
    FeDistantLight      = 0x3f,
    FeDropShadow        = 0x40,
    FeFlood             = 0x41,
    FeFuncA             = 0x42,
    FefuncB             = 0x43,
    FeFuncG             = 0x44,
    FeFuncR             = 0x45,
    FeGaussianBlur      = 0x46,
    FeImage             = 0x47,
    FeMerge             = 0x48,
    FeMergeNode         = 0x49,
    FeMorphology        = 0x4a,
    FeOffset            = 0x4b,
    FePointLight        = 0x4c,
    FeSpecularLighting  = 0x4d,
    FeSpotlight         = 0x4e,
    FeTile              = 0x4f,
    FeTurbulence        = 0x50,
    FieldSet            = 0x51,
    FigCaption          = 0x52,
    Figure              = 0x53,
    Font                = 0x54,
    Footer              = 0x55,
    ForeignObject       = 0x56,
    Form                = 0x57,
    Frame               = 0x58,
    Frameset            = 0x59,
    Glyphref            = 0x5a,
    H1                  = 0x5b,
    H2                  = 0x5c,
    H3                  = 0x5d,
    H4                  = 0x5e,
    H5                  = 0x5f,
    H6                  = 0x60,
    Head                = 0x61,
    Header              = 0x62,
    Hgroup              = 0x63,
    Hr                  = 0x64,
    Html                = 0x65,
    I                   = 0x66,
    IFrame              = 0x67,
    Image               = 0x68,
    Img                 = 0x69,
    Input               = 0x6a,
    Ins                 = 0x6b,
    Isindex             = 0x6c,
    Kbd                 = 0x6d,
    Keygen              = 0x6e,
    Label               = 0x6f,
    Legend              = 0x70,
    Li                  = 0x71,
    LinearGradient      = 0x72,
    Link                = 0x73,
    Listing             = 0x74,
    Main                = 0x75,
    MAlignMark          = 0x76,
    Map                 = 0x77,
    Mark                = 0x78,
    Marquee             = 0x79,
    Math                = 0x7a,
    Menu                = 0x7b,
    Meta                = 0x7c,
    Meter               = 0x7d,
    MFenced             = 0x7e,
    MGlyph              = 0x7f,
    Mi                  = 0x80,
    Mn                  = 0x81,
    Mo                  = 0x82,
    Ms                  = 0x83,
    MText               = 0x84,
    MultiCol            = 0x85,
    Nav                 = 0x86,
    NextId              = 0x87,
    Nobr                = 0x88,
    NoEmbed             = 0x89,
    NoFrames            = 0x8a,
    NoScript            = 0x8b,
    Object              = 0x8c,
    Ol                  = 0x8d,
    OptGroup            = 0x8e,
    Option              = 0x8f,
    Output              = 0x90,
    P                   = 0x91,
    Param               = 0x92,
    Path                = 0x93,
    Picture             = 0x94,
    PlainText           = 0x95,
    Pre                 = 0x96,
    Progress            = 0x97,
    Q                   = 0x98,
    RadialGradient      = 0x99,
    Rb                  = 0x9a,
    Rp                  = 0x9b,
    Rt                  = 0x9c,
    Rtc                 = 0x9d,
    Ruby                = 0x9e,
    S                   = 0x9f,
    Samp                = 0xa0,
    Script              = 0xa1,
    Section             = 0xa2,
    Select              = 0xa3,
    Slot                = 0xa4,
    Small               = 0xa5,
    Source              = 0xa6,
    Spacer              = 0xa7,
    Span                = 0xa8,
    Strike              = 0xa9,
    Strong              = 0xaa,
    Style               = 0xab,
    Sub                 = 0xac,
    Summary             = 0xad,
    Sup                 = 0xae,
    Svg                 = 0xaf,
    Table               = 0xb0,
    Tbody               = 0xb1,
    Td                  = 0xb2,
    Template            = 0xb3,
    TextArea            = 0xb4,
    TextPath            = 0xb5,
    TFoot               = 0xb6,
    Th                  = 0xb7,
    Thead               = 0xb8,
    Time                = 0xb9,
    Title               = 0xba,
    Tr                  = 0xbb,
    Track               = 0xbc,
    Tt                  = 0xbd,
    U                   = 0xbe,
    Ul                  = 0xbf,
    Var                 = 0xc0,
    Video               = 0xc1,
    Wbr                 = 0xc2,
    Xmp                 = 0xc3,
    _LastEntry          = 0xc4
}
