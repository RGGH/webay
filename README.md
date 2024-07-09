[![Rust](https://github.com/RGGH/webay/actions/workflows/rust.yml/badge.svg)](https://github.com/RGGH/webay/actions/workflows/rust.yml)
# light touch scrape for specific items on Ebay

##  scrape and extract data from eBay listings using Rust, reqwest, and select crates.

```❯ cargo run
   Compiling webay v0.1.0 (/home/rag/Documents/rust/webay)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 23.63s
     Running `target/debug/webay`
Creating database sqlite://sqlite.db
Create db success
Create listings table result: SqliteQueryResult { changes: 0, last_insert_rowid: 0 }
Database initialized successfully.
Title: "Shop on eBay"
URL: "https://ebay.com/itm/123456?itmmeta=012DEW30YG0MEEKND7NH&hash=item123546:g:acwAA9KNiJowH:sc:ShippingMethodStandard!95008!US!-1&itmprp=enc%3AbgepL1tlUHjMGCVfSTGJh%2BzsVKeJ3CQk7NizDI4BZeppuFnmyS6Ijyp8lh%2FnEw%2BWqO7uTV1Q6izE1R0T54aV8j71F4xlWfVcGft4%2FiOQhtqVXA1rW6M1atPARQRmhqUxtEPJKhKtSFgI%2Bvwlzb0GwVCtkp%3ABlBMUObkmabpYw"
Price: 0.0
Details: "Brand New"
Seller: "N/A"
Shipping: "N/A"
Location: "N/A"
Sold: false
==================================================
Title: "Shop on eBay"
URL: "https://ebay.com/itm/123456```
