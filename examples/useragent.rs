use trotter::{Actor, UserAgent};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Actor::default()
        .user_agent(UserAgent::Indexer)
        .get("catboomer.net") // <- Points at a site that denies all robot traffic
        .await?;

    Ok(())
}
