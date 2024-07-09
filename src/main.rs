use scrape::scrape;
mod scrape;
fn main()->Result<(),Box<dyn std::error::Error>>{
    let _ = scrape();
    Ok(())
}
