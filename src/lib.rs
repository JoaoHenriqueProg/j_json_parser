// V 2
// https://github.com/JoaoHenriqueProg/j_json_parser

use std::fmt::format;

pub struct Parser {
    cur_text: String,
    cur_i: usize,
}

#[derive(Clone, Debug)]
pub enum JsonType {
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonType>),
    Object(JsonObject),
    Null,
}

#[derive(Clone, Debug)]
pub struct JsonObject {
    children: Vec<(String, JsonType)>,
}

impl ToString for JsonObject {
    fn to_string(&self) -> String {
        self.priv_stringify(0)
    }
}

impl JsonObject {
    fn new() -> JsonObject {
        JsonObject {
            children: Vec::new(),
        }
    }

    // TODO: refatorar, usa muita String, dava de fazer usando só números
    fn priv_stringify_number(&self, val: f64) -> String {
        let stringed = val.to_string();

        if stringed.ends_with("000") {
            return format!("{:e}", val);
        }
        if stringed.starts_with("0.000") {
            return format!("{:e}", val);
        }

        return stringed;
    }

    fn priv_stringify(&self, indent: u8) -> String {
        if self.children.len() == 0 {
            return "{}".to_string();
        }

        let mut to_return: String = "{".to_string();

        for to_spit in &self.children {
            to_return.push('\n');

            for _ in 0..(indent + 1) * 2 {
                to_return.push(' ');
            }

            match &to_spit.1 {
                JsonType::Object(val) => {
                    to_return.push_str(&format!(
                        "\"{}\": {},",
                        to_spit.0,
                        val.priv_stringify(indent + 1)
                    ));
                }

                JsonType::Bool(val) => {
                    to_return.push_str(&format!("\"{}\": {},", to_spit.0, val));
                }
                JsonType::Number(val) => {
                    to_return.push_str(&format!(
                        "\"{}\": {},",
                        to_spit.0,
                        self.priv_stringify_number(*val)
                    ));
                }
                JsonType::String(val) => {
                    to_return.push_str(&format!("\"{}\": \"{}\",", to_spit.0, val));
                }
                JsonType::Array(val) => {
                    to_return.push_str(&format!(
                        "\"{}\": {},",
                        to_spit.0,
                        &self.priv_stringify_array(val, indent + 1)
                    ));
                }
                JsonType::Null => {
                    to_return.push_str(&format!("\"{}\": null,", to_spit.0));
                }
            }
        }

        to_return.pop();

        to_return.push('\n');
        for _ in 0..indent * 2 {
            to_return.push(' ');
        }

        to_return.push_str("}");
        return to_return;
    }

    fn priv_stringify_array(&self, array_to_stringify: &Vec<JsonType>, indent: u8) -> String {
        if array_to_stringify.len() == 0 {
            return "[]".to_string();
        }

        let mut to_return: String = "[".to_string();

        for to_spit in array_to_stringify {
            to_return.push('\n');

            for _ in 0..(indent + 1) * 2 {
                to_return.push(' ');
            }

            match &to_spit {
                JsonType::Object(val) => {
                    to_return.push_str(&format!("{},", val.priv_stringify(indent + 1)));
                }
                JsonType::Bool(val) => {
                    to_return.push_str(&format!("{},", val));
                }
                JsonType::Number(val) => {
                    to_return.push_str(&format!("{},", val));
                }
                JsonType::String(val) => {
                    to_return.push_str(&format!("{},", val));
                }
                JsonType::Array(val) => {
                    to_return
                        .push_str(&format!("{},", &self.priv_stringify_array(val, indent + 1)));
                }
                JsonType::Null => {
                    to_return = "null,".to_string();
                }
            }
        }

        to_return.pop();

        to_return.push('\n');
        for _ in 0..indent * 2 {
            to_return.push(' ');
        }

        to_return.push_str("]");
        return to_return;
    }

    fn get_index_of_key<T: ToString>(&self, key: T) -> i64 {
        for (i, child) in self.children.iter().enumerate() {
            if child.0 == key.to_string() {
                return i as i64;
            }
        }
        return -1;
    }

    pub fn get<T: ToString>(&self, key: T) -> Result<JsonType, JsonError> {
        let i = self.get_index_of_key(key.to_string());
        if i == -1 {
            return Err(JsonError::KeyNotFound);
        }

        return Ok(self.children[i as usize].1.clone());
    }

    pub fn get_bool<T: ToString>(&self, key: T) -> Result<bool, JsonError> {
        let i = self.get_index_of_key(key.to_string());
        if i == -1 {
            return Err(JsonError::KeyNotFound);
        }

        match self.children[i as usize].1 {
            JsonType::Bool(val) => {
                return Ok(val);
            }
            _ => {
                return Err(JsonError::WrongTypeValueRequest);
            }
        }
    }

