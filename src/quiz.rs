use {
    std::collections::HashMap,
    rand::Rng,
    std::num::ParseIntError
};

use crate::{
    helpers::{
        convert_to_integer, 
        get_input_as_integer, 
        split_str_to_vec, 
        get_char_input
    }, 
    answers::{
        to_choices_enum, 
        Choices
    }
};

#[derive(Clone)]
struct Quiz {
    id: u64, // Starts from 0
    name: String, // example: "Rust lang", "Mathematics"
    description: String, // This is a description of a question ......
    answer: Choices, // "a" / "b" / "c" / "d"
    choices: Vec<String>, // Vectors of multiple choices
    asked: bool // to prevent duplicates
}

struct Quizzes {
    list: HashMap<u64, Quiz>, 
    correct: u64, // Number of questions answered correctly
}

impl Quizzes {
    /// Add quiz to list
    fn add(&mut self, id: u64, name: &str, description: &str, answer: Choices, choices: Vec<String>) {
        let x = Quiz{
            id,
            name: name.to_string(),
            description: description.to_string(),
            answer,
            choices,
            asked: false
        };
        self.list.insert(id, x);
    }

    /// Convert from String to each of the datatypes required to insert as Quiz
    fn raw_add(&mut self, id: &str, name: &str, description: &str, answer: &str, choices: &str) -> Result<(), ParseIntError> {
        let f1 = id.trim().parse::<u64>()?; // u64::from_str_radix(id.trim(), 10)?;
        let f2 = name.trim();
        let f3 = description.trim();
        let f4 = to_choices_enum(answer).unwrap();
        let f5 = split_str_to_vec(choices, '|');
        self.add(f1, f2, f3, f4, f5);
        Ok(())
    }

    // TODO: Cursed double clone, perhaps there is a way to prevent it?
    /// Returns a random, yet to be asked question
    fn get_unasked_question(&self) -> Option<Quiz> {
        let x: Vec<_> = self.list.iter().filter(|v| !v.1.asked).map(|v| v.1).collect();
        if !x.is_empty() {
            let rng = rand::thread_rng().gen_range(0..x.len() - 1);
            return Some(x.get(rng).unwrap().clone().clone());
        }
        None
    }

    /// Prints question, first the question's name, then the description (detail), then provide the multiple choices
    fn print_pertanyaan(&self, quiz: &Quiz, quiz_number: usize) {
        println!("{}", quiz.name);
        println!("{}. {}", quiz_number, quiz.description);
        let mut curr_option: u8 = 65;
        for i in &quiz.choices{
            println!("{}. {}", curr_option as char, i);
            curr_option += 1;
        }
    }

    /// Uses get_unasked_question to get random questions which it then outputs
    ///
    /// If an unasked question is found, it will mark it as an asked question
    /// 
    /// Uses print_pertanyaan to output the question
    fn ask(&mut self) {
        for i in 0..5 {
            let mut question = match self.get_unasked_question() {
                Some(q) => q,
                None => {
                    println!("Tidak ada pertanyaan lagi yang dapat ditanyakan");
                    return;
                },
            };
            self.print_pertanyaan(&question, i + 1);
            let answered = get_char_input("Input ['A' | 'B' | 'C' | 'D']: ", 'A', 'D');
            if to_choices_enum(&answered.to_string()).unwrap() == question.answer{
                self.correct += 1;
                question.asked = true;
                self.list.insert(question.id, question);
            }
        }
    }
}