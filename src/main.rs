use scrape::scrape;
mod model;
mod scrape;

fn main()->Result<(),Box<dyn std::error::Error>>{
    let _ = scrape();
    Ok(())
}
