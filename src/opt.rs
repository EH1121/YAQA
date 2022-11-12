use std::time::Instant;

use chrono::{Local, Duration};
use structopt::StructOpt;

use crate::files::{
    load_topics, 
    load_quizzes, 
    load_leaderboards
};

use crate::leaderboards;

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
            /// Topic is optional
            Command::Play { topic } => {
                // Should topic be left empty, then randomize topic
                let topics = load_topics(option.verbose)?;
                let mut leaderboards = load_leaderboards(option.verbose)?;

                let top = match topics.get_topic(&topic){
                    Some(e) => e,
                    None => {
                        let x = topic.unwrap();
                        println!("Topic {x} Not Found");
                        return Ok(());
                    }
                };

                let mut y = load_quizzes(top, option.verbose)?;
                
                // let chrono_datetime = Local::now().format("%Y-%m-%d %H-%M-%S");

                y.ask();

                // let z = chrono_datetime

                // let chrono_datetime_since = Local::now().format("%Y-%m-%d %H-%M-%S");

                // let duration = chrono_datetime - chrono_datetime_since;

                
                // Leaderboards update?
                let score: f64 = (y.correct as f64 / y.questions_asked as f64) * 100.0;



                Ok(())
            },
            Command::Leaderboards { topic } => {
                let mut leaderboards = load_leaderboards(option.verbose)?;
                match leaderboards.get_leaderboards_by_name(&topic){
                    Some(t) => {
                        for i in t{
                            i.print();
                        }
                    },
                    None => println!("No leaderboards for {topic} found"),
                }
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