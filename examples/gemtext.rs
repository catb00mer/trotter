use clap::Parser;

#[derive(Parser)]
struct Cli {
    url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Cli { url } = Cli::parse();

    println!("{}", trotter::trot(url).await?.gemtext()?);

    Ok(())
}
