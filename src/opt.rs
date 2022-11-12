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

                // Should topic be left empty, then randomize topic
                let topics = files::load_topics(option.verbose)?;
                let mut leaderboards = files::load_leaderboards(option.verbose)?;

                let top = match topic{
                    Some(_) => topics.get_topic_by_name(&topic.unwrap()),
                    None => topics.get_topic(0, true),
                };

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