use server::run;
use std::error::Error;

mod server;
mod service;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    run().await?;
    Ok(())
}
