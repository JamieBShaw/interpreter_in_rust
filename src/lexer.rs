use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,
    // Identifiers + literals
    IDENT(String),
    INT(String),
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQUAL,
    NOTEQUAL,
    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::IDENT(x) => write!(f, "Ident({})", x),
            Token::INT(x) => write!(f, "Int({})", x),
            Token::ILLEGAL => write!(f, "Illegal"),
            Token::EOF => write!(f, "Eof"),
            Token::ASSIGN => write!(f, "Assign"),
            Token::PLUS => write!(f, "Plus"),
            Token::COMMA => write!(f, "Comma"),
            Token::SEMICOLON => write!(f, "Semicolon"),
            Token::LPAREN => write!(f, "Lparen"),
            Token::RPAREN => write!(f, "Rparen"),
            Token::LBRACE => write!(f, "LSquirly"),
            Token::RBRACE => write!(f, "RSquirly"),
            Token::FUNCTION => write!(f, "Function"),
            Token::LET => write!(f, "Let"),
            Token::MINUS => write!(f, "Minus"),
            Token::BANG => write!(f, "Bang"),
            Token::ASTERISK => write!(f, "Asteriks"),
            Token::SLASH => write!(f, "Slash"),
            Token::LT => writeln!(f, "LessThan"),
            Token::GT => write!(f, "GreaterThan"),
            Token::TRUE => write!(f, "True"),
            Token::FALSE => write!(f, "False"),
            Token::IF => write!(f, "If"),
            Token::ELSE => write!(f, "Else"),
            Token::RETURN => write!(f, "Return"),
            Token::EQUAL => write!(f, "Equal"),
            Token::NOTEQUAL => write!(f, "NotEqual"),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Lexer {
    input: Vec<u8>,
    pos: usize,
    read_pos: usize,
    ch: u8,
}

