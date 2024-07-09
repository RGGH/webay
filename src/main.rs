use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Class;

//const URL: &str = "https://www.ebay.co.uk/sch/i.html?p4432023.m570.l1313&_nkw=power+farming&_sacat=0";
const URL: &str =
    "https://www.ebay.co.uk/sch/i.html?p2334524.m570.l1311&_nkw=bamboo+cutlery+organiser";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let resp = client
        .get(URL)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .send()?
        .text()?;

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
            .and_then(|l| l.attr("href"));
        let price = listing
            .find(Class("s-item__price"))
            .next()
            .map(|p| p.text());
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

        println!("Title: {:?}", title);
        println!("URL: {:?}", url);
        println!("Price: {:?}", price);
        println!("Details: {:?}", details);
        println!("Seller: {:?}", seller);
        println!("Shipping: {:?}", shipping);
        println!("Location: {:?}", location);
        println!("Sold: {:?}", sold);

        println!("{:=^50}", "");
    }
    println!("Total results found: {}", count);

    Ok(())
}
