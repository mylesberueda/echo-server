use crate::api;

#[derive(clap::Args)]
#[command(arg_required_else_help = true)]
pub(crate) struct Arguments {
    #[clap(subcommand)]
    command: Option<Commands>,
    args: Option<String>,
}

#[derive(clap::Subcommand)]
pub(crate) enum Commands {
    /// Start the server
    Start {
        host: Option<String>,
        port: Option<String>,
    },
}

pub(crate) async fn run(args: &Arguments) -> crate::Result<()> {
    if let Some(commands) = &args.command {
        match commands {
            Commands::Start { host, port } => {
                let _server = api::server::Server::new(host, port).await?;
                Ok(())
            }
        }
    } else {
        Err(color_eyre::eyre::eyre!("Arg required"))
    }
}