impl Lexer {
    fn new(input: String) -> Self {
        let mut s = Self {
            input: input.into_bytes(),
            read_pos: 0,
            ch: 0,
            pos: 0,
        };

        s.read_char();
        s
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = Default::default();
        } else {
            self.ch = self.input[self.read_pos];
        }

        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::EQUAL // return out of the function
                } else {
                    Token::ASSIGN
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NOTEQUAL // return out of the function
                } else {
                    Token::BANG
                }
            }
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            b'+' => Token::PLUS,
            b'-' => Token::MINUS,
            b'*' => Token::ASTERISK,
            b'/' => Token::SLASH,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b'0'..=b'9' => return Token::INT(self.read_int()),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                return match ident.as_str() {
                    "fn" => Token::FUNCTION,
                    "let" => Token::LET,
                    "if" => Token::IF,
                    "return" => Token::RETURN,
                    "true" => Token::TRUE,
                    "false" => Token::FALSE,
                    "else" => Token::ELSE,
                    _ => Token::IDENT(ident),
                };
            }
            0 => Token::EOF,
            _ => unreachable!(),
        };

        self.read_char();
        tok
    }

    fn peek_char(&self) -> u8 {
        if self.read_pos >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_pos];
        }
    }

    fn read_ident(&mut self) -> String {
        let current_pos = self.pos;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[current_pos..self.pos]).to_string();
    }

    fn read_int(&mut self) -> String {
        let current_pos = self.pos;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[current_pos..self.pos]).to_string();
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input: String = "=+(){},;".into();
        let expected_tokens = vec![
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
        ];
        let mut lexer = Lexer::new(input);

        for expected_tok in expected_tokens {
            let actual_tok = lexer.next_token();
            println!("actual: {:?}, expected: {:?}", actual_tok, expected_tok);

            assert_eq!(actual_tok, expected_tok);
        }
    }

    #[test]
    fn test_next_token_complex() {
        let input = "let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        ";

        let expected_tokens = vec![
            Token::LET,
            Token::IDENT("five".to_owned()),
            Token::ASSIGN,
            Token::INT("5".to_owned()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_owned()),
            Token::ASSIGN,
            Token::INT("10".to_owned()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_owned()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_owned()),
            Token::COMMA,
            Token::IDENT("y".to_owned()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_owned()),
            Token::PLUS,
            Token::IDENT("y".to_owned()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_owned()),
            Token::ASSIGN,
            Token::IDENT("add".to_owned()),
            Token::LPAREN,
            Token::IDENT("five".to_owned()),
            Token::COMMA,
            Token::IDENT("ten".to_owned()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::EOF,
        ];
        let mut lexer = Lexer::new(input.to_owned());

        for expected_tok in expected_tokens {
            let actual_tok = lexer.next_token();
            println!("actual: {:?}, expected: {:?}", actual_tok, expected_tok);
            assert_eq!(actual_tok, expected_tok);
        }
    }

    #[test]
    fn test_next_token_complex_even_more_complex() {
        let input = "let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        ";

        let expected_tokens = vec![
            Token::LET,
            Token::IDENT("five".to_owned()),
            Token::ASSIGN,
            Token::INT("5".to_owned()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_owned()),
            Token::ASSIGN,
            Token::INT("10".to_owned()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_owned()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_owned()),
            Token::COMMA,
            Token::IDENT("y".to_owned()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_owned()),
            Token::PLUS,
            Token::IDENT("y".to_owned()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_owned()),
            Token::ASSIGN,
            Token::IDENT("add".to_owned()),
            Token::LPAREN,
            Token::IDENT("five".to_owned()),
            Token::COMMA,
            Token::IDENT("ten".to_owned()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT("5".to_owned()),
            Token::SEMICOLON,
            Token::INT("5".to_owned()),
            Token::LT,
            Token::INT("10".to_owned()),
            Token::GT,
            Token::INT("5".to_owned()),
            Token::SEMICOLON,
            Token::EOF,
        ];
        let mut lexer = Lexer::new(input.to_owned());

        for expected_tok in expected_tokens {
            let actual_tok = lexer.next_token();
            println!("actual: {:?}, expected: {:?}", actual_tok, expected_tok);
            assert_eq!(actual_tok, expected_tok);
        }
    }

    #[test]
    fn test_next_token_complex_even_more_and_more_complex() {
        let input = "let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        ";

        let expected_tokens = vec![
            Token::LET,
            Token::IDENT("five".to_owned()),
            Token::ASSIGN,
            Token::INT("5".to_owned()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_owned()),
            Token::ASSIGN,
            Token::INT("10".to_owned()),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_owned()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_owned()),
            Token::COMMA,
            Token::IDENT("y".to_owned()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_owned()),
            Token::PLUS,
            Token::IDENT("y".to_owned()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_owned()),
            Token::ASSIGN,
            Token::IDENT("add".to_owned()),
            Token::LPAREN,
            Token::IDENT("five".to_owned()),
            Token::COMMA,
            Token::IDENT("ten".to_owned()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT("5".to_owned()),
            Token::SEMICOLON,
            Token::INT("5".to_owned()),
            Token::LT,
            Token::INT("10".to_owned()),
            Token::GT,
            Token::INT("5".to_owned()),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT("5".to_owned()),
            Token::LT,
            Token::INT("10".to_owned()),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT("10".to_owned()),
            Token::EQUAL,
            Token::INT("10".to_owned()),
            Token::SEMICOLON,
            Token::INT("10".to_owned()),
            Token::NOTEQUAL,
            Token::INT("9".to_owned()),
            Token::SEMICOLON,
            Token::EOF,
        ];
        let mut lexer = Lexer::new(input.to_owned());

        for expected_tok in expected_tokens {
            let actual_tok = lexer.next_token();
            println!("actual: {:?}, expected: {:?}", actual_tok, expected_tok);
            assert_eq!(actual_tok, expected_tok);
        }
    }
}
