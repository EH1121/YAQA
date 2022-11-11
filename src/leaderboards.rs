#[derive(Debug)]
struct Leaderboard {
    topic_name: String,
    player_name: String,
    score: u64,
    // start_time: String,
    // end_time: String,
    duration: u64
}

#[derive(Debug)]
pub struct Leaderboards {
    list: Vec<Leaderboard>
}

impl Leaderboards {
    pub fn new() -> Self {
        Self{
            list: Vec::new(),
        }
    }

    pub fn add_new_leaderboards(&mut self, topic_name: &str, player_name: &str, score: u64, duration: u64) {
        self.list.push(Leaderboard{
            topic_name: topic_name.to_string(), 
            player_name: player_name.to_string(), 
            score, 
            duration 
        })
    }
}