use std::{
    path::PathBuf, 
    fs::File, 
    io::Read
};

use crate::{
    topics_csv::{self, Topic}, 
    leaderboards, 
    helpers::{self, split_str_to_vec},
    quiz::Quizzes, 
    answers::{Choices, to_choices_enum}
};

// Topics: Read from file - DONE
/// Parses input from string and adds it to Topics
fn parse_topic(string: &str) -> Result<(u64, String, String, String, String), String> {
    let strings: Vec<&str> = string.trim_end().split(',').collect();

    let topic_id = match strings.first() {
        Some(str) => match helpers::convert_to_integer(str) {
            Ok(v) => v,
            Err(_) => return Err("ParseIntError".to_string())
        },
        None => return Err("Missing topic_id".to_string())
    };

    let leaderboard_name = match strings.get(1).filter(|str| !str.is_empty()) {
        Some(str) => str.to_string(),
        None => return Err("Missing leaderboard_name".to_string())
    };

    let file_name = match strings.get(2).filter(|str| !str.is_empty()) {
        Some(str) => str.to_string(),
        None => return Err("Missing file_name".to_string())
    };

    let topic_name = match strings.get(3).filter(|str| !str.is_empty()) {
        Some(str) => str.to_string(),
        None => return Err("Missing topic_name".to_string())
    };

    let topic_description = match strings.get(4).filter(|str| !str.is_empty()) {
        Some(str) => str.to_string(),
        None => return Err("Missing topic_description".to_string())
    };
    Ok(( topic_id, leaderboard_name, file_name, topic_name, topic_description ))
}


/// Parses input from string and adds it to Topics
fn parse_topics(topics: String, verbose: bool) -> topics_csv::Topics {
    let mut tpcs = topics_csv::Topics::new();
    for (i, topic) in topics.split('\n').enumerate() {
        if !topic.is_empty() {
            match parse_topic(topic) {
                Ok(tpc) => tpcs.add_new_topic(tpc.0, &tpc.1, &tpc.2, &tpc.3, &tpc.4),
                Err(err) => if verbose { println!("Error in parsing lines in topics.csv on line {}: {err}", i+1) }
            }
        }
    }
    tpcs
}

/// Reads "topics.csv", which will then be parsed by each line and returns Topics which contains all the topics
pub fn load_topics(verbose: bool) -> std::io::Result<topics_csv::Topics> {
    let path = PathBuf::from("src/data/topics.csv");
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(parse_topics(buffer, verbose))
}

// Quiz: Read from file - DONE
/// Parses input from string and adds it to Topics
fn parse_quiz(string: &str) -> Result<(u64, String, String, Choices, Vec<String>), String> {
    let strings: Vec<&str> = string.trim_end().split(',').collect();

    let quiz_id = match strings.first() {
        Some(str) => match helpers::convert_to_integer(str) {
            Ok(v) => v,
            Err(_) => return Err("ParseIntError".to_string())
        },
        None => return Err("Missing quiz id".to_string())
    };

    let quiz_name = match strings.get(1).filter(|str| !str.is_empty()) {
        Some(str) => str.to_string(),
        None => return Err("Missing quiz name".to_string())
    };

    let quiz_description = match strings.get(2).filter(|str| !str.is_empty()) {
        Some(str) => str.to_string(),
        None => return Err("Missing description".to_string())
    };

    let quiz_answer = match strings.get(3).filter(|str| !str.is_empty()) {
        Some(str) => {
            match to_choices_enum(str){
                Ok(e) => e,
                Err(_) => return Err("Error parsing answer".to_string())
            }
        },
        None => return Err("Missing quiz answer".to_string())
    };

    let quiz_choices = match strings.get(4).filter(|str| !str.is_empty()) {
        Some(str) => split_str_to_vec(str, '|'),
        None => return Err("Missing quiz choices".to_string())
    };
    Ok(( quiz_id, quiz_name, quiz_description, quiz_answer, quiz_choices))
}

pub fn parse_quizzes(quizzes: String, filename: &str, verbose: bool) -> Quizzes { //-> Quizzes{
    let mut qz = Quizzes::new();
    for (i, q) in quizzes.split('\n').enumerate() {
        if !q.is_empty() {
            match parse_quiz(q) {
                Ok(quiz) => qz.add(quiz.0, &quiz.1, &quiz.2, quiz.3, quiz.4),
                Err(err) => if verbose { println!("Error in parsing lines in {filename} on line {}: {err}", i+1) }
            }
        }
    }
    qz
}

pub fn load_quizzes(topic: Topic, verbose: bool) -> std::io::Result<Quizzes> { //-> Quizzes{
    // topic_id: u64,
    // leaderboard_name: String,
    // file_name: String,
    // topic_name: String,
    // topic_description: String
    let path = PathBuf::from("src/data/questions/".to_owned() + &topic.file_name);
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(parse_quizzes(buffer, &topic.file_name, verbose))
}

// Leaderboards: Read and Write
/// Parses leaderboard with format: topic_name, player_name, score, duration
pub fn parse_leaderboard(string: &str) -> Result<(String, String, f64, u64), String> {
    let strings: Vec<&str> = string.trim_end().split(',').collect();

    let topic_name = match strings.first().filter(|str| !str.is_empty()) {
        Some(str) => str.to_string(),
        None => return Err("Missing leaderboard_name".to_string())
    };

    let player_name = match strings.get(1).filter(|str| !str.is_empty()) {
        Some(str) => str.to_string(),
        None => return Err("Missing leaderboard_name".to_string())
    };

    let score = match strings.get(2) {
        Some(str) => match helpers::convert_to_float(str) {
            Ok(v) => v,
            Err(_) => return Err("ParseIntError".to_string())
        },
        None => return Err("Missing topic_id".to_string())
    };

    let duration = match strings.get(3) {
        Some(str) => match helpers::convert_to_integer(str) {
            Ok(v) => v,
            Err(_) => return Err("ParseIntError".to_string())
        },
        None => return Err("Missing topic_id".to_string())
    };
    Ok(( topic_name, player_name, score, duration ))
}

/// Parses input from string and adds it to leaderboards
fn parse_leaderboards(leaderboards: String, verbose: bool) -> leaderboards::Leaderboards {
    let mut ldbs = leaderboards::Leaderboards::new();
    for (i, leaderboard) in leaderboards.split('\n').enumerate() {
        if !leaderboard.is_empty() {
            match parse_leaderboard(leaderboard) {
                Ok(ldb) => ldbs.add_new_leaderboards(&ldb.0, &ldb.1, ldb.2, ldb.3),
                Err(err) => if verbose { println!("Error in parsing lines in leaderboards.csv on line {}: {err}", i+1) }
            }
        }
    }
    ldbs
}

/// Reads "leaderboards.csv", which will then be parsed by each line and returns leaderboards which contains all the players
pub fn load_leaderboards(verbose: bool) -> std::io::Result<leaderboards::Leaderboards> {
    let path = PathBuf::from("src/data/leaderboards.csv");
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(parse_leaderboards(buffer, verbose))
}

// TODO: Write
pub fn write_leaderboards(leaderboards: leaderboards::Leaderboards) {
    todo!();
}