use clap::Parser;
use std::{process::ExitCode, time::Duration};
use trotter::{error::ResponseErr, Actor, UserAgent};

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
    cert: Option<String>,

    #[clap(short, long)]
    key: Option<String>,

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
}

async fn run() -> Result<(), TrotErr> {
    let Cli {
        url,
        cert,
        key,
        output,
        user_agent,
        timeout,
        gemtext_only,
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
    let response = actor.get(url).await?;

    // Save or output
    if let Some(output) = output {
        response.save_to_path(output)?;
    } else if gemtext_only {
        println!("{}", response.gemtext()?);
    } else {
        println!("{}", response.text()?);
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
