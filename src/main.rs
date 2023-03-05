
#[derive(Debug)]
enum TokenType {
    //Add,
    //Sub,
    //Mult,
    //Div,
    Operator,
    Lparen,
    Rparen,
    Number,
}

#[derive(Debug)]
struct Token {
    value: String,
    token_type: TokenType,
    pos: usize,
}

const OPERATORS: [char; 4] = ['+', '*', '-', '/'];

struct Tokenizer {
    position: usize,
    source_chars: Vec<char>,
    tokens: Vec<Token>,
    curr_char: char,
}
impl Tokenizer {
    fn from(source: &String) -> Tokenizer {
        let mut char_vec: Vec<char> = source.chars().collect();
        char_vec.push('\0');
        return Tokenizer {
            position: 0,
            curr_char: char_vec[0],
            source_chars: char_vec,
            tokens: Vec::<Token>::new(),
        };
    }
    fn on_last_char(&self) -> bool {
        self.position == self.source_chars.len() - 1
    }
    fn eat(&mut self) {
        self.position += 1;
        self.curr_char = self.source_chars[self.position];
    }
    fn get_num_token(&mut self) -> Token {
        let mut num_str = String::from("");
        while self.curr_char.is_numeric() {
            num_str.push(self.curr_char);
            if self.on_last_char() {
                break;
            }
            self.eat();
        }
        return Token {
            value: num_str,
            token_type: TokenType::Number,
            pos: self.position,
        };
    }
    fn tokenize(&mut self) -> &Vec<Token> {
        while !self.on_last_char() {
            if self.curr_char == '(' {
                self.tokens.push(Token {
                    value: String::from("("),
                    token_type: TokenType::Lparen,
                    pos: self.position,
                });
                self.eat();
            } else if self.curr_char == ')' {
                self.tokens.push(Token {
                    value: String::from(")"),
                    token_type: TokenType::Rparen,
                    pos: self.position,
                });
                self.eat();
            } else if self.curr_char.is_numeric() {
                let t = self.get_num_token();
                self.tokens.push(t);
            } else if OPERATORS.contains(&self.curr_char) {
                self.tokens.push(Token {
                    value: String::from(self.curr_char),
                    token_type: TokenType::Operator,
                    pos: self.position,
                });
                self.eat();
            } else {
                self.eat();
            }
        }
        return &self.tokens;
    }
}

struct Parser<'a> {
    tokens: &'a Vec<Token>,
    position: usize,
    current_token: &'a Token,
}

impl Parser<'_>{
    fn from_tokens(source_tokens: &Vec<Token>) -> Parser {
        let p = Parser {
            tokens: source_tokens,
            position: 0,
            current_token: source_tokens.get(0).unwrap(),
        };

        return p;
    }
}

fn main() {
    let input_str: String = String::from("7+4 * (9+1-2)*3");
    let mut tokenizer: Tokenizer = Tokenizer::from(&input_str);
    let tokens: &Vec<Token> = tokenizer.tokenize();
    println!("Here are the tokens");
    for token in tokens {
        println!(
            "Token(value: {}, type: {:?}, pos: {})",
            token.value, token.token_type, token.pos
        );
    }
}
