use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(String),
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,

    True, 
    False,
    If,
    Else,
    Return,
    Equal,
    NotEqual,
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

    fn peek_char(&self) -> u8 {
        if self.read_pos >= self.input.len() {
            return 0;
        }
        return self.input[self.read_pos];
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let token = match self.ch {
            b'=' =>  {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Equal
                }
                else {
                    Token::Assign
                }
            },
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'<' => Token::Lt,
            b'>' => Token::Gt,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEqual
                }
                else {
                    Token::Bang
                }
            }
            b'-' => Token::Minus,
            b'/' => Token::Slash,
            b'*' => Token::Asterisk,
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
        while self.ch.is_ascii_whitespace() {
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
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,

            val => Token::Ident(val.to_string())
        }
    }

    fn read_number(&mut self) -> Token {
        let pos = self.pos;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

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


    #[test]
    fn test_extended_tokens() -> Result<()> {
        let input = r#"!-/*5;
        5 < 10 > 5;
        10 == 10;
        10 != 9;
        "#;
        
        let mut lexer = Lexer::new(input.to_string());

        let test_tokens = [
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Int("5".to_string()),
            Token::Lt,
            Token::Int("10".to_string()),
            Token::Gt,
            Token::Int("5".to_string()),
            Token::Semicolon,

            Token::Int("10".to_string()),
            Token::Equal,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Int("10".to_string()),
            Token::NotEqual,
            Token::Int("9".to_string()),
            Token::Semicolon,
        ];

        for (i, token) in test_tokens.iter().enumerate() {
            
            let next_token = lexer.next_token()?;
            println!("{}: expected: {:?}, received: {:?}", i, token, next_token);
            assert_eq!(token, &next_token);
        }

        return Ok(());
    }


    #[test]
    fn test_if_statement() -> Result<()> {
        let input = r#"
        if (5 < 10) {
        return true;
        } else {
        return false;
        }
        "#;

        let mut lexer = Lexer::new(input.to_string());

        let test_tokens = [
            Token::If,
            Token::LParen,
            Token::Int("5".to_string()),
            Token::Lt,
            Token::Int("10".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace
        ];

        for (i, token) in test_tokens.iter().enumerate() {
            
            let next_token = lexer.next_token()?;
            println!("{}: expected: {:?}, received: {:?}", i, token, next_token);
            assert_eq!(token, &next_token);
        }

        return Ok(());

    }

}
