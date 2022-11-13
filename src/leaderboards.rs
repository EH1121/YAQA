use crate::helpers;

pub struct Leaderboard {
    pub topic_name: String,
    pub player_name: String,
    pub score: f64,
    pub start_time: String,
    pub end_time: String,
    pub duration: u64
}

pub struct Leaderboards {
    list: Vec<Leaderboard>
}

impl Leaderboards {
    pub fn new() -> Self {
        Self{
            list: Vec::new(),
        }
    }

    pub fn leaderboards_as_vec(&self) -> Vec<&Leaderboard> {
        self.list.iter().collect()
    }

    pub fn add_new_leaderboards(&mut self, topic_name: &str, player_name: &str, score: f64, start_time: String, end_time: String, duration: u64) {
        self.list.push( Leaderboard {
            topic_name: topic_name.to_lowercase(), 
            player_name: player_name.to_lowercase(), 
            score, 
            start_time,
            end_time,
            duration 
        })
    }

    pub fn get_leaderboards_by_name(&mut self, topic_name: &str) -> Option<Vec<&Leaderboard>> {
        let mut x: Vec<&Leaderboard> = self.list.iter().filter(|v| v.topic_name.to_lowercase().eq(&topic_name.to_lowercase())).collect();
        if !x.is_empty() {
            x.sort_by(|a, b| b.score.total_cmp(&a.score));
            return Some(x);
        }
        None
    }

    pub fn print_leaderboard_by_topic(&mut self, topic_name: &str) {
        match self.get_leaderboards_by_name(topic_name) {
            Some(t) => {
                println!("------------------------------------------------------------------------------------------------------------------------------");
                println!("no. |              start time              |                 end time             | duration |     player name     |   score");
                println!("------------------------------------------------------------------------------------------------------------------------------");
                for (idx, i) in t.iter().enumerate() {
                    let x = "     player name     ".len() - i.player_name.len() - 1;
                    let y = " duration ".len() - i.duration.to_string().len() - 1;
                    println!("{}.  | {} | {} | {}{}| {}{}| {}", idx + 1, i.start_time, i.end_time, i.duration, helpers::repeat_char(y, ' '), i.player_name, helpers::repeat_char(x, ' '), i.score);
                    println!("------------------------------------------------------------------------------------------------------------------------------");
                }
            },
            None => println!("No leaderboards for {topic_name} found")
        }
    }
}