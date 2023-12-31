use clap::Parser;
use rlc_model::*;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// robot language file
    #[arg(short, long)]
    file: String,
    /// verbose level
    #[arg(short, long, default_value_t = 1)]
    verbose: u8,
}

fn main() {
    let args = Args::parse();
    if args.verbose > 0 {
        //
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", "info")
        }
        env_logger::init();
    }
    //
    if let Ok(model) = load_model(&args.file) {
        if args.verbose >= 3 {
            println!("\n--------------------------------------------------\n");
            println!("{}", model.rl_model);
        }
        if args.verbose >= 2 {
            println!("\n--------------------------------------------------\n");
            println!("{}", model);
        }
    }
}