    pub fn get_number<T: ToString>(&self, key: T) -> Result<f64, JsonError> {
        let i = self.get_index_of_key(key.to_string());
        if i == -1 {
            return Err(JsonError::KeyNotFound);
        }

        match self.children[i as usize].1 {
            JsonType::Number(val) => {
                return Ok(val);
            }
            _ => {
                return Err(JsonError::WrongTypeValueRequest);
            }
        }
    }

    pub fn get_string<T: ToString>(&self, key: T) -> Result<String, JsonError> {
        let i = self.get_index_of_key(key.to_string());
        if i == -1 {
            return Err(JsonError::KeyNotFound);
        }

        match &self.children[i as usize].1 {
            JsonType::String(val) => {
                return Ok(val.clone());
            }
            _ => {
                return Err(JsonError::WrongTypeValueRequest);
            }
        }
    }

    pub fn get_array<T: ToString>(&self, key: T) -> Result<Vec<JsonType>, JsonError> {
        let i = self.get_index_of_key(key.to_string());
        if i == -1 {
            return Err(JsonError::KeyNotFound);
        }

        match &self.children[i as usize].1 {
            JsonType::Array(val) => {
                return Ok(val.clone());
            }
            _ => {
                return Err(JsonError::WrongTypeValueRequest);
            }
        }
    }

    pub fn get_obj<T: ToString>(&self, key: T) -> Result<JsonObject, JsonError> {
        let i = self.get_index_of_key(key.to_string());
        if i == -1 {
            return Err(JsonError::KeyNotFound);
        }

        match &self.children[i as usize].1 {
            JsonType::Object(val) => {
                return Ok(val.clone());
            }
            _ => {
                return Err(JsonError::WrongTypeValueRequest);
            }
        }
    }

    fn set_entry(&mut self, to_add: (String, JsonType)) {
        let i = self.get_index_of_key(&to_add.0);
        if i == -1 {
            self.children.push(to_add);
        } else {
            self.children[i as usize] = to_add;
        }
    }

    pub fn set_bool<T: ToString>(&mut self, new_key: T, new_value: bool) {
        let to_add = (new_key.to_string(), JsonType::Bool(new_value));
        self.set_entry(to_add);
    }
    pub fn set_number<T: ToString>(&mut self, new_key: T, new_value: f64) {
        let to_add = (new_key.to_string(), JsonType::Number(new_value));
        self.set_entry(to_add);
    }
    pub fn set_string<T: ToString>(&mut self, new_key: T, new_value: T) {
        let to_add = (new_key.to_string(), JsonType::String(new_value.to_string()));
        self.set_entry(to_add);
    }
    pub fn set_array<T: ToString>(&mut self, new_key: T, new_value: Vec<JsonType>) {
        let to_add = (new_key.to_string(), JsonType::Array(new_value));
        self.set_entry(to_add);
    }
    pub fn set_null<T: ToString>(&mut self, new_key: T) {
        let to_add = (new_key.to_string(), JsonType::Null);
        self.set_entry(to_add);
    }
    /// .Requires the obj to not exist
    pub fn insert_obj<T: ToString>(&mut self, new_key: T) {
        let to_add = (new_key.to_string(), JsonType::Object(JsonObject::new()));
        if self.get_index_of_key(new_key.to_string()) != -1 {
            panic!("Cannot insert an object that already exists!")
        }
        self.set_entry(to_add);
    }
}

#[derive(Debug)]
pub enum JsonError {
    KeyNotFound,
    WrongTypeValueRequest,
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

    fn get_substr(&mut self, len: usize) -> String {
        let to_return = self.cur_text.chars().skip(self.cur_i).take(len).collect();

        self.cur_i += len;

        return to_return;
    }

    // TODO: Only supports one line json, redo or completely remove later
    #[allow(dead_code)]
    fn print_cur_char_loc(&self) {
        let chars: Vec<char> = self.cur_text.chars().skip(0).take(self.cur_i).collect();
        let slice: String = chars.into_iter().collect();
        println!("{}", slice);
        for _ in 0..self.cur_i {
            print!(" ");
        }
        print!("A\n");
    }

