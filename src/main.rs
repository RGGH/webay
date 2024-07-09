use tokio;
mod db;
mod scrape;
mod model;

#[tokio::main]
async fn main() {
    match db::db_new().await {
        Ok(db) => {
            println!("Database initialized successfully.");

            match scrape::scrape().await {
                Ok(listings) => {
                    for listing in listings {
                        match db::insert_listing(&db, &listing).await {
                            Ok(_) => println!("Inserted listing: {:?}", listing),
                            Err(e) => eprintln!("Error inserting listing: {}. Listing: {:?}", e, listing),
                        }
                    }
                }
                Err(e) => eprintln!("Error scraping data: {}", e),
            }
        }
        Err(e) => eprintln!("Error initializing database: {}", e),
    }
}

