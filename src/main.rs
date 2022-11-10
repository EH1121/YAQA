use structopt::StructOpt;

mod opt;
mod quiz;
mod helpers;

fn main() {
    let args = opt::Opt::from_args();
    // if let Err(e) =  {
    //     println!("Failed to start: {}", e);
    // }
    opt::Opt::run(args)
}
