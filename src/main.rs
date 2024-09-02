mod crawl;
pub(crate) mod data_collector;
mod init;
pub(crate) mod prelude;
mod tui;
use clap::Command;

#[tokio::main]
async fn main() {
    crate::prelude::init();
    let arg = clap::Command::new("Nickname Generator")
        .about("Nickname Generator")
        .subcommand(Command::new("init").about("Init Api Key"))
        .subcommand(Command::new("crawl").about("Crawl Dictionary"))
        .subcommand_required(false)
        .get_matches();

    match arg.subcommand() {
        Some(("init", _)) => init::main().await,
        Some(("crawl", _)) => crawl::main().await,
        _ => tui::main().unwrap(),
    }
}
