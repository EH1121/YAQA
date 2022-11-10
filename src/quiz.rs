use {
    core::num,
    std::collections::HashMap,
    chrono,
    rand::Rng
};

use crate::helpers::{
    convert_to_integer, 
    get_input_as_integer, 
    split_str_to_vec
};

#[derive(Clone)]
struct Quiz {
    id: u64, // Starts from 0
    name: String, // example: "Rust lang", "Mathematics"
    description: String, // This is a description of a question ......
    answer: String, // "a" / "b" / "c" / "d"
    choices: Vec<String>, // Vectors of multiple choices
    asked: bool // to prevent duplicates
}

struct Quizzes {
    list: HashMap<u64, Quiz>, 
    correct: u64, // Number of questions answered correctly
}

impl Quizzes {
    /// Add quiz to list
    fn add(&mut self, id: u64, name: String, description: String, answer: String, choices: Vec<String>) {
        let x = Quiz{
            id,
            name,
            description,
            answer,
            choices,
            asked: false
        };
        self.list.insert(id, x);
    }

    /// Returns a random, yet to be asked question
    fn get_unasked_question(&self) -> Option<Quiz> {
        let x: Vec<_> = self.list.iter().filter(|v| !v.1.asked).map(|v| v.1.clone()).collect();
        if !x.is_empty() {
            let rng = rand::thread_rng().gen_range(0..x.len() - 1);
            return Some(x.get(rng).unwrap().clone());
        }
        None
    }

    /// Prints question, first the question's name, then the description (detail), then provide the multiple choices
    fn print_pertanyaan(&self, quiz: &Quiz) {
        println!("{}", quiz.name);
        println!("{}", quiz.description);
        for i in &quiz.choices{
            println!("{}", i);
        }
    }

    /// Uses get_unasked_question to get random questions which it then outputs
    ///
    /// If an unasked question is found, it will mark it as an asked question
    /// 
    /// Uses print_pertanyaan to output the question
    fn ask(&mut self) {
        for i in 0..5{
            let question = match self.get_unasked_question() {
                Some(mut q) => {
                    q.asked = true;
                    self.list.insert(q.id, q.clone());
                    q
                },
                None => {
                    println!("Tidak ada pertanyaan lagi yang dapat ditanyakan");
                    return;
                },
            };
            self.print_pertanyaan(&question);
        }
    }
}