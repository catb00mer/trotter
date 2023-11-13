use clap::Parser;
use std::{path::PathBuf, process::ExitCode, time::Duration};
use trotter::{
    error::ResponseErr,
    parse::{Gemtext, Symbol},
    Actor, UserAgent,
};

#[derive(thiserror::Error, Debug)]
enum TrotErr {
    #[error("{0}")]
    ActorErr(#[from] trotter::error::ActorError),

    #[error("{0}")]
    Response(#[from] trotter::error::ResponseErr),

    #[error("Expected one of these: archiver, indexer, researcher, webproxy")]
    BadUserAgent,
}

/// 🎠 Trot: A command-line gemini client. Non-success statuses are included in the exit code.
#[derive(Parser)]
struct Cli {
    url: String,

    #[clap(short, long)]
    input: Option<String>,

    #[clap(short, long)]
    cert: Option<PathBuf>,

    #[clap(short, long)]
    key: Option<PathBuf>,

    /// archiver, indexer, researcher, webproxy
    #[clap(short, long)]
    user_agent: Option<String>,

    /// Adjust timeout in seconds (default 5)
    #[clap(short, long)]
    timeout: Option<u64>,

    /// Write output to file
    #[clap(short, long)]
    output: Option<String>,

    /// Only allow gemtext responses. Has no effect when using --output
    #[clap(short, long)]
    gemtext_only: bool,

    /// Print pretty gemtext responses.
    #[clap(short, long)]
    pretty_print: bool,
}

async fn run() -> Result<(), TrotErr> {
    let Cli {
        url,
        input,
        cert,
        key,
        output,
        user_agent,
        timeout,
        gemtext_only,
        pretty_print,
    } = Cli::parse();

    let mut actor = Actor {
        cert,
        key,
        ..Default::default()
    };

    // Set user agent
    if let Some(u) = user_agent {
        actor.user_agent = Some(match u.as_str() {
            "archiver" => UserAgent::Archiver,
            "indexer" => UserAgent::Indexer,
            "researcher" => UserAgent::Researcher,
            "webproxy" => UserAgent::Webproxy,
            _ => return Err(TrotErr::BadUserAgent),
        })
    };

    // Set timeout
    if let Some(t) = timeout {
        actor.timeout = Duration::from_secs(t);
    }

    // Get response
    let response = if let Some(input) = input {
        actor.input(url, input).await?
    } else {
        actor.get(url).await?
    };

    // Save or output
    if let Some(output) = output {
        response.save_to_path(output)?;
        return Ok(());
    }

    let text = if gemtext_only {
        response.gemtext()?
    } else {
        response.text()?
    };

    // Pretty print
    if pretty_print && response.is_gemtext() {
        for g in Gemtext::parse(&text).inner() {
            match g {
                Symbol::Text(a) => print!("{a}"),
                Symbol::Link(a, b) => print!("\x1b[0;4m{b}\x1b[0m \x1b[2m{a}"),
                Symbol::List(a) => print!("• {a}"),
                Symbol::Quote(a) => print!("\x1b[33;3;1m« {a} »"),
                Symbol::Header1(a) => print!("\x1b[32;1m▍ {a}"),
                Symbol::Header2(a) => print!("\x1b[36;1m▋ {a}"),
                Symbol::Header3(a) => print!("\x1b[34;1m█ {a}"),
                Symbol::Codeblock(a, b) => print!("\x1b[35;2m{a}\x1b[0m\n\x1b[35m{b}"),
            }
            println!("\x1b[0m");
        }
    } else {
        println!("{text}");
    }
    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Err(e) => match e {
            TrotErr::Response(ResponseErr::UnexpectedStatus(_, status, meta)) => {
                println!("{meta}");
                ExitCode::from(status.value())
            }
            _ => {
                eprintln!("🎠 Trot error :: {e}");
                ExitCode::from(1)
            }
        },
        Ok(_) => ExitCode::from(0),
    }
}
