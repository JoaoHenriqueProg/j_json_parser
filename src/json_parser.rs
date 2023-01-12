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
    Object(Vec<(String, JsonType)>),
    Null,
}

#[derive(Debug)]
pub enum JsonError {
    KeyNotFound,
    TriedToAccessChildrenOnANonObjectJsonType,
    WrongTypeValueRequest,
}

impl JsonType {
    pub fn get(&self, key: String) -> Result<JsonType, JsonError> {
        match self {
            JsonType::Object(val) => {
                // yes, it's slow, I'm gonna chage it later

                for child in val {
                    if child.0 == key {
                        return Ok(child.1.clone());
                    }
                }

                return Err(JsonError::KeyNotFound);
            }
            _ => {
                return Err(JsonError::TriedToAccessChildrenOnANonObjectJsonType);
            }
        }
    }

    pub fn get_bool(&self, key: String) -> Result<bool, JsonError> {
        match self {
            JsonType::Object(val) => {
                // yes, it's slow, I'm gonna chage it later

                for child in val {
                    if child.0 == key {
                        match child.1 {
                            JsonType::Bool(val) => {
                                return Ok(val);
                            }
                            _ => {
                                return Err(JsonError::WrongTypeValueRequest);
                            }
                        }
                    }
                }

                return Err(JsonError::KeyNotFound);
            }
            _ => {
                return Err(JsonError::TriedToAccessChildrenOnANonObjectJsonType);
            }
        }
    }

    pub fn get_number(&self, key: String) -> Result<f64, JsonError> {
        match self {
            JsonType::Object(val) => {
                // yes, it's slow, I'm gonna chage it later

                for child in val {
                    if child.0 == key {
                        match child.1 {
                            JsonType::Number(val) => {
                                return Ok(val);
                            }
                            _ => {
                                return Err(JsonError::WrongTypeValueRequest);
                            }
                        }
                    }
                }

                return Err(JsonError::KeyNotFound);
            }
            _ => {
                return Err(JsonError::TriedToAccessChildrenOnANonObjectJsonType);
            }
        }
    }
    
    pub fn get_string(&self, key: String) -> Result<String, JsonError> {
        match self {
            JsonType::Object(val) => {
                // yes, it's slow, I'm gonna chage it later

                for child in val {
                    if child.0 == key {
                        match &child.1 {
                            JsonType::String(val) => {
                                return Ok(val.clone());
                            }
                            _ => {
                                return Err(JsonError::WrongTypeValueRequest);
                            }
                        }
                    }
                }

                return Err(JsonError::KeyNotFound);
            }
            _ => {
                return Err(JsonError::TriedToAccessChildrenOnANonObjectJsonType);
            }
        }
    }
    
    pub fn get_array(&self, key: String) -> Result<Vec<JsonType>, JsonError> {
        match self {
            JsonType::Object(val) => {
                // yes, it's slow, I'm gonna chage it later

                for child in val {
                    if child.0 == key {
                        match &child.1 {
                            JsonType::Array(val) => {
                                return Ok(val.clone());
                            }
                            _ => {
                                return Err(JsonError::WrongTypeValueRequest);
                            }
                        }
                    }
                }

                return Err(JsonError::KeyNotFound);
            }
            _ => {
                return Err(JsonError::TriedToAccessChildrenOnANonObjectJsonType);
            }
        }
    }

    pub fn get_obj(&self, key: String) -> Result<Vec<(String, JsonType)>, JsonError> {
        match self {
            JsonType::Object(val) => {
                // yes, it's slow, I'm gonna chage it later

                for child in val {
                    if child.0 == key {
                        match &child.1 {
                            JsonType::Object(val) => {
                                return Ok(val.clone());
                            }
                            _ => {
                                return Err(JsonError::WrongTypeValueRequest);
                            }
                        }
                    }
                }

                return Err(JsonError::KeyNotFound);
            }
            _ => {
                return Err(JsonError::TriedToAccessChildrenOnANonObjectJsonType);
            }
        }
    }
            
    pub fn print(&self) {
        match self {
            JsonType::Object(_) => {
                println!("{}", self.stringify());
            }

            _ => {
                panic!("It is not possible to print something else than a JsonType::Object")
            }
        }
    }

    pub fn stringify(&self) -> String {
        return self.priv_stringify(self, 0);
    }

    fn priv_stringify(&self, to_spit: &JsonType, indent: u8) -> String {
        let mut to_return: String = "".to_string();

        match to_spit {
            JsonType::Object(val) => {
                to_return.push_str("{\n");
                for element in val {
                    for _ in 0..(indent + 1) * 2 {
                        to_return.push(' ');
                    }

                    to_return.push_str(&format!(
                        "\"{}\": {}\n",
                        element.0,
                        self.priv_stringify(&element.1, indent + 1)
                    ));
                }
                for _ in 0..indent * 2 {
                    to_return.push(' ');
                }
                to_return.push_str("},");
            }

            JsonType::Bool(val) => {
                to_return = format!("{},", val);
            }
            JsonType::Number(val) => {
                to_return = format!("{},", val);
            }
            JsonType::String(val) => {
                to_return = format!("\"{}\",", val);
            }
            JsonType::Array(val) => {
                to_return.push_str("[\n");
                for element in val.iter() {
                    for _ in 0..(indent + 1) * 2 {
                        to_return.push(' ');
                    }

                    to_return.push_str(&self.priv_stringify(element, indent + 1));
                    to_return.push('\n');
                }
                for _ in 0..indent * 2 {
                    to_return.push(' ');
                }

                to_return.push_str("],");
            }
            JsonType::Null => {
                to_return = "null,".to_string();
            }
        }

        return to_return;
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

    fn parse_number(&mut self) -> f64 {
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

    fn parse_object(&mut self) -> Vec<(String, JsonType)> {
        let mut to_return: Vec<(String, JsonType)> = Vec::new();

        self.expect_char('{');

        self.cur_i += 1;

        if self.cur_char() == '}' {
            self.cur_i += 1;
            return to_return;
        }

        loop {
            self.ignore_white_space();

            self.print_cur_char_loc();
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
                    to_return.push((new_key, JsonType::Bool(result)));
                }

                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => {
                    let result = self.parse_number();
                    // println!(
                    //     "Added key {} with value {}",
                    //     new_key.clone(),
                    //     result.clone().to_string()
                    // );
                    to_return.push((new_key, JsonType::Number(result)));
                }

                '"' => {
                    let result = self.parse_string();
                    // println!(
                    //     "Added key {} with value \"{}\"",
                    //     new_key.clone(),
                    //     result.clone().to_string()
                    // );
                    to_return.push((new_key, JsonType::String(result)));
                }

                '[' => {
                    let result = self.parse_array();

                    // println!("Added key {} with value array", new_key.clone());

                    to_return.push((new_key, JsonType::Array(result)));
                }

                'n' => {
                    let chars: Vec<char> = self.cur_text.chars().skip(self.cur_i).take(4).collect();
                    let slice: String = chars.into_iter().collect();

                    self.cur_i += 4;

                    if slice == "null" {
                        // println!("Added key {} with value null", new_key.clone(),);

                        to_return.push((new_key, JsonType::Null));
                    } else {
                        // panic!("Expected null found something else")
                    }
                }

                '}' => {}

                '{' => {
                    let result = self.parse_object();

                    // println!("Added key {} as obj", new_key.clone());

                    to_return.push((new_key, JsonType::Object(result)));
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
