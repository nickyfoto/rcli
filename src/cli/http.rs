use std::path::PathBuf;

use clap::Parser;

use crate::{process_http_serve, CmdExecuter};

use super::verify_path;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecuter for HttpServeOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        process_http_serve(self.dir.clone(), self.port).await
    }
}

impl CmdExecuter for HttpSubCommand {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(opts) => opts.execute().await,
        }
    }
}
