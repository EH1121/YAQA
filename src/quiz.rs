use std::collections::HashMap;

struct Quiz{
    id: u64, // Starts from 0
    name: String, // example: "Rust lang", "Mathematics"
    description: String, // This is a description of a question ......
    answer: String, // "a" / "b" / "c" / "d"
    choices: Vec<String>, // Vectors of multiple choices
    asked: bool // to prevent duplicates
}

struct Quizzes{
    list: HashMap<u64, Quiz>, 
    correct: u64, // Number of questions answered correctly
}

impl Quizzes{
    // Add quiz to list
    fn add(&mut self, id: u64, name: String, description: String, answer: String, choices: Vec<String>){

    }

    // Returns a random, yet to be asked question
    fn get_unasked_question(self) {//-> Quiz{

    }

    // Prints question, first the question's name, then the description (detail), then provide the multiple choices
    fn ask(self){

    }
}