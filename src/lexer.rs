use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(String),
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let
}

pub struct Lexer {
    input: Vec<u8>,
    pos: usize,
    read_pos: usize,
    ch: u8,
}


impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Self {
            input: input.into_bytes(),
            pos: 0,
            read_pos: 0,
            ch: 0,
        };

        l.read_char();
        return l;
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = 0;
        }
        else {
            self.ch = self.input[self.read_pos];
        }

        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let token = match self.ch {
            b'=' => Token::Assign,
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            0 => Token::Eof,
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    return Ok(self.read_identifier())
                }
                else if self.ch.is_ascii_digit(){
                    return Ok(self.read_number())
                }
                else {
                    Token::Illegal
                }
            }
        };

        self.read_char();
        return Ok(token);
    }

    fn skip_whitespace(&mut self) {
        println!("{}", self.ch);
        while self.ch.is_ascii_whitespace() {
            println!("-{}", self.ch);
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> Token {
        let pos = self.pos;
        while self.ch.is_ascii_alphabetic() {
            self.read_char();
        }

        let identifier = String::from_utf8_lossy(&self.input[pos..self.pos]).to_string();

        match identifier.as_str() {
            "fn" => Token::Function,
            "let" => Token::Let,
            val => Token::Ident(val.to_string())
        }
    }

    fn read_number(&mut self) -> Token {
        let pos = self.pos;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        println!("{:?}", &self.input);
        return Token::Int(String::from_utf8_lossy(&self.input[pos..self.pos]).to_string());
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_next_token() -> Result<()> {
        let input = "=+(){},;".to_owned();
        let mut lexer = Lexer::new(input);

        let test_tokens = [
            Token::Assign, 
            Token::Plus, 
            Token::LParen,
            Token::RParen, 
            Token::LBrace, 
            Token::RBrace, 
            Token::Comma, 
            Token::Semicolon
        ];

        for token in test_tokens {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, received: {:?}", token, next_token);
            assert_eq!(token, next_token);
        }

        return Ok(());
    }


    #[test]
    fn test_next_token() -> Result<()> {
        let input = r#"let five = 5;
                        let ten = 10;
                        let add = fn(x, y) {
                            x + y;
                        };
                        let result = add(five, ten);"#;

        let mut lexer = Lexer::new(input.to_owned());

        let test_tokens = [
            Token::Let,
            Token::Ident("five".to_owned()),
            Token::Assign,
            Token::Int("5".to_owned()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_owned()),
            Token::Assign,
            Token::Int("10".to_owned()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_owned()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_owned()),
            Token::Comma,
            Token::Ident("y".to_owned()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_owned()),
            Token::Plus,
            Token::Ident("y".to_owned()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_owned()),
            Token::Assign,
            Token::Ident("add".to_owned()),
            Token::LParen,
            Token::Ident("five".to_owned()),
            Token::Comma,
            Token::Ident("ten".to_owned()),
            Token::RParen,
            Token::Semicolon
        ];


        for (i, token) in test_tokens.iter().enumerate() {
            
            let next_token = lexer.next_token()?;
            println!("{}: expected: {:?}, received: {:?}", i, token, next_token);
            assert_eq!(token, &next_token);
        }

        return Ok(());
    }

}
