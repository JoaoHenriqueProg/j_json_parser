use j_json_parser::{JsonType, Parser};

fn main() {
    let mut parser = Parser::new();
    //parser.load(r#"
    let to_parse = r#"
{"result":
  {
  "integers": {
    "positive_integer": 123,
    "negative_integer": -123,
    "zero": 0
  },
  "floating_points": {
    "positive_float": 123.45,
    "negative_float": -123.45
  },
  "exponents": {
    "positive_exponent": 1.23e6,
    "negative_positive_exponent": -1.23e6,
    "negative_exponent": 1.23e-6,
    "positive_large_exponent": 1e30,
    "negative_large_exponent": 1e-30
  },
  "special_cases": {
    "large_integer": 9007199254740991,
    "small_integer": -9007199254740991,
    "large_float": 1.7976931348623157e308,
    "small_float": 5e-324                  
  }
}  
}
"#;
    
    parser.load(to_parse);
    
    let mut test = parser.parse();
    println!("{}", test.to_string());
}
