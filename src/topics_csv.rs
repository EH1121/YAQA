use std::collections::HashMap;
use rand::Rng;

use crate::helpers;

#[derive(Clone)]
pub struct Topic {
    pub topic_id: u64,
    pub leaderboard_name: String,
    pub file_name: String,
    pub topic_name: String,
    pub topic_description: String
}

pub struct Topics { list: HashMap<u64, Topic> }

/// A List that implements Quizzes
impl Topics {
    pub fn new() -> Self { Self{ list: HashMap::new() } }

    /// Inserts a new topic name
    pub fn add_new_topic(&mut self, topic_id: u64, leaderboard_name: &str, filename: &str, topic_name: &str, topic_description: &str) {
        self.list.insert(topic_id, Topic {
            topic_id,
            leaderboard_name: leaderboard_name.to_string(),
            file_name: filename.to_string(),
            topic_name: topic_name.to_string(),
            topic_description: topic_description.to_string()
        });
    }

    /// topic_id: ID of topic in data
    /// 
    /// topic_name: if None, then it will randomize topic, else it returns topic by name
    pub fn get_topic(&self, topic_name: &Option<String>) -> Option<Topic> {
        if self.list.is_empty() { return None };
        match topic_name {
            Some(topic) => {
                let low_case_topic_name = topic.to_lowercase();
                for value in self.list.values() {
                    if value.topic_name == low_case_topic_name {
                        return Some(value.clone());
                    }
                }
                None
            },
            None => {
                let mut rng = rand::thread_rng();
                let mut t_id = rng.gen_range(0..self.list.len()) as u64;
                let keys: Vec<&u64> = self.list.keys().collect();
                t_id = **keys.get(t_id as usize).unwrap();
                if self.list.contains_key(&t_id){
                    return Some(self.list.get(&t_id).unwrap().clone());
                }
                None
            }
        }
    }

    /// Prints all topics that currently exists
    pub fn print_all_topics(&self) {
        if self.list.is_empty() { return };
        let mut x = 1;
        println!("no. |   topic name   | topic description");
        for i in &self.list {
            let k = "   topic name   ".len() - i.1.topic_name.len() - 1;
            println!("{}.  | {}{}| {}", x, i.1.topic_name, helpers::repeat_char(k, ' '), i.1.topic_description);
            x += 1;
        }
    }
}