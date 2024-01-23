/*
NOTE: this module needs to be rewritten/reworked; specifically the part for handling
Token::Identifier; we need to break that up into an actual identifier/keyword, vs a
number literal (either integer or float), where a float might have a period. I think
it makes sense to handle certain complicated and low-level parsing tasks in this.

oh, also we should rename this new/rewitten module lexer.rs, which better represents
what the module is accomplishing- being a lexer. it will include these token typedefs,
and then we can move onto transforming the "token stream" into a "sytax tree".
*/
#[derive(Debug)]
pub struct TokenStream {
    pub tokens: Vec<TokenItem>,
    pub line_offsets: Vec<usize>,
}

impl TokenStream {
    pub fn new() -> Self {
        TokenStream {
            tokens: vec![],
            line_offsets: vec![],
        }
    }

    pub fn offset_to_line_col(&self, offset: usize) -> (usize, usize) {
        let mut line_start = 0;
        let mut line_num = 1;
        for i in self.line_offsets.iter().copied() {
            if i >= offset {
                break;
            }
            line_num += 1;
            line_start = i + 1;
        };
        let col = offset - line_start + 1;
        (line_num, col)
    }
}

#[derive(Debug)]
pub struct TokenItem {
    pub token: Token,
    pub start: usize,
    pub len: usize,
}

#[derive(Debug)]
pub enum Token {
    LineFeed,
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

#[derive(Debug)]
pub struct TokenError {
    pub partial_stream: TokenStream,
    pub error_kind: TokenErrorKind,
    pub loc: usize,
}

#[derive(Debug)]
pub enum TokenErrorKind {
    UnknownToken(u8),
    UnclosedString,
    UnclosedComment,
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
pub fn parse(source: &[u8]) -> Result<TokenStream, TokenError> {
    let mut ts = TokenStream::new();
    let mut cursor = 0;
    while cursor < source.len() {
        let c = source[cursor];
        let start = cursor;
        cursor += 1;
        // handle identifiers
        if is_identifier(c) {
            let mut i = cursor;
            loop {
                // looking for any non digit/alpha/unicode
                // worry about legal syntax later
                if i >= source.len() || !is_identifier(source[i]) {
                    ts.tokens.push(TokenItem {
                        token: Token::Identifier,
                        start,
                        // TODO double check math
                        len: i - start,
                    });
                    cursor = i;
                    break;
                }
                i += 1;
            }
            continue;
        }
        // handle strings
        if c == DOUBLEQUOTE || c == SINGLEQUOTE {
            let mut i = cursor;
            let mut escape = false;
            loop {
                if i >= source.len() {
                    return Err(TokenError {
                        partial_stream: ts,
                        error_kind: TokenErrorKind::UnclosedString,
                        loc: start
                    });
                }
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
                        start: start,
                        // TODO double check math
                        len: i - start,
                    });
                    cursor = i;
                    break;
                }
            }
            continue;
        }
        // handle comments
        if c == SLASH {
            let mut i = cursor;
            let c2 = source[i];
            if c2 == SLASH {
                // line comment
                loop {
                    if i > source.len() || source[i] == LINEFEED {
                        ts.tokens.push(TokenItem {
                            token: Token::LineComment
                            start,
                            len: i - start,
                        });
                        cursor = i;
                        break;
                    }
                    i += 1;
                }
            } else if c2 == ASTERISK {
                // block comment
                loop {
                    if i + 1 >= source.len() {
                        return Err(TokenError {
                            partial_stream: ts,
                            error_kind: TokenErrorKind::UnclosedComment,
                            loc: start
                        });
                    }
                    let d = source[i];
                    i += 1;
                    if d == ASTERISK && source[i] == SLASH {
                        ts.tokens.push(TokenItem {
                            token: Token::BlockComment,
                            start,
                            len: i - start,
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
            x if x == LINEFEED => {
                ts.line_offsets.push(start);
                Token::LineFeed
            },
            x if x == SPACE => continue,
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

            x => {
                return Err(TokenError {
                    partial_stream: ts,
                    error_kind: TokenErrorKind::UnknownToken(x),
                    loc: start
                });
            },
        };
        ts.tokens.push(TokenItem {
            token,
            start,
            len: 1,
        });
    }
    Ok(ts)
}
