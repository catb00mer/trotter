use clap::Parser;
use trotter::Actor;

#[derive(Parser)]
struct Cli {
    url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Cli { url } = Cli::parse();

    println!(
        "{}",
        Actor::default()
            .cert_file("demo.crt")
            .key_file("demo.key")
            .get(url)
            .await?
            .gemtext()?
    );

    Ok(())
}
