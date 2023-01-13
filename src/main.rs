use crate::json_parser::JsonError;

mod json_parser;

fn main() {
    let mut parser = json_parser::Parser::new();
    parser.load(r#"{            "boolean_true":true,"boolean_false" : false ,"array_1":[1,2,3],"integer":123,"float":123.456     ,      "string":"Hello, World!","empty_string":"","whitespace_string":"   ", "null_val"    :  null,"null_val2":null,   "empty_array":[],"array_with_null_values":[null,null,null],"array_with_nested_objects_and_arrays":[[1,2,3],{"a":"apple","b":"banana","c":"cherry"},[{"x":"x-ray","y":"yellow","z":"zebra"},{"foo":"bar"}]],"empty_object":{},"object_with_null_values":{"a":null,"b":null,"c":null},"object_with_nested_objects_and_arrays":{"nested_array":[{"a":1,"b":2,"c":3},{"d":4,"e":5,"f":6}],"nested_object":{"g":7,"h":8,"i":9}},"null_value":null}
"#);
    let test = parser.parse();

    test.print();

    // to proper testing later
    let non_existent_child = test.get("this_key_does_not_exist");
    match non_existent_child {
        Ok(_) => {
            unreachable!();
        }
        Err(err) => match err {
            JsonError::KeyNotFound => {}
            _ => {
                unreachable!()
            }
        },
    }

    println!("=-=");
    println!("{}", test.stringify());
    println!("=-=");
    println!("{}", test.get_bool("boolean_true").unwrap());
    println!("{}", test.get_number("float").unwrap());
    println!("{}", test.get_string("string").unwrap());

    match test.get_number("string") {
        Ok(_) => unreachable!(),
        Err(err) => match err {
            JsonError::WrongTypeValueRequest => {}
            _ => {
                unreachable!()
            }
        },
    }

    let array1 = test.get_array("array_1");

    match array1 {
        Ok(val) => {
            println!("{:#?}", val[0]);
            println!("{:#?}", val[1]);
            println!("{:#?}", val[2]);
        },
        Err(_) => unreachable!(),
    }
}
