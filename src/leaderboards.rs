struct Leaderboard {
    id: u64,
    topic_id: u64,
    name: String,
    score: u64
}

struct Leaderboards {
    list: Vec<Leaderboard>
}