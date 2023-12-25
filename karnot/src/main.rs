use clap::{Parser, Subcommand};
use karnot;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Init a new App Chain config
    Init,
    /// Lists all the existing App Chain configs
    List,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => karnot::cli::init::init(),
        Some(Commands::List) => karnot::cli::list::list(),
        None => println!("Use --help to see the complete list of available commands"),
    }
}