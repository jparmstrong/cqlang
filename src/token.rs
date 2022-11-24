use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<&'static str, TokenType> = {
        HashMap::from([
            ("and",     TokenType::And),
            ("or",      TokenType::Or),
            ("if",      TokenType::If),
            ("else",    TokenType::Else),
            ("fun",     TokenType::Fun),
            ("true",    TokenType::True),
            ("false",   TokenType::False),
            ("while",   TokenType::While),
            ("var",     TokenType::Var),
        ])
    };
}

#[derive(Clone, Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star, Colon,

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
    number: f64,
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
        let r = self.peeki(1);
        self.current+=1;
        r
    }

    fn peek(&self) -> char {
        self.peeki(1)
    }

    fn peeki(&self, i: usize) -> char {
        match self.source[self.current..(self.current+i)].last() {
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
        let t = Token { toktype: toktype, lexeme: String::new(), line: self.line, number: 0. };
        self.tokens.push(t);
    }

    fn add_token_number(&mut self, lexeme: String) {
        let num = lexeme.parse::<f64>().unwrap();
        let t = Token { toktype: TokenType::Number, lexeme: lexeme, line: self.line, number: num };
        self.tokens.push(t);
    }

    fn add_token_identifer(&mut self, lexeme: String) {

        let tokentype = match HASHMAP.get(&*lexeme) {
            Some(x) => x.clone(),
            None => TokenType::Identifier
        };

        let t = Token { toktype: tokentype, lexeme: lexeme, line: self.line, number: 0. };
        self.tokens.push(t);
    }

    fn is_alpha(&self, c: char) -> bool {
        c >= 'a' && c <= 'z' ||
        c >= 'A' && c <= 'Z' ||
        c >= '_'
    }

    fn alpha(&mut self) {
        while self.is_alpha(self.peek()) { self.advance(); };
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.add_token_identifer(lexeme)
    }

    fn is_numeric(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) {
        while self.is_numeric(self.peek()) { self.advance(); };

        if self.peek() == '.' && self.is_numeric(self.peeki(2)) {
            self.advance();

            while self.is_numeric(self.peek()) { self.advance(); };
        }
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.add_token_number(lexeme);
    }

    fn scan(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            let c = self.advance();

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
                ':' => self.add_token(TokenType::Colon),
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
                'a'..='z' | 'A'..='Z' | '_' => { self.alpha() },
                '0'..='9' => { self.number() },
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