    fn expect_char(&mut self, to_expect: char) -> bool {
        if self.cur_char() != to_expect {
            return false;
        }

        self.cur_i += 1;
        true
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

    pub fn load<T: ToString>(&mut self, new_text: T) {
        self.cur_text = new_text.to_string();
        self.cur_i = 0;
    }

    fn parse_string(&mut self) -> String {
        self.expect_char('"');

        let mut to_return = "".to_string();
        while self.cur_char() != '"' {
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

        let chars = self.get_substr(keyword_len);

        if chars == "true" {
            return true;
        } else if chars == "false" {
            return false;
        } else {
            panic!("Expected true or false, got: {}", chars);
        }
    }

    fn parse_number(&mut self) -> f64 {
        let mut stringed_number = "".to_string();

        while ![' ', ',', '\n', '\t', ']', '}'].contains(&self.cur_char()) {
            match self.cur_char() {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' | '-' | 'e' => {
                    let prev = self.cur_text.chars().nth(self.cur_i - 1).unwrap();
                    if self.cur_char() == '-' && stringed_number.len() != 0 {
                        if prev != 'e' {
                            panic!("Minus sign can only be at the start of a Number or after an exponential sign!")
                        }
                    }
                    if self.cur_char() == 'e' && stringed_number.len() == 0 {
                        panic!("Exponential sign can not be at the start of a Number!")
                    }
                    if self.cur_char() == 'e' {
                        if stringed_number.contains("e") {
                            panic!("Tried to put two 'e' in a number!")
                        }
                    }
                    if self.cur_char() == '.' {
                        if stringed_number.contains(".") {
                            panic!("Tried to put two '.' in a number!")
                        }
                    }
                    stringed_number.push(self.cur_char());
                }

                _ => {
                    panic!(
                        "Something went wrong in number parsing! Found char: {}",
                        self.cur_char()
                    )
                }
            }

            self.cur_i += 1;
        }

        return stringed_number.parse().unwrap();
    }

    fn parse_array(&mut self) -> Vec<JsonType> {
        let mut to_return: Vec<JsonType> = Vec::new();

        self.expect_char('[');

        loop {
            self.ignore_white_space();

            match self.cur_char() {
                't' | 'f' => {
                    let result = self.parse_bool();
                    to_return.push(JsonType::Bool(result));
                }

                // TODO: aparentemente, números em json não podem começar com .
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' | '-' => {
                    let result = self.parse_number();
                    to_return.push(JsonType::Number(result));
                }

                '"' => {
                    let result = self.parse_string();
                    to_return.push(JsonType::String(result));
                }

                '[' => {
                    let result = self.parse_array();
                    to_return.push(JsonType::Array(result));
                }

                'n' => {
                    let chars = self.get_substr(4);

                    if chars == "null" {
                        to_return.push(JsonType::Null);
                    } else {
                        panic!("Expected null found something else")
                    }
                }

                ']' => {}

                '{' => {
                    let result = self.parse_object();
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
            self.ignore_white_space();

            if self.cur_char() == ']' {
                panic!("Trailling commas are not allowed after the last item of an array!")
            }
        }
        self.cur_i += 1;

        return to_return;
    }

    fn parse_object(&mut self) -> JsonObject {
        let mut to_return: JsonObject = JsonObject::new();

        self.expect_char('{');

        // * em caso de bloco vazio
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
            self.ignore_white_space();

            match self.cur_char() {
                't' | 'f' => {
                    let result = self.parse_bool();
                    to_return.children.push((new_key, JsonType::Bool(result)));
                }

                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' | '-' => {
                    let result = self.parse_number();
                    to_return.children.push((new_key, JsonType::Number(result)));
                }

                '"' => {
                    let result = self.parse_string();
                    to_return.children.push((new_key, JsonType::String(result)));
                }

                '[' => {
                    let result = self.parse_array();
                    to_return.children.push((new_key, JsonType::Array(result)));
                }

                'n' => {
                    let chars = self.get_substr(4);

                    if chars == "null" {
                        to_return.children.push((new_key, JsonType::Null));
                    } else {
                        panic!("Expected null found something else");
                    }
                }

                '}' => {}

                '{' => {
                    let result = self.parse_object();
                    to_return.children.push((new_key, JsonType::Object(result)));
                }

                _ => {
                    unimplemented!("Strange path, found char: {}", self.cur_char())
                }
            }

            self.ignore_white_space();

            if self.cur_char() == '}' {
                self.cur_i += 1;
                break;
            }

            self.expect_char(',');

            self.ignore_white_space();

            if self.cur_char() == '}' {
                panic!("Trailling commas are not allowed after the last child of an object!")
            }
        }

        return to_return;
    }

    pub fn parse(&mut self) -> JsonObject {
        if self.cur_i != 0 {
            panic!("Please load a new json file!")
        }

        self.ignore_white_space();
        self.parse_object()
    }
}
