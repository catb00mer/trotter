use clap::Parser;
use trotter::{Actor, UserAgent};

#[derive(Parser)]
struct Cli {
    url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Cli { url } = Cli::parse();

    let r = Actor::default()
        .user_agent(UserAgent::Indexer)
        .cert_file("demo.crt")
        .key_file("demo.key")
        .get(url)
        .await?;

    println!("{}", r.gemtext()?);

    Ok(())
}
