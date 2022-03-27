#[derive(Debug)]
enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

struct Token {
    type_: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

impl Token {
    fn new(type_: TokenType, lexeme: String, literal: String, line: usize) -> Token {
        Token {
            type_,
            lexeme,
            literal,
            line,
        }
    }

    fn to_string(&self) -> String {
        format!("{:?} {} {}", self.type_, self.lexeme, self.literal)
    }
}
