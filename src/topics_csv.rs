use std::collections::HashMap;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Topic {
    pub topic_id: u64,
    pub leaderboard_name: String,
    pub file_name: String,
    pub topic_name: String,
    pub topic_description: String
}

#[derive(Debug)]
pub struct Topics {
    list: HashMap<u64, Topic>
}

/// A List that implements Quizzes
impl Topics {
    pub fn new() -> Self {
        Self{
            list: HashMap::new(),
        }
    }

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
    /// is_random: Ignores topic_id and randomize topic that is returned
    pub fn get_topic(&self, topic_id: u64, is_random: bool) -> Option<Topic> {
        if self.list.is_empty() {
            return None;
        }
        let mut t_id = topic_id;

        if is_random {
            let mut rng = rand::thread_rng();
            t_id = rng.gen_range(0..self.list.len()) as u64;

            let keys: Vec<&u64> = self.list.iter().map(|v| v.0).collect();

            t_id = keys.get(t_id as usize).unwrap().clone().clone();
        }

        if self.list.contains_key(&t_id){
            Some(self.list.get(&t_id).unwrap().clone());
        }
        None
    }

    /// For input from CLI
    pub fn get_topic_by_name(&self, topic_name: &str) -> Option<Topic> {
        if self.list.is_empty() {
            return None;
        }
        let low_case_topic_name = topic_name.to_lowercase();
        for (k, v) in &self.list{
            if v.topic_name == low_case_topic_name{
                return Some(v.clone());
            }
        }
        None
    }

    /// Prints all topics that currently exists
    pub fn print_all_topics(&self) {
        if self.list.is_empty() {
            return;
        }
        for i in &self.list {
            println!("{}. {} | {}", i.1.topic_id, i.1.topic_name, i.1.topic_description);
        }
    }
}


