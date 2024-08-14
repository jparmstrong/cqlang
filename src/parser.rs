
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

struct BinOp {
    op: Token,
    left : Token,
    right: Token
}

struct UniOp {
    op: Token,
    right: Token
}

impl Parser {

    fn advance(&mut self) -> Token {
        let r = self.peeki(1);
        self.current+=1;
        r
    }

    fn peek(&self) -> Token {
        self.peeki(1)
    }

    fn peeki(&self, i: usize) -> Token {
        match self.tokens[self.current..(self.current+i)].last() {
            Some(x) => *x,
            None => '\0'
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // fn eval(&mut self) {
    //     let t = self.advance();
    //     while !self.is_at_end() {
    //         match t.token {
    //             String => if a
    //         }
    //     }
    // }

}

pub fn build_parser(tokens: Vec<Token>) {
    let mut parser = Parser {
        tokens: tokens,
        current: 0,
    };


}