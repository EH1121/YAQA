use structopt::StructOpt;

use crate::files;

#[derive(StructOpt)]
enum Command {
    Play {
        #[structopt(short)]
        topic: Option<String>
    },
    Leaderboards {
        topic: String
    }
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
                // let topics = files::load_topics(option.verbose)?;
                // let leaderboards = files::load_leaderboards(option.verbose)?;
                Ok(())
            },
            Command::Leaderboards { topic } => {
                Ok(())
            }
        }
    }
}