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
    "positive_exponent": 1.23e4,
    "negative_exponent": 1.23e-4,
    "positive_large_exponent": 1e30,
    "negative_large_exponent": 1e-30
  },
  "mixed_numbers": {
    "positive_int_in_float_context": 456.0,
    "negative_int_in_float_context": -456.0,
    "small_positive_float": 0.00123,
    "small_negative_float": -0.00123
  },
  "special_cases": {
    "large_integer": 9007199254740991,   // Max safe integer in JavaScript
    "small_integer": -9007199254740991,  // Min safe integer in JavaScript
    "large_float": 1.7976931348623157e308, // Max double precision floating-point number
    "small_float": 5e-324                  // Min positive subnormal double-precision number
  },
  "non_number_elements": {
    "string": "test",
    "boolean_true": true,
    "boolean_false": false,
    "null_value": null,
    "array": [1, 2.2, -3.3e10, "four", true, null],
    "nested_object": {
      "inner_key": 12345
    }
  }
}
"#;
    
    parser.load(r#"{"a":-3}"#);
    
    let mut test = parser.parse();
    test.set_bool("is_working", true);
    test.set_bool("are_you_sure", true);
    test.set_bool("is_working", false);
    test.set_number("number_test", 69.);
    test.set_number("number_test", 420.);
    test.set_string("test_string", "yo");
    test.set_string("test_string", "bruh");
    test.set_array("test_array", vec![JsonType::Bool(true), JsonType::Number(69.)]);
    test.set_array("test_array", vec![JsonType::Bool(false), JsonType::Number(420.)]);
    test.set_null("null_test");
    test.set_array("empty", vec![]);
    test.insert_obj("empty_obj");
    let a = test.get_number("number_test").unwrap();
    
    println!("{}", a);
    println!("{}", test.to_string());
    
    println!("{}", test.get_string("test_string").unwrap());
}
