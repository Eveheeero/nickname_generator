mod crawl;
pub(crate) mod data_collector;
pub(crate) mod generator;
mod generator_main;
mod init;
pub(crate) mod prelude;
mod reform;
mod tui;
use clap::Command;

#[tokio::main]
async fn main() {
    let arg = clap::Command::new("Nickname Generator")
        .about("Nickname Generator")
        .subcommand(Command::new("init").about("Init Api Key"))
        .subcommand(Command::new("crawl").about("Crawl Dictionary"))
        .subcommand(Command::new("reform").about("Reform Data"))
        .subcommand(
            Command::new("generator")
                .about("Nickname Generator")
                .allow_external_subcommands(true),
        )
        .subcommand_required(false)
        .get_matches();

    match arg.subcommand() {
        Some(("init", _)) => init::main().await,
        Some(("crawl", _)) => crawl::main().await,
        Some(("reform", _)) => reform::reform().await,
        Some(("generator", extra)) => {
            if extra.subcommand().is_none() {
                generator_main::main().await;
            } else {
                let mut args = Vec::new();
                args.push(extra.subcommand().unwrap().0.to_string());
                let extra = extra.subcommand().unwrap().1;
                args.append(
                    &mut extra
                        .get_raw("")
                        .unwrap_or_default()
                        .into_iter()
                        .map(|x| x.to_str().unwrap())
                        .map(ToOwned::to_owned)
                        .collect::<Vec<_>>(),
                );
                generator_main::generate(args).await;
            }
        }
        _ => tui::main().unwrap(),
    }
}
