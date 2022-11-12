#[derive(Debug)]
pub struct Leaderboard {
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

impl Leaderboard{
    pub fn print(&self){
        
    }
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

    pub fn get_leaderboards_by_name(&mut self, topic_name: &str) -> Option<Vec<&Leaderboard>>{
        let mut x: Vec<&Leaderboard> = self.list.iter().filter(|v| v.topic_name == topic_name.to_lowercase()).collect();
        
        if !x.is_empty(){
            x.sort_by(|a, b| b.score.cmp(&a.score));
            return Some(x);
        }
        None
    }

}