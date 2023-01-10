use std::collections::HashMap;

pub struct Parser {
    cur_text: String,
    cur_i: usize,
}

pub enum JsonType {
    Bool(bool),
    Number(f32),
    String(String),
    Array(Vec<JsonType>),
    Object(HashMap<String, JsonType>),
    Null,
}

impl JsonType {
    pub fn print(&self) {
        match self {
            _ => {
                unimplemented!();
            }
        }
    }
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            cur_text: "".to_string(),
            cur_i: 0,
        }
    }

    fn cur_char(&self) -> char {
        // self.print_cur_char_loc();
        self.cur_text.chars().nth(self.cur_i).unwrap()
    }

    // Only supports one line json, redo or completely remove later
    fn print_cur_char_loc(&self) {
        // print!("{}", self.cur_text);
        let chars: Vec<char> = self.cur_text.chars().skip(0).take(self.cur_i).collect();
        let slice: String = chars.into_iter().collect();
        println!("{}", slice);
        for _ in 0..self.cur_i {
            print!(" ");
        }
        print!("A\n");
    }

    fn expect_char(&self, to_expect: char) {
        if self.cur_char() != to_expect {
            panic!("Expected: '{}' but got: '{}'!", to_expect, self.cur_char())
        }
    }

    fn ignore_white_space(&mut self) {
        while self.cur_char() == ' '
            || self.cur_char() == '\n'
            || self.cur_char() == '\t'
            || self.cur_char() == '\r'
        {
            self.cur_i += 1;
        }
    }

    pub fn load(&mut self, new_text: String) {
        self.cur_text = new_text;
        self.cur_i = 0;
    }

    fn parse_string(&mut self) -> String {
        self.expect_char('"');

        self.cur_i += 1;

        let mut to_return = "".to_string();
        loop {
            if self.cur_char() == '"' {
                break;
            }

            to_return.push(self.cur_char());

            self.cur_i += 1;
        }

        self.cur_i += 1;
        self.ignore_white_space();

        return to_return;
    }

    fn parse_bool(&mut self) -> bool {
        let keyword_len;

        if self.cur_char() == 'f' {
            keyword_len = 5;
        } else {
            keyword_len = 4;
        }

        let chars: Vec<char> = self
            .cur_text
            .chars()
            .skip(self.cur_i)
            .take(keyword_len)
            .collect();
        let slice: String = chars.into_iter().collect();

        self.cur_i += keyword_len;

        if slice == "true" {
            return true;
        } else if slice == "false" {
            return false;
        } else {
            panic!("Expected true or false, got: {}", slice);
        }
    }

    fn parse_number(&mut self) -> f32 {
        let mut stringed_number = "".to_string();

        loop {
            match self.cur_char() {
                ' ' | ',' | '\n' | '\t' | ']' | '}' => {
                    break;
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => {
                    if self.cur_char() == '.' {
                        if stringed_number.contains(".") {
                            panic!("Tried to put two '.' in a number!")
                        } else {
                            stringed_number.push('.');
                        }
                    } else {
                        stringed_number.push(self.cur_char());
                    }
                }

                _ => {
                    panic!("Something went wrong in number parsing!")
                }
            }

            self.cur_i += 1;
        }

        return stringed_number.parse().unwrap();
    }

    fn parse_array(&mut self) -> Vec<JsonType> {
        let mut to_return: Vec<JsonType> = Vec::new();

        self.expect_char('[');

        self.cur_i += 1;

        loop {
            self.ignore_white_space();

            match self.cur_char() {
                't' | 'f' => {
                    let result = self.parse_bool();
                    // println!("Added key {} to awway", result.clone().to_string());
                    to_return.push(JsonType::Bool(result));
                }

                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => {
                    let result = self.parse_number();
                    // println!("Added key {} to array", result.clone().to_string());
                    to_return.push(JsonType::Number(result));
                }

                '"' => {
                    let result = self.parse_string();
                    // println!("Added key {} to array", result.clone().to_string());
                    to_return.push(JsonType::String(result));
                }

                '[' => {
                    let result = self.parse_array();

                    // println!("Added array to array");

                    to_return.push(JsonType::Array(result));
                }

                'n' => {
                    let chars: Vec<char> = self.cur_text.chars().skip(self.cur_i).take(4).collect();
                    let slice: String = chars.into_iter().collect();

                    self.cur_i += 4;

                    if slice == "null" {
                        // println!("Added null to array");

                        to_return.push(JsonType::Null);
                    } else {
                        panic!("Expected null found something else")
                    }
                }

                ']' => {}

                '{' => {
                    let result = self.parse_object();
                    // println!("Added object to array");
                    to_return.push(JsonType::Object(result));
                }

                _ => {
                    unimplemented!("Strange path in array parser")
                }
            }

            self.ignore_white_space();

            if self.cur_char() == ']' {
                break;
            }

            self.expect_char(',');
            self.cur_i += 1;
        }
        self.cur_i += 1;

        return to_return;
    }

    fn parse_object(&mut self) -> HashMap<String, JsonType> {
        let mut to_return: HashMap<String, JsonType> = HashMap::new();

        self.expect_char('{');

        self.cur_i += 1;

        if self.cur_char() == '}' {
            self.cur_i += 1;
            return to_return;
        }

        loop {
            self.ignore_white_space();

            let new_key = self.parse_string();
            if new_key == "" {
                panic!("Empty key!");
            }

            self.expect_char(':');
            self.cur_i += 1;
            self.ignore_white_space();

            match self.cur_char() {
                't' | 'f' => {
                    let result = self.parse_bool();
                    // println!(
                    // "Added key {} with value {}",
                    // new_key.clone(),
                    // result.clone().to_string()
                    // );
                    to_return.insert(new_key, JsonType::Bool(result));
                }

                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => {
                    let result = self.parse_number();
                    // println!(
                    //     "Added key {} with value {}",
                    //     new_key.clone(),
                    //     result.clone().to_string()
                    // );
                    to_return.insert(new_key, JsonType::Number(result));
                }

                '"' => {
                    let result = self.parse_string();
                    // println!(
                    //     "Added key {} with value \"{}\"",
                    //     new_key.clone(),
                    //     result.clone().to_string()
                    // );
                    to_return.insert(new_key, JsonType::String(result));
                }

                '[' => {
                    let result = self.parse_array();

                    // println!("Added key {} with value array", new_key.clone());

                    to_return.insert(new_key, JsonType::Array(result));
                }

                'n' => {
                    let chars: Vec<char> = self.cur_text.chars().skip(self.cur_i).take(4).collect();
                    let slice: String = chars.into_iter().collect();

                    self.cur_i += 4;

                    if slice == "null" {
                        // println!("Added key {} with value null", new_key.clone(),);

                        to_return.insert(new_key, JsonType::Null);
                    } else {
                        // panic!("Expected null found something else")
                    }
                }

                '}' => {}

                '{' => {
                    let result = self.parse_object();

                    // println!("Added key {} as obj", new_key.clone());

                    to_return.insert(new_key, JsonType::Object(result));
                }

                _ => {
                    unimplemented!("Strange path")
                }
            }

            self.ignore_white_space();

            if self.cur_char() == '}' {
                self.cur_i += 1;
                break;
            }

            self.expect_char(',');
            self.cur_i += 1;
        }

        return to_return;
    }

    pub fn parse(&mut self) -> JsonType {
        if self.cur_i != 0 {
            panic!("Please load a new json file!")
        }

        JsonType::Object(self.parse_object())
    }
}
