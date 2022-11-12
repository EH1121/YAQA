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
    /// topic_name: if None, then it will randomize topic, else it returns topic by name
    pub fn get_topic(&self, topic_name: &Option<String>) -> Option<Topic> {
        if self.list.is_empty() {
            return None;
        }

        match topic_name{
            Some(t) => {
                let low_case_topic_name = t.to_lowercase();
                for (k, v) in &self.list{
                    if v.topic_name == low_case_topic_name{
                        return Some(v.clone());
                    }
                }
                return None;
            },
            None => {
                let mut rng = rand::thread_rng();
                let mut t_id = rng.gen_range(0..self.list.len()) as u64;

                let keys: Vec<&u64> = self.list.iter().map(|v| v.0).collect();

                t_id = keys.get(t_id as usize).unwrap().clone().clone();

                if self.list.contains_key(&t_id){
                    return Some(self.list.get(&t_id).unwrap().clone());
                }
                return None;
            }
        }
    }

    /// For input from CLI

    /// Prints all topics that currently exists
    pub fn print_all_topics(&self) {
        if self.list.is_empty() {
            return;
        }
        let mut x = 1;
        for i in &self.list {
            println!("{}. {} | {}", x, i.1.topic_name, i.1.topic_description);
            x += 1;
        }
    }
}


