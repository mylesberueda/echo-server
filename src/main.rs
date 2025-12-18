pub(crate) mod api;
mod commands;
use clap::Parser as _;
use commands::*;

type Result<T> = color_eyre::Result<T>;

#[derive(clap::Parser)]
#[clap(name = "Echo Server")]
#[clap(author = "Myles <myles@polypixel.io>")]
#[clap(version = "0.1.0")]
#[clap(about = "A server that responds back to HTTP requests")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
#[command(arg_required_else_help = true)]
enum Commands {
    /// Server command
    Server(server::Arguments),
}

#[tokio::main]
async fn main() -> crate::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    if let Some(cmds) = &cli.command {
        match cmds {
            Commands::Server(args) => server::run(args).await,
        }?;
    };

    Ok(())
}
