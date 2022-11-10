use structopt::StructOpt;

#[derive(StructOpt)]
enum Command {
    Play {
        #[structopt(short)]
        topic: Option<String>
    },
    Leaderboards {
        // FIXME: topic: data type
    }
}

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    command: Command,
    #[structopt(short, help = "verbose")]
    verbose: bool
}

impl Opt {
    pub fn run(option: Opt) {
        match option.command {
            Command::Play { topic } => {

            },
            Command::Leaderboards {  } => {
                
            }
        }
    }
}