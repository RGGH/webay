use std::io;
use std::io::Write;
use std::process::{Command, Stdio};
use tokio;
mod db;
mod model;
mod scrape;

#[tokio::main]
async fn main() {
    // Prompt the user to input the URL
    let mut url = String::new();
    print!("Please enter the URL to scrape: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut url).unwrap();
    let url = url.trim();

    match db::db_new().await {
        Ok(db) => {
            println!("Database initialized successfully.");

            match scrape::scrape(url).await {
                Ok(listings) => {
                    for listing in listings {
                        match db::insert_listing(&db, &listing).await {
                            Ok(_) => println!("Inserted listing: {:?}", listing),
                            Err(e) => {
                                eprintln!("Error inserting listing: {}. Listing: {:?}", e, listing)
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Error scraping data: {}", e),
            }
        }
        Err(e) => eprintln!("Error initializing database: {}", e),
    }

    // Define the Python interpreter and the script path
    let python_interpreter = "python3";
    let script_path = "make_plot.py";

    // Execute the Python script
    let _ = Command::new(python_interpreter)
        .arg(script_path)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute Python script")
        .wait_with_output()
        .expect("Failed to wait on Python script");
}


