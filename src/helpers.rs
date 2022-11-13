use std::{
    num, 
    char,
    io::Write
};

/// Converts string to integer, otherwise error
pub fn convert_to_integer(to_parse: &str) -> Result<u64, num::ParseIntError> {
    let integer = to_parse.parse::<u64>()?;
    Ok(integer)
}

/// Converts string to float, otherwise error
pub fn convert_to_float(to_parse: &str) -> Result<f64, num::ParseFloatError> {
    let float = to_parse.parse::<f64>()?;
    Ok(float)
}

/// For input purposes, immediately flush to console
pub fn print(to_print: &str) {
    print!("{}", to_print);
    std::io::stdout().flush().expect("Failed to output line");
}

/// Helper input function
// pub fn get_input_as_integer(line: &str) -> u64 {
//     loop {
//         let mut input = String::new();
//         print(line);
//         while std::io::stdin().read_line(&mut input).is_err() {
//             println!("Failed to input string")
//         }
//         match convert_to_integer(input.trim()) {
//             Ok(e) => {
//                 return e;
//             },
//             Err(_) => println!("Invalid Input, Please try again")
//         }
//     }
// }

/// Converts string to local datetime
// pub fn convert_to_local_datetime(datetime_timezone: &str) -> Result<DateTime<Local>, String> {

//     let fields: Vec<_> = datetime_timezone.split(' ').collect();
//     let local_timezone = match fields.get(2){
//         Some(lt) => Local.(lt.to_string()),
//         None => return Err("Failed to convert datetime to local".to_string()),
//     };


//     let datetime = match fields.get(0){
//         Some(d) => {
//             match fields.get(1){
//                 Some(t) => {
//                     d.to_string().push(' ');
//                     d.to_string().push_str(t);
//                     d
//                 }
//                 None => return Err("Failed to convert datetime to local".to_string()),
//             }
//         }
//         None => return Err("Failed to convert datetime to local".to_string()),
//     };

//     let Naivedt = match datetime.parse::<NaiveDateTime>(){
//         Ok(x) => {
//             return Ok(Local.from_local_datetime(&x).unwrap().with_timezone(local_timezone));
    //             },
//             Err(_) => return Err("Failed to convert datetime to local".to_string()),
//     };

//     // match datetime.parse::<NaiveDateTime>() {
//     //     Ok(x) => {
//       //   return Ok(Local.from_local_datetime(datetime).unwrap().with_timezone(local_timezone));

//         // },
//         // Err(_) => return Err("Failed to convert datetime to local".to_string()),
// }

pub fn repeat_char(times: usize, ch: char) -> String{
    let mut str = String::new();
    for _ in 0..times{
        str.push(ch);
    }
    str
}

/// Splits input string by char, which is then put into a vector of string
pub fn split_str_to_vec(s: &str, ch: char) -> Vec<String> {
    s.split(ch).map(|v| v.to_string()).collect()
}

/// Attempts to convert string to char, takes in a single char, if fail, return ParseCharError
pub fn convert_to_char(to_parse: &str) -> Result<char, char::ParseCharError> {
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
pub fn get_char_input(print_line: &str, from: char, to: char, case_insensitive: bool) -> char {
    loop {
        let mut input = String::new();
        print(print_line);
        while std::io::stdin().read_line(&mut input).is_err() {
            println!("Failed to input string")
        }
        match convert_to_char(input.trim()) {
            Ok(e) => {
                if case_insensitive {
                    let ch = convert_to_char(&e.to_lowercase().to_string()).unwrap();
                    let lowercased_from = convert_to_char(&from.to_lowercase().to_string()).unwrap();
                    let lowercased_to = convert_to_char(&to.to_lowercase().to_string()).unwrap();
                    if ch >= lowercased_from && ch <= lowercased_to {
                        return e;
                    }
                } 
                if e >= from && e <= to {
                    return  e;
                }
            },
            Err(_) => println!("Invalid Input, Please try again")
        }
    }
}

/// Helper function to get string input 
pub fn get_string_input(print_line: &str, min_len: usize, max_len: usize) -> String {
    loop {
        let mut input = String::new();
        print(print_line);
        while std::io::stdin().read_line(&mut input).is_err() {
            println!("Failed to input string")
        }
        if !input.trim().is_empty() && (input.len() >= min_len && input.len() <= max_len) {
            return input.trim().to_string();
        }
    }
}