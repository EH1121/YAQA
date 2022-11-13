

use chrono::Local;
use structopt::StructOpt;

use crate::files::{
    load_topics, 
    load_quizzes, 
    load_leaderboards, write_leaderboards
};

use crate::helpers::{get_string_input, convert_to_integer};

#[derive(StructOpt)]
enum Command {
    Play {
        #[structopt(short)]
        topic: Option<String>
    },
    Leaderboards {
        topic: String
    },
    List,
}

#[derive(StructOpt)]
#[structopt(name = "YAQA", about = "Yet Another Quiz Apps", author = "Eric & Raymond")]
pub struct Opt {
    #[structopt(subcommand)]
    command: Command,
    #[structopt(short, long, help = "verbose")]
    verbose: bool
}

impl Opt {
    pub fn run(option: Opt) -> Result<(), std::io::Error> {
        match option.command {
            // TODO: P;ay Game and Use Data
            // Topic is optional
            Command::Play { topic } => {
                // Should topic be left empty, then randomize topic
                let play_topic = load_topics(option.verbose)?;
                let mut leaderboards = load_leaderboards(option.verbose)?;

                let top = match play_topic.get_topic(&topic){
                    Some(e) => e,
                    None => {
                        let x = topic.unwrap();
                        println!("Topic {x} Not Found");
                        return Ok(());
                    }
                };

                let mut y = load_quizzes(&top, option.verbose)?;
                
                let start_dt = Local::now();

                y.ask(5);

                let end_dt = Local::now();

                let score: f64 = (y.correct as f64 / y.questions_asked as f64) * 100.0;

                let duration = end_dt - start_dt;
                let int_duration = convert_to_integer(&duration.num_seconds().to_string()).unwrap();

                let top_name = top.topic_name;

                let player_name = get_string_input("Input player name [5 - 20]: ", 5, 20);

                leaderboards.add_new_leaderboards(&top_name, &player_name, score, start_dt.to_string(), end_dt.to_string(), int_duration);

                write_leaderboards(leaderboards);

                Ok(())
            },
            Command::Leaderboards { topic } => {
                let mut leaderboards = load_leaderboards(option.verbose)?;

                leaderboards.print_leaderboard_by_topic(&topic);
                Ok(())
            },
            Command::List{} => {
                let topics = load_topics(option.verbose)?;
                // Lists all topics
                topics.print_all_topics();
                Ok(())
            }
        }
    }
}