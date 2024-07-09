use reqwest::Client;
use select::document::Document;
use select::predicate::Class;

use crate::model::Listing;

const URL: &str =
    "https://www.ebay.co.uk/sch/i.html?p2334524.m570.l1311&_nkw=bamboo+cutlery+organiser";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36";

pub async fn scrape() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let resp = client
        .get(URL)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .send()
        .await?
        .text()
        .await?;

    let doc = Document::from(resp.as_str());

    let mut count = 0;

    for listing in doc.find(Class("s-item")) {
        count += 1;

        let title = listing
            .find(Class("s-item__title"))
            .next()
            .map(|t| t.text());
        let url = listing
            .find(Class("s-item__link"))
            .next()
            .and_then(|l| l.attr("href").map(String::from));
        let price = listing
            .find(Class("s-item__price"))
            .next()
            .map(|p| {
                let price_str = p.text();
                let price_f32 = price_str
                    .replace("£", "") // Replace with appropriate currency symbol
                    .replace(",", "") // Remove thousand separators if present
                    .trim() // Trim any extra whitespace
                    .parse::<f32>() // Parse the string into f32
                    .unwrap_or(0.0); // Default to 0.0 if parsing fails
                price_f32
            })
            .unwrap_or(0.0); // Default to 0.0 if price element not found

        let details = listing
            .find(Class("s-item__subtitle"))
            .next()
            .map(|d| d.text());
        let seller = listing
            .find(Class("s-item__seller-info"))
            .next()
            .map(|s| s.text());
        let shipping = listing
            .find(Class("s-item__shipping"))
            .next()
            .map(|s| s.text());
        let location = listing
            .find(Class("s-item__location"))
            .next()
            .map(|l| l.text());
        let sold = listing
            .find(Class("s-item__quantity-sold"))
            .next()
            .map(|s| s.text());

        let listing = Listing {
            title: title.unwrap_or_else(|| "N/A".to_string()),
            url: url.unwrap_or_else(|| "N/A".to_string()),
            price,
            details: details.unwrap_or_else(|| "N/A".to_string()),
            seller: seller.unwrap_or_else(|| "N/A".to_string()),
            shipping: shipping.unwrap_or_else(|| "N/A".to_string()),
            location: location.unwrap_or_else(|| "N/A".to_string()),
            sold: sold.is_some(), // Example: Use a boolean condition if sold is present
        };

        println!("Title: {:?}", listing.title);
        println!("URL: {:?}", listing.url);
        println!("Price: {:?}", listing.price);
        println!("Details: {:?}", listing.details);
        println!("Seller: {:?}", listing.seller);
        println!("Shipping: {:?}", listing.shipping);
        println!("Location: {:?}", listing.location);
        println!("Sold: {:?}", listing.sold);

        println!("{:=^50}", "");
    }
    println!("Total results found: {}", count);

    Ok(())
}
