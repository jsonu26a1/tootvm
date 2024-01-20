fn main() {
    println!("Hello, world!");
}

struct TokenStream {
    pub tokens: Vec<TokenItem>,
    pub cursor: usize,
}

pub struct TokenItem {
    pub token: Token,
    pub start: usize,
    pub length: usize,
}

pub enum Token {
    LineFeed,
    Space,
    Exclaimation,
    Hash,
    Dollar,
    Percent,
    Ampersand,

    LeftParen,
    RightParen,
    Asterisk,
    Plus,
    Comma,
    Minus,
    Period,
    Slash,

    Colon,
    Semicolon,
    LessThan,
    Equal,
    GreaterThan,
    Question,
    AtSign,

    LeftSquare,
    BackSlash,
    RightSquare,
    Caret,
    Underscore,
    Tick,

    LeftCurly,
    VerticalBar,
    RightCurly,
    Tilde,

    Identifier,
    SingleQuoteString,
    DoubleQuoteString,
    LineComment,
    BlockComment,

    Unknown,
}

static LINEFEED: u8 = 0x0A;
static SPACE: u8 = 0x20;
static EXCLAIMATION: u8 = 0x21;
static DOUBLEQUOTE: u8 = 0x22;
static HASH: u8 = 0x23;
static DOLLAR: u8 = 0x24;
static PERCENT: u8 = 0x25;
static AMPERSAND: u8 = 0x26;
static SINGLEQUOTE: u8 = 0x27;

static LEFTPAREN: u8 = 0x28;
static RIGHTPAREN: u8 = 0x29;
static ASTERISK: u8 = 0x2A;
static PLUS: u8 = 0x2B;
static COMMA: u8 = 0x2C;
static MINUS: u8 = 0x2D;
static PERIOD: u8 = 0x2E;
static SLASH: u8 = 0x2F;

static COLON: u8 = 0x3A;
static SEMICOLON: u8 = 0x3B;
static LESSTHAN: u8 = 0x3C;
static EQUAL: u8 = 0x3D;
static GREATERTHAN: u8 = 0x3E;
static QUESTION: u8 = 0x3F;
static ATSIGN: u8 = 0x40;

static LEFTSQUARE: u8 = 0x5B;
static BACKSLASH: u8 = 0x5C;
static RIGHTSQUARE: u8 = 0x5D;
static CARET: u8 = 0x5E;
static UNDERSCORE: u8 = 0x5F;
static TICK: u8 = 0x60;

static LEFTCURLY: u8 = 0x7B;
static VERTICALBAR: u8 = 0x7C;
static RIGHTCURLY: u8 = 0x7D;
static TILDE: u8 = 0x7E;

static UNICODE_START: u8 = 0x80;

static DIGIT_START: u8 = 0x30;
static DIGIT_END: u8 = 0x39;

static UPPER_START: u8 = 0x41;
static UPPER_END: u8 = 0x5A;

static LOWER_START: u8 = 0x61;
static LOWER_END: u8 = 0x7A;

fn is_identifier(c: u8) -> bool {
    (c >= DIGIT_START && c <= DIGIT_END) ||
    (c >= UPPER_START && c <= UPPER_END) ||
    (c >= LOWER_START && c <= LOWER_END) ||
    c == UNDERSCORE ||
    c >= UNICODE_START
}

// TODO bounds checks
fn stage1(source: &[u8]) -> TokenStream {
    let mut ts = TokenStream { tokens: vec![], cursor: 0 };
    let mut cursor = 0;
    loop {
        let c = source[cursor];
        // handle identifiers
        if is_identifier(c) {
            let mut i = cursor + 1;
            loop {
                let d = source[i];
                i += 1;
                // looking for any non digit/alpha/unicode
                // worry about legal syntax later
                if !is_identifier(d) {
                    ts.tokens.push(TokenItem {
                        token: Token::Identifier,
                        start: cursor,
                        // TODO double check math
                        length: i - cursor,
                    });
                    cursor = i;
                    break;
                }
            }
            continue;
        }
        // handle strings
        if c == DOUBLEQUOTE || c == SINGLEQUOTE {
            let mut i = cursor + 1;
            let mut escape = false;
            loop {
                let d = source[i];
                i += 1;
                // planned escapes include 7bit 2digit hex, 24bit 6digit hex, whitespaces n,r,t, null, backslash
                // but they won't be implemented here
                if d == BACKSLASH {
                    escape = !escape;
                } else {
                    escape = false;
                }
                if d == c && !escape {
                    ts.tokens.push(TokenItem {
                        token: if c == DOUBLEQUOTE {
                                Token::DoubleQuoteString
                            } else {
                                Token::SingleQuoteString
                            },
                        start: cursor,
                        // TODO double check math
                        length: i - cursor,
                    });
                    cursor = i;
                    break;
                }
            }
            continue;
        }
        // handle comments
        if c == SLASH {
            let mut i = cursor + 1;
            let c2 = source[i];
            if c2 == SLASH {
                // line comment
                todo!();
            } else if c2 == ASTERISK {
                // block comment
                loop {
                    let d = source[i];
                    if d == ASTERISK && source[i+1] == SLASH {
                        ts.tokens.push(TokenItem {
                            token: Token::BlockComment,
                            start: cursor,
                            length: todo!(),
                        });
                        cursor = i;
                        break;
                    }
                }
            }
            continue;
        }
        // handle others
        let token = match c {
            x if x == LINEFEED => Token::LineFeed,
            x if x == SPACE => Token::Space,
            x if x == EXCLAIMATION => Token::Exclaimation,
            x if x == HASH => Token::Hash,
            x if x == DOLLAR => Token::Dollar,
            x if x == PERCENT => Token::Percent,
            x if x == AMPERSAND => Token::Ampersand,

            x if x == LEFTPAREN => Token::LeftParen,
            x if x == RIGHTPAREN => Token::RightParen,
            x if x == ASTERISK => Token::Asterisk,
            x if x == PLUS => Token::Plus,
            x if x == COMMA => Token::Comma,
            x if x == MINUS => Token::Minus,
            x if x == PERIOD => Token::Period,
            x if x == SLASH => Token::Slash,

            x if x == COLON => Token::Colon,
            x if x == SEMICOLON => Token::Semicolon,
            x if x == LESSTHAN => Token::LessThan,
            x if x == EQUAL => Token::Equal,
            x if x == GREATERTHAN => Token::GreaterThan,
            x if x == QUESTION => Token::Question,
            x if x == ATSIGN => Token::AtSign,

            x if x == LEFTSQUARE => Token::LeftSquare,
            x if x == BACKSLASH => Token::BackSlash,
            x if x == RIGHTSQUARE => Token::RightSquare,
            x if x == CARET => Token::Caret,
            x if x == UNDERSCORE => Token::Underscore,
            x if x == TICK => Token::Tick,

            x if x == LEFTCURLY => Token::LeftCurly,
            x if x == VERTICALBAR => Token::VerticalBar,
            x if x == RIGHTCURLY => Token::RightCurly,
            x if x == TILDE => Token::Tilde,

            _ => Token::Unknown,
        };
        ts.tokens.push(TokenItem {
            token,
            start: cursor,
            length: todo!(),
        })
    }
    ts.cursor = cursor;
    return ts;
}