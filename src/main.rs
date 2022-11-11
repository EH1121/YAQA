use quiz::Quizzes;
use structopt::StructOpt;

mod files;
mod opt;
mod quiz;
mod helpers;
mod answers;
mod leaderboards;
mod topics_csv;

fn main() {
    let args = opt::Opt::from_args();
    // if let Err(e) =  {
    //     println!("Failed to start: {}", e);
    // }
    opt::Opt::run(args)
}
