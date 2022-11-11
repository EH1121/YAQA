use std::{
    path::PathBuf, 
    fs::File, 
    io::Read
};

use crate::{
    topics_csv, 
    leaderboards, 
    helpers
};

// Quiz: Read from file - DONE
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

fn parse_topics(topics: String, verbose: bool) -> topics_csv::Topics {
    let mut tpcs = topics_csv::Topics::new();
    for topic in topics.split('\n') {
        if !topic.is_empty() {
            match parse_topic(topic) {
                Ok(tpc) => tpcs.add_new_topic(tpc.0, &tpc.1, &tpc.2, &tpc.3, &tpc.4),
                Err(err) => if verbose { println!("Error in parsing lines in topics.csv: {err}") }
            }
        }
    }
    tpcs
}

pub fn load_topics(verbose: bool) -> std::io::Result<topics_csv::Topics> {
    let path = PathBuf::from("src/data/topics.csv");
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(parse_topics(buffer, verbose))
}

// Leaderboards: Read and Write
pub fn parse_leaderboard(string: &str) -> Result<(String, String, u64, u64), String> {
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
        Some(str) => match helpers::convert_to_integer(str) {
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

fn parse_leaderboards(leaderboards: String, verbose: bool) -> leaderboards::Leaderboards {
    let mut ldbs = leaderboards::Leaderboards::new();
    for leaderboard in leaderboards.split('\n') {
        if !leaderboard.is_empty() {
            match parse_leaderboard(leaderboard) {
                Ok(ldb) => ldbs.add_new_leaderboards(&ldb.0, &ldb.1, ldb.2, ldb.3),
                Err(err) => if verbose { println!("Error in parsing lines in topics.csv: {err}") }
            }
        }
    }
    ldbs
}

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