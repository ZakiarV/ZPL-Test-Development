use std::iter::Peekable;
use std::str::Chars;


#[derive(Debug, PartialEq)]
pub enum TokenType {
    Keyword(String), // Keywords
    Identifier(String), // Identifiers
    Number(i64), // Numbers
    StringLiteral(String), // Strings
    Operator(String), // Operators
    Separator(char), // Separators
    EOF, // End of file
}


pub fn tokenize(input: &str) -> Vec<TokenType> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '+' | '-' | '*' | '/' | '%' | '=' => {
                tokens.push(TokenType::Operator(c.to_string()));
                chars.next();
            }
            '(' | ')' | '{' | '}' | '[' | ']' | ',' | ';' => {
                tokens.push(TokenType::Separator(c));
                chars.next();
            }
            '0'..='9' => {
                let num = consume_numbers(&mut chars);
                tokens.push(TokenType::Number(num));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let ident = consume_identifier(&mut chars);
                if is_keyword(&ident) {
                    tokens.push(TokenType::Keyword(ident));
                } else {
                tokens.push(TokenType::Identifier(ident));
                }
            }
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            _ => {
                panic!("Unknown character: {}", c);
            }
        }
    }
    tokens.push(TokenType::EOF);
    tokens
}


fn consume_numbers(chars: &mut Peekable<Chars>) -> i64 {
    let mut num = String::new();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                num.push(c);
                chars.next();
            }
            _ => break,
        }
    }

    num.parse::<i64>().unwrap()
    
}


fn consume_identifier(chars: &mut Peekable<Chars>) -> String {
    let mut ident = String::new();

    while let Some(&c) = chars.peek() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                if (ident.is_empty() && c.is_numeric()) {
                    panic!("Identifiers cannot start with a number");
                }
                ident.push(c);
                chars.next();
            }
            _ => break,
        }
    }

    ident
}


fn is_keyword(ident: &str) -> bool {
    match ident {
        "if" | "elif" | "else" | "while" | "for" | "str" | "int" | "float" | "bool" | "char" | "return" => true,
        _ => false,
    }
}