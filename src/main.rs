use tokio::process::Command;

mod api;
mod cmd;
mod log;
mod menu;
mod pip;
mod read_req;

use cmd::command::{Cmd, PM};

#[tokio::main]
async fn main() {
    match check_uv_installed().await {
        Ok(_) => run_app(PM::UV).await,
        Err(_) => run_app(PM::Pip).await,
    }
}

async fn check_uv_installed() -> Result<(), tokio::io::Error> {
    let _ = Command::new("uv").args(["version"]).output().await?;
    Ok(())
}

async fn run_app(pm: PM) {
    let command = Cmd::new(pm);
    command.execute().await;
}
