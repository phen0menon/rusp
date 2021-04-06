use std::fmt::Debug;
use std::fs;

struct Parser {
    curr_pos: usize,
    text: String,
}

#[derive(Debug)]
enum ObjectType {
    SYMBOL,
    STRING,
    NUMBER,
    BOOLEAN,
    LIST,
}

#[derive(Debug)]
enum ObjectVal {
    ListValue(Vec<Object>),
    IntegerValue(i32),
    StringValue(String),
}

#[derive(Debug)]
struct Object {
    _type: ObjectType,
    _val: Option<ObjectVal>,
}

fn create_object(_type: ObjectType) -> Object {
    return Object { _type, _val: None };
}

fn create_sym_obj(content: &str) -> Object {
    let mut obj = create_object(ObjectType::SYMBOL);
    obj._val = Some(ObjectVal::StringValue(content.to_string()));
    return obj;
}

fn create_bool_obj(value: bool) -> Object {
    let mut obj = create_object(ObjectType::BOOLEAN);
    obj._val = Some(ObjectVal::IntegerValue(value as i32));
    return obj;
}

fn create_num_obj(content: i32) -> Object {
    let mut obj = create_object(ObjectType::NUMBER);
    obj._val = Some(ObjectVal::IntegerValue(content));
    return obj;
}

fn create_str_obj(str: String) -> Object {
    let mut obj = create_object(ObjectType::STRING);
    obj._val = Some(ObjectVal::StringValue(str));
    return obj;
}

fn create_list_obj(content: Vec<Object>) -> Object {
    let mut obj = create_object(ObjectType::LIST);
    obj._val = Some(ObjectVal::ListValue(content));
    return obj;
}

fn parse(parser: &mut Parser) {
    if parser.text.len() == 0 {
        panic!("EOF")
    }

    while has_next_char(parser) {
        let obj = read_expr(parser);
        println!("{:#?}", obj);
    }
}

fn has_next_char(parser: &mut Parser) -> bool {
    return parser.curr_pos < parser.text.len();
}

fn current_char(parser: &mut Parser) -> char {
    return parser
        .text
        .chars()
        .nth(parser.curr_pos)
        .expect("Unexpected EOF");
}

fn skip_char(parser: &mut Parser) {
    parser.curr_pos += 1;
}

fn next_char(parser: &mut Parser) -> char {
    parser.curr_pos += 1;
    return *(&parser
        .text
        .chars()
        .nth(parser.curr_pos)
        .expect("Unexpected EOF"));
}

fn expect_char(parser: &mut Parser, ch: char) -> bool {
    return current_char(parser) == ch;
}

fn read_number(parser: &mut Parser) -> Object {
    let chr = current_char(parser);
    let mut alpha_value = String::from("");
    while has_next_char(parser) && chr.is_numeric() {
        alpha_value += &chr.to_string();
    }
    let numeric_value: i32 = alpha_value.parse().expect("SyntaxError: not a number");
    return create_num_obj(numeric_value);
}

fn read_list(parser: &mut Parser) -> Object {
    let mut list: Vec<Object> = vec![];
    skip_char(parser);
    while !expect_char(parser, ')') {
        if !has_next_char(parser) {
            panic!("SyntaxError: Unexpected EOF");
        }
        let parsed_expr = read_expr(parser);
        list.push(parsed_expr);
    }
    skip_char(parser);
    return create_list_obj(list);
}

fn is_symbol(chr: char) -> bool {
    return chr.is_alphanumeric() || chr == '+' || chr == '-' || chr == '/' || chr == '*';
}

fn read_symbol(parser: &mut Parser) -> Object {
    let mut symbol = String::from("");
    let mut chr = current_char(parser);
    while has_next_char(parser) && is_symbol(chr) {
        symbol += &chr.to_string();
        chr = next_char(parser);
    }
    return create_sym_obj(&symbol);
}

fn read_str(parser: &mut Parser) -> Object {
    skip_char(parser);
    let mut str = String::from("");
    let mut chr = current_char(parser);
    while !expect_char(parser, '"') {
        if !has_next_char(parser) {
            panic!("Unexpected EOF")
        }
        str += &chr.to_string();
        chr = next_char(parser);
    }
    skip_char(parser);
    return create_str_obj(str);
}

fn read_expr(parser: &mut Parser) -> Object {
    let chr = current_char(parser);

    match chr {
        // Parse list
        '(' => {
            return read_list(parser);
        }
        // Remove unnecessary whitespaces and continue reading expr
        ' ' | '\r' | '\n' => {
            skip_char(parser);
            return read_expr(parser);
        }
        // Parse string skipping trailing and leading " symbol
        '"' => {
            return read_str(parser);
        }
        _ => {
            if chr.is_numeric() {
                return read_number(parser);
            }

            if is_symbol(chr) {
                return read_symbol(parser);
            }

            panic!("Invalid symbol: {}", chr)
        }
    }
}

fn get_file_contents(filename: &str) -> String {
    fs::read_to_string(filename).expect("An error occured while reading the file")
}

fn main() {
    let file_contents = get_file_contents("examples/circle_square.lisp");

    let mut parser = Parser {
        curr_pos: 0,
        text: file_contents,
    };

    parse(&mut parser);
}
