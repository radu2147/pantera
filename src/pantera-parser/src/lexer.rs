use crate::token::{Token, TokenType};
use crate::errors::ParseError;

pub struct Lexer {
    pub source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: u128,
}

impl Lexer {
    pub fn scan_tokens(mut self) -> Result<Vec<Token>, ParseError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens.push(Token {
            typ: TokenType::Eof,
            line: self.line,
        });
        Ok(self.tokens)
    }

    pub fn match_equal(
        &mut self,
        condition_met_token_type: TokenType,
        alternative_token_type: TokenType,
    ) {
        if self.match_char('=') {
            self.tokens.push(Token {
                typ: condition_met_token_type,
                line: self.line,
            })
        } else {
            self.tokens.push(Token {
                typ: alternative_token_type,
                line: self.line,
            })
        }
    }

    fn scan_token(&mut self) -> Result<(), ParseError> {
        let c = self.advance();
        match c {
            '(' => self.tokens.push(Token {
                typ: TokenType::LeftBrace,
                line: self.line,
            }),
            ')' => self.tokens.push(Token {
                typ: TokenType::RightBrace,
                line: self.line,
            }),
            '{' => self.tokens.push(Token {
                typ: TokenType::LeftParen,
                line: self.line,
            }),
            '}' => self.tokens.push(Token {
                typ: TokenType::RightParen,
                line: self.line,
            }),
            '^' => self.tokens.push(Token {
                typ: TokenType::Pow,
                line: self.line,
            }),
            ',' => self.tokens.push(Token {
                typ: TokenType::Comma,
                line: self.line,
            }),
            '.' => self.tokens.push(Token {
                typ: TokenType::Dot,
                line: self.line,
            }),
            '+' => self.tokens.push(Token {
                typ: TokenType::Plus,
                line: self.line,
            }),
            '-' => self.tokens.push(Token {
                typ: TokenType::Minus,
                line: self.line,
            }),
            '*' => self.tokens.push(Token {
                typ: TokenType::Star,
                line: self.line,
            }),
            ';' => self.tokens.push(Token {
                typ: TokenType::Semicolon,
                line: self.line,
            }),
            '=' => self.tokens.push(Token {
                typ: TokenType::Equal,
                line: self.line,
            }),
            '>' => {
                self.match_equal(TokenType::GraterEqual, TokenType::Grater);
            }
            '<' => {
                self.match_equal(TokenType::LessEqual, TokenType::Less);
            }
            '/' => {
                if self.match_char('/') {
                    while let Some(chr) = self.peek() {
                        if chr != '\n' {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.tokens.push(Token {
                        typ: TokenType::Slash,
                        line: self.line,
                    })
                }
            }
            ' ' | '\t' | '\r' => {}
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let mut number_literal = String::new();
                number_literal.push(c);
                while let Some(chr) = self.peek() {
                    if !Self::is_digit(&chr) {
                        break;
                    }
                    number_literal.push(chr);
                    self.advance();
                }
                if let Some(chr) = self.peek() {
                    if chr == '.' {
                        if let Some(chr_next) = self.peek_next() {
                            if Self::is_digit(&chr_next) {
                                number_literal.push(chr);
                                self.advance();
                                while let Some(chr) = self.peek() {
                                    if !Self::is_digit(&chr) {
                                        break;
                                    }
                                    number_literal.push(chr);
                                    self.advance();
                                }
                            }
                        }
                    }
                }
                self.tokens.push(Token {
                    typ: TokenType::Number(number_literal.parse::<f32>().unwrap()),
                    line: self.line,
                })
            }
            'a' | 'A' | 'b' | 'B' | 'c' | 'C' | 'd' | 'D' | 'e' | 'E' | 'f' | 'F' | 'g' | 'G'
            | 'h' | 'H' | 'i' | 'I' | 'j' | 'J' | 'k' | 'K' | 'l' | 'L' | 'm' | 'M' | 'n' | 'N'
            | 'o' | 'O' | 'p' | 'P' | 'q' | 'Q' | 'r' | 'R' | 's' | 'S' | 't' | 'T' | 'u' | 'U'
            | 'v' | 'V' | 'w' | 'W' | 'x' | 'X' | 'y' | 'Y' | 'z' | 'Z' | '_' => {
                let mut identifier_string = String::new();
                identifier_string.push(c);
                while let Some(chr) = self.peek() {
                    if Self::is_alpha(&chr) || Self::is_digit(&chr) {
                        identifier_string.push(chr);
                        self.advance();
                    } else {
                        break;
                    }
                }
                if let Some(tok) = self.get_keyword_token(&identifier_string, self.line) {
                    self.tokens.push(tok)
                } else {
                    self.tokens.push(Token {
                        typ: TokenType::Identifier(identifier_string),
                        line: self.line,
                    })
                }
            }
            '\n' => self.line += 1,
            '"' => {
                let mut string_literal = String::new();
                while let Some(chr) = self.peek() {
                    if chr == '"' {
                        break;
                    }
                    if chr == '\n' {
                        self.line += 1;
                    }
                    string_literal.push(chr);
                    self.advance();
                }
                if self.is_at_end() {
                    return Err(ParseError {
                        message: "Unterminated string.".to_string(),
                        line: self.line,
                    });
                }
                self.advance();
                self.tokens.push(Token {
                    typ: TokenType::String(string_literal),
                    line: self.line,
                });
            }
            _ => {
                return Err(ParseError {
                    message: "Unexpected character.".to_string(),
                    line: self.line,
                })
            }
        };
        Ok(())
    }

    fn get_keyword_token(&self, identifier_string: &str, line: u128) -> Option<Token> {
        match identifier_string {
            "if" => Some(Token {
                typ: TokenType::If,
                line,
            }),
            "is" => Some(Token {
               typ: TokenType::Is,
                line
            }),
            "else" => Some(Token {
                typ: TokenType::Else,
                line,
            }),
            "print" => Some(Token {
                typ: TokenType::Print,
                line,
            }),
            "and" => Some(Token {
                typ: TokenType::And,
                line,
            }),
            "or" => Some(Token {
                typ: TokenType::Or,
                line,
            }),
            "null" => Some(Token {
                typ: TokenType::Nil,
                line,
            }),
            "not" => Some(Token {
                typ: TokenType::Not,
                line,
            }),
            "for" => Some(Token {
                typ: TokenType::For,
                line,
            }),
            "while" => Some(Token {
                typ: TokenType::While,
                line,
            }),
            "return" => Some(Token {
                typ: TokenType::Return,
                line,
            }),
            "true" => Some(Token {
                typ: TokenType::True,
                line,
            }),
            "false" => Some(Token {
                typ: TokenType::False,
                line,
            }),
            "var" => Some(Token {
                typ: TokenType::Var,
                line,
            }),
            "fun" => Some(Token {
                typ: TokenType::Fun,
                line,
            }),
            "break" => Some(Token {
                typ: TokenType::Break,
                line,
            }),
            "const" => Some(Token {
                typ: TokenType::Const,
                line
            }),
            _ => None,
        }
    }

    fn is_alpha(chr: &char) -> bool {
        *chr >= 'a' && *chr <= 'z' || *chr >= 'A' && *chr <= 'Z' || *chr == '_'
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() as i32 {
            return None;
        }
        Some(
            self.source
                .chars()
                .nth((self.current + 1) as usize)
                .unwrap(),
        )
    }

    fn is_digit(chr: &char) -> bool {
        *chr >= '0' && *chr <= '9'
    }

    fn match_char(&mut self, chr: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current as usize).unwrap() != chr {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        Some(self.source.chars().nth(self.current as usize).unwrap())
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source
            .chars()
            .nth((self.current - 1) as usize)
            .unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i32
    }

    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{rng};
    use rand::prelude::IndexedRandom;
    use super::*;

    struct Pair<K, V> {
        key: K,
        value: V
    }

    #[test]
    fn works_fine() {
        let lexer = Lexer::new("fun main");
        let res = lexer.scan_tokens();
        if let Ok(toks) = res {
            assert_eq!(toks.len(), 3);
            assert_eq!(toks.get(0).unwrap().typ, TokenType::Fun);
            assert_eq!(toks.get(1).unwrap().typ, TokenType::Identifier("main".to_string()));
        } else {
            assert!(false);
        }

    }

    #[test]
    fn can_lex_numbers() {
        let numbers = vec![
            "12", "1.2", "1.43", "4", "0", "0.13"
        ];
        let not_numbers = vec![
            "12.a", "14a 2", "1-23", "-13"
        ];
        for number in numbers {
            let lexer = Lexer::new(number);
            let tokens = lexer.scan_tokens().unwrap();
            assert_eq!(tokens.len(), 2);
        }

        for not_number in not_numbers {

            let lexer = Lexer::new(not_number);
            let tokens = lexer.scan_tokens().unwrap();
            assert_ne!(tokens.len(), 2);
        }
    }

    #[test]
    fn can_lex_ids() {
        let ids = [
            "main", "a24", "test_2", "test_1_23_ae", "_smth", "SMTH"
        ];

        let non_ids = [
            "1abc", "for", "-", "/", "*"
        ];

        for id in ids {
            let lexer = Lexer::new(id);
            let tokens = lexer.scan_tokens().unwrap();
            assert_eq!(tokens.len(), 2);
            assert_eq!(tokens.get(0).unwrap().typ, TokenType::Identifier(id.to_string()));
        }

        for non_id in non_ids {
            let lexer = Lexer::new(non_id);
            let tokens = lexer.scan_tokens().unwrap();
            assert!(tokens.len() > 2 || !matches!(tokens.get(0).unwrap().typ, TokenType::Identifier(_)));
        }
    }

    #[test]
    fn can_peek() {
        let mut lexer = Lexer::new("fun ");
        lexer.advance();
        assert_eq!(lexer.peek().unwrap(), 'u');
        lexer.advance();
        lexer.advance();
        assert_eq!(lexer.peek().unwrap(), ' ');
        lexer.advance();
        assert_eq!(lexer.peek(), None);
    }

    #[test]
    fn can_advance() {
        let mut lexer = Lexer::new("fun ");
        lexer.advance();
        assert_eq!(lexer.current, 1);
        lexer.advance();
        lexer.advance();
        assert_eq!(lexer.current, 3);
    }

    #[test]
    fn can_is_at_end() {
        let mut lexer = Lexer::new("fu");
        assert!(!lexer.is_at_end());
        lexer.advance();
        assert!(!lexer.is_at_end());
        lexer.advance();
        assert!(lexer.is_at_end());
    }

    #[test]
    fn can_match_keyword() {

        let keywords = vec![
            Pair {key: "for", value: TokenType::For},
            Pair {key: "var", value: TokenType::Var},
            Pair {key: "const", value: TokenType::Const},
            Pair {key: "if", value: TokenType::If},
            Pair {key: "else", value: TokenType::Else},
            Pair {key: "fun", value: TokenType::Fun},
            Pair {key: "null", value: TokenType::Nil},
            Pair {key: "is", value: TokenType::Is},
            Pair {key: "not", value: TokenType::Not},
            Pair {key: "and", value: TokenType::And},
            Pair {key: "or", value: TokenType::Or},
            Pair {key: "print", value: TokenType::Print},
            Pair {key: "true", value: TokenType::True},
            Pair {key: "false", value: TokenType::False},
            Pair {key: "break", value: TokenType::Break},
            Pair {key: "return", value: TokenType::Return},

        ];

        let random_scenarios: [Pair<fn(&str) -> String, usize>; 7] = [
            Pair{ key: |x: &str| {format!("let {x};")}, value: 1},
            Pair{ key: |x: &str| {format!("for kw in {x}")},value: 3},
            Pair{ key: |x: &str| {format!("{x} + 2")}, value: 0},
            Pair{ key: |x: &str| {format!("({x})")}, value: 1},
            Pair{ key: |x: &str| {format!("{x}.some")}, value: 0},
            Pair{ key: |x: &str| {format!("-{x}")},value: 1},
            Pair{ key: |x: &str| {format!("if ( \n ( {x} ) )")}, value: 3}
        ];

        let mut rng = rng();

        for keyword in &keywords {
            let Some(scenario) =random_scenarios.choose(&mut rng) else {panic!("Cannot find scenarios")};
            let lexer = Lexer::new((scenario.key)(keyword.key).as_str());
            let Ok(tokens)= lexer.scan_tokens() else { panic!("Something really went wrong") };
            assert_eq!(tokens.get(scenario.value).unwrap().typ, keyword.value);
        }

        // if a kw has a suffix alphanumeric char it's an identifier
        for kw in &keywords {
            let Some(scenario) =random_scenarios.choose(&mut rng) else {panic!("Cannot find scenarios")};
            let lexer = Lexer::new((scenario.key)(format!("{}2", kw.key).as_str()).as_str());
            let Ok(tokens)= lexer.scan_tokens() else { panic!("Something really went wrong") };
            assert!(matches!(tokens.get(scenario.value).unwrap().typ, TokenType::Identifier(_)));
        }

        // if a kw has a prefix char it's an identifier
        for kw in &keywords {
            let Some(scenario) =random_scenarios.choose(&mut rng) else {panic!("Cannot find scenarios")};
            let lexer = Lexer::new((scenario.key)(format!("x{}", kw.key).as_str()).as_str());
            let Ok(tokens)= lexer.scan_tokens() else { panic!("Something really went wrong") };
            assert!(matches!(tokens.get(scenario.value).unwrap().typ, TokenType::Identifier(_)));
        }

        // if a kw has both prefix and suffix char it's an identifier
        for kw in &keywords {
            let Some(scenario) =random_scenarios.choose(&mut rng) else {panic!("Cannot find scenarios")};
            let lexer = Lexer::new((scenario.key)(format!("x{}1", kw.key).as_str()).as_str());
            let Ok(tokens)= lexer.scan_tokens() else { panic!("Something really went wrong") };
            assert!(matches!(tokens.get(scenario.value).unwrap().typ, TokenType::Identifier(_)));
        }
    }

    #[test]
    fn can_match_operator() {
        let operators = vec![
            Pair {key: "+", value: TokenType::Plus},
            Pair {key: "-", value: TokenType::Minus},
            Pair {key: "/", value: TokenType::Slash},
            Pair {key: "*", value: TokenType::Star},
            Pair {key: "^", value: TokenType::Pow},
            Pair {key: ">", value: TokenType::Grater},
            Pair {key: ">=", value: TokenType::GraterEqual},
            Pair {key: "<", value: TokenType::Less},
            Pair {key: "<=", value: TokenType::LessEqual},
            Pair {key: "=", value: TokenType::Equal},
        ];

        let random_scenarios: [Pair<fn(&str) -> String, usize>; 7] = [
            Pair{ key: |x: &str| {format!("3 {x} 54;")}, value: 1},
            Pair{ key: |x: &str| {format!("if variable {x} true")},value: 2},
            Pair{ key: |x: &str| {format!("{x} true")}, value: 0},
            Pair{ key: |x: &str| {format!("({x})")}, value: 1},
            Pair{ key: |x: &str| {format!("{x}2")}, value: 0},
            Pair{ key: |x: &str| {format!("-{x}")},value: 1},
            Pair{ key: |x: &str| {format!("if ( \n ( {x} ) )")}, value: 3}
        ];

        let mut rng = rng();

        for operator in &operators {
            let Some(scenario) =random_scenarios.choose(&mut rng) else {panic!("Cannot find scenarios")};
            let lexer = Lexer::new((scenario.key)(operator.key).as_str());
            let Ok(tokens)= lexer.scan_tokens() else { panic!("Something really went wrong") };
            assert_eq!(tokens.get(scenario.value).unwrap().typ, operator.value);
        }
    }

    #[test]
    fn can_match_semicolon() {
        let lexer = Lexer::new("var x;");
        let Ok(tokens)= lexer.scan_tokens() else { panic!("Something really went wrong") };
        assert_eq!(tokens.get(2).unwrap().typ, TokenType::Semicolon);
    }

    #[test]
    fn can_match_dot() {
        let lexer = Lexer::new("x.property");
        let Ok(tokens)= lexer.scan_tokens() else { panic!("Something really went wrong") };
        assert_eq!(tokens.get(1).unwrap().typ, TokenType::Dot);

        let lexer = Lexer::new("var x = 3.13;");
        let Ok(tokens)= lexer.scan_tokens() else { panic!("Something really went wrong") };
        for token in tokens {
            assert_ne!(token.typ, TokenType::Dot);
        }
    }

    #[test]
    fn can_match_comma() {
        let lexer = Lexer::new("x(1, 2, 3)");
        let Ok(tokens)= lexer.scan_tokens() else { panic!("Something really went wrong") };
        assert_eq!(tokens.get(3).unwrap().typ, TokenType::Comma);
        assert_eq!(tokens.get(5).unwrap().typ, TokenType::Comma);
    }

    #[test]
    fn can_match_braces() {
        let lexer = Lexer::new("x((3+2)*4)");
        let Ok(tokens)= lexer.scan_tokens() else { panic!("Something really went wrong") };
        assert_eq!(tokens.get(1).unwrap().typ, TokenType::LeftBrace);
        assert_eq!(tokens.get(2).unwrap().typ, TokenType::LeftBrace);

        assert_eq!(tokens.get(6).unwrap().typ, TokenType::RightBrace);
        assert_eq!(tokens.get(9).unwrap().typ, TokenType::RightBrace);
    }

    #[test]
    fn can_match_parens() {
        let lexer = Lexer::new("if trouble {\nrun()\n}");
        let Ok(tokens)= lexer.scan_tokens() else { panic!("Something really went wrong") };
        assert_eq!(tokens.get(2).unwrap().typ, TokenType::LeftParen);
        assert_eq!(tokens.get(6).unwrap().typ, TokenType::RightParen)
    }
}