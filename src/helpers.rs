use std::{
    num::ParseIntError, 
    io::{self, Write}, 
    char::ParseCharError
};

/// Converts string to integer, otherwise error
pub fn convert_to_integer(to_parse: &str) -> Result<u64, ParseIntError> {
    let integer = to_parse.parse::<u64>()?;
    Ok(integer)
}

/// For input purposes, immediately flush to console
pub fn print(to_print: &str){
    print!("{}", to_print);
    io::stdout().flush().expect("Failed to output line");
}

/// Helper input function
pub fn get_input_as_integer(line: &str) -> u64 {
    loop {
        let mut input = String::new();
        print(line);
        while io::stdin().read_line(&mut input).is_err(){
            println!("Failed to input string")
        }
        match convert_to_integer(input.trim()){
            Ok(e) => {
                return e;
            },
            Err(_) => println!("Invalid Input, Please try again")
        }
    }
}

/// Splits input string by char, which is then put into a vector of string
pub fn split_str_to_vec(s: &str, ch: char) -> Vec<String>{
    s.split(ch).map(|v| v.to_string()).collect()
}   


/// Attempts to convert string to char, takes in a single char, if fail, return ParseCharError
pub fn convert_to_char(to_parse: &str) -> Result<char, ParseCharError> {
    let ch = to_parse.parse::<char>()?;
    Ok(ch)
}


/// Get a char input from char A to char B
/// 
/// print_line: Text for what to input, example: Input [A, B, C, D]: 
/// 
/// from: Get char from 'A'
/// 
/// to: Get char up to 'Z'
pub fn get_char_input(print_line: &str, from: char, to: char) -> char {
    loop {
        let mut input = String::new();
        print(print_line);
        while io::stdin().read_line(&mut input).is_err(){
            println!("Failed to input string")
        }
        match convert_to_char(input.trim()){
            Ok(e) => {
                if e as u8 >= from as u8 && e as u8 <= to as u8 {
                    return e;
                }
            },
            Err(_) => println!("Invalid Input, Please try again")
        }
    }
}
