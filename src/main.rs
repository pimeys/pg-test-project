use quaint::{pooled::Quaint, prelude::*};

static CONN_STR: &str = "postgres://postgres:prisma@localhost:5432/postgres";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    log::info!("WTF");

    let pool = Quaint::builder(CONN_STR)?.build();
    let conn = pool.check_out().await?;

    let res = conn.query_raw("SELECT 1 AS number", &[]).await?;
    println!("Response object: {:?}", res);

    Ok(())
}
