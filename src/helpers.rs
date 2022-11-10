use std::{
    num::ParseIntError, 
    io::{self, Write}
};

/// Converts string to integer, otherwise error
pub fn convert_to_integer(to_parse: &str) -> Result<i64, ParseIntError> {
    let integer = to_parse.parse::<i64>()?;
    Ok(integer)
}

/// For input purposes, immediately flush to console
pub fn print(line: &str) {
    print!("{}", line);
    io::stdout().flush(); // TODO: flush?
}

/// Helper input function
pub fn get_input_as_integer(line: &str) -> i64 {
    let mut x = 0;
    loop {
        let mut input = String::new();
        print(line);
        while io::stdin().read_line(&mut input).is_err(){
            println!("Failed to input string")
        }
        match convert_to_integer(input.trim()){
            Ok(e) => {
                x = e;
                break;
            },
            Err(_) => println!("Invalid Input, Please try again")
        }
    }
    x
}

pub fn split_str_to_vec(s: String, ch: char) -> Vec<String> {
    s.split(ch).map(|v| v.to_string()).collect()
}   