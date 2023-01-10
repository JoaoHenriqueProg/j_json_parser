mod json_parser;

fn main() {
    let mut parser = json_parser::Parser::new();
    parser.load(r#"{            "boolean_true":true,"boolean_false"      :false,"integer":123,"float":123.456     ,      "string":"Hello, World!","empty_string":"","whitespace_string":"   ", "null_val"    :  null,"null_val2":null,   "empty_array":[],"array_with_null_values":[null,null,null],"array_with_nested_objects_and_arrays":[[1,2,3],{"a":"apple","b":"banana","c":"cherry"},[{"x":"x-ray","y":"yellow","z":"zebra"},{"foo":"bar"}]],"empty_object":{},"object_with_null_values":{"a":null,"b":null,"c":null},"object_with_nested_objects_and_arrays":{"nested_array":[{"a":1,"b":2,"c":3},{"d":4,"e":5,"f":6}],"nested_object":{"g":7,"h":8,"i":9}},"null_value":null}
"#.to_string());
    let test = parser.parse();
    
    test.print();
    }
