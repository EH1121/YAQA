use std::collections::HashMap;

use rand::Rng;

use crate::quiz::*;

pub struct Topics{
    list: HashMap<u64, Quizzes>
}

/// A List that implements Quizzes
impl Topics{
    pub fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }
    /// 
    pub fn add_new_topic(&mut self, topic_id: u64, topic_name: &str, filename: &str) {
        let x = Quizzes::new(topic_id, topic_name.to_string(), filename.to_string());
        self.list.insert(topic_id, x);
    }
    // TODO: What to add to None? Return?, perhaps add verbose for new data?
    /// Adds new data to a topic
    pub fn add_data_to_topic(&mut self, topic_id: u64, question_id: &str, name: &str, description: &str, answer: &str, choices: &str) {
        let x = match self.list.get(&topic_id){
            Some(mut e) => {
                let mut cloned = e.clone();
                match cloned.raw_add(question_id, name, description, answer, choices){
                    Ok(_) => {
                        self.list.insert(topic_id, cloned);
                    },
                    Err(_) => todo!(),
                }
            },
            None => todo!(),
        };
    }

    /// topic_id: ID of topic in data
    /// 
    /// is_random: Ignores topic_id and randomize topic that is returned
    pub fn get_topic(self, topic_id: u64, is_random: bool) -> Option<Quizzes> {
        if self.list.is_empty() {
            return None;
        }
        let mut t_id = topic_id;
        if is_random{
            let mut rng = rand::thread_rng();
            t_id = rng.gen_range(0..self.list.len()) as u64;
            // let x = t_id = self.list.keys();
            // t_id = 
        }
        Some(self.list.get(&t_id).unwrap().clone())
    }

    /// Prints all topics that currently exists
    pub fn print_all_topics(self) {
        if self.list.is_empty(){
            return;
        }
    }

}


