#[derive(Debug)]
pub struct Leaderboard {
    topic_name: String,
    player_name: String,
    score: f64,
    start_time: String,
    end_time: String,
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

    pub fn add_new_leaderboards(&mut self, topic_name: &str, player_name: &str, score: f64, start_time: String, end_time: String, duration: u64) {
        self.list.push(Leaderboard{
            topic_name: topic_name.to_lowercase(), 
            player_name: player_name.to_lowercase(), 
            score, 
            start_time,
            end_time,
            duration 
        })
    }

    pub fn get_leaderboards_by_name(&mut self, topic_name: &str) -> Option<Vec<&Leaderboard>>{
        let mut x: Vec<&Leaderboard> = self.list.iter().filter(|v| v.topic_name.to_lowercase() == topic_name.to_lowercase()).collect();
        if !x.is_empty(){
            x.sort_by(|a, b| b.score.total_cmp(&a.score));
            return Some(x);
        }
        None
    }

    pub fn print_leaderboard_by_topic(&mut self, topic_name: &str){
        match self.get_leaderboards_by_name(topic_name){
            Some(t) => {
                for (idx, i) in t.iter().enumerate() {
                    println!("{}. {} | {} | {} | {} | {}", idx + 1, i.start_time, i.end_time, i.duration, i.player_name, i.score);
                }
            },
            None => {
                println!("No leaderboards for {topic_name} found")
            },
        }
    }

}