use structopt::StructOpt;

use crate::files::{self, load_topics};
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

                // If topic isn't given an argument:
                // 1. List all Topics
                // 2. Wait for input
                // 3. On input, go to get_topic
                // 4. If get_topic does not find the related topic, end
                // 5. Else proceed to play

                let topics = files::load_topics(option.verbose)?;
                let mut leaderboards = files::load_leaderboards(option.verbose)?;

                if topic == None{
                    
                }


                Ok(())
            },
            Command::Leaderboards { topic } => {
                let mut leaderboards = files::load_leaderboards(option.verbose)?;
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
                let topics = files::load_topics(option.verbose)?;
                // Lists all topics

                topics.print_all_topics();

                Ok(())
            }
        }
    }
}