use db::db_new;
use scrape::scrape;
use tokio;
mod db;
mod model;
mod scrape;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create db and table to match model
    let _ = db_new().await;
    // scrape
    if let Err(e) = scrape().await {
        eprintln!("Error scraping data: {}", e);
    }
    Ok(())
}
