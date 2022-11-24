use std::str::Chars;

#[derive(Clone, Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    EOF
}

#[derive(Clone, Debug)]
pub struct Token {
    toktype: TokenType,
    lexeme: String,
    line: u32
}

pub struct Tokenizer {
    tokens: Vec<Token>,
    source: Vec<char>,
    start: usize,
    current: usize,
    line: u32,
}

impl Tokenizer {

    fn advance(&mut self) -> char {
        let r = match self.source[self.current..(self.current+1)].first() {
            Some(x) => *x,
            None => '\0'
        };
        self.current+=1;
        r
    }

    fn peek(&self) -> char {
        match self.source[self.current..(self.current+1)].first() {
            Some(x) => *x,
            None => '\0'
        }
    }

    fn pmatch(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false };
        if self.peek() != expected { return false };
        self.current+=1;
        return true;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, toktype : TokenType) {
        let t = Token { toktype: toktype, lexeme: String::new(), line: self.line };
        self.tokens.push(t);
    }

    fn is_alpha(c: &str) -> bool {
        c >= "a" && c <= "z" ||
        c >= "A" && c <= "Z" ||
        c >= "_"
    }

    fn is_numeric(c: &str) -> bool {
        c >= "0" && c <= "9"
    }

    fn number() {

    }

    fn scan(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            let c = self.advance();
            print!("[{}]", c);
            match c {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                ';' => self.add_token(TokenType::Semicolon),
                '/' => self.add_token(TokenType::Slash),
                '*' => self.add_token(TokenType::Star),
                '!' =>
                   if self.pmatch('=') {
                        self.add_token(TokenType::BangEqual)
                    } else {
                        self.add_token(TokenType::Bang)
                    },
                '=' =>
                    if self.pmatch('=') {
                        self.add_token(TokenType::EqualEqual)
                    } else {
                        self.add_token(TokenType::Equal)
                    },
                '>' =>
                    if self.pmatch('=') {
                        self.add_token(TokenType::GreaterEqual)
                    } else {
                        self.add_token(TokenType::Greater)
                    },
                '<' =>
                    if self.pmatch('=') {
                        self.add_token(TokenType::LessEqual)
                    } else {
                        self.add_token(TokenType::Less)
                    },
                'a'..='z' | 'A'..='Z' | '_'  => self.number(),
                '0'..='9' => (),
                ' ' | '\t' | '\r' => (),
                '\n' => self.line+=1,
                _ => print!("!{}!", c as u32),
            }
        }
    }

}

pub fn build_tokenizer(source: &String) -> Vec<Token> {
    let mut tk = Tokenizer {
        tokens: Vec::new(),
        source: source.chars().collect(),
        start: 0,
        current: 0,
        line: 1
    };
    tk.scan();
    tk.tokens
}
