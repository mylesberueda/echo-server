use crate::api;

#[derive(clap::Args)]
pub(crate) struct Arguments {
    #[clap(subcommand)]
    command: Option<Commands>,
    args: Option<String>,
}

#[derive(clap::Subcommand)]
pub(crate) enum Commands {
    Start {
        host: Option<String>,
        port: Option<String>,
    },
}

pub(crate) async fn run(args: &Arguments) -> crate::Result<()> {
    if let Some(commands) = &args.command {
        match commands {
            Commands::Start { host, port } => api::server::Server::new(host, port).await,
        }
    } else {
        Ok(())
    }
}
