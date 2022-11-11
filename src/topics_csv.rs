use std::collections::HashMap;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Topic {
    topic_id: u64,
    leaderboard_name: String,
    file_name: String,
    topic_name: String,
    topic_description: String
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

    /// 
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
    pub fn get_topic(self, topic_id: u64, is_random: bool) -> Option<Topic> {
        if self.list.is_empty() {
            return None;
        }
        let mut t_id = topic_id;
        if is_random {
            let mut rng = rand::thread_rng();
            t_id = rng.gen_range(0..self.list.len()) as u64;
            // let x = t_id = self.list.keys();
            // t_id = 
        }
        Some(self.list.get(&t_id).unwrap().clone())
    }

    /// Prints all topics that currently exists
    pub fn print_all_topics(self) {
        if self.list.is_empty() {
            return;
        }
        for i in self.list {
            println!("{}. {}\nDescription: {}", i.1.topic_id, i.1.topic_name, i.1.topic_description);
        }
    }
}


