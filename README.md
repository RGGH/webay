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
Title: "Bamboo Cutlery Tray Expandable Drawer Organiser Storage Compartment Utensil"
URL: "https://www.ebay.co.uk/itm/387099361921?itmmeta=01J2C2BGE60EX341565342X30N&hash=item5a20eb8281:g:B10AAOSwbMBmcdtG&itmprp=enc%3AAQAJAAAA0AN0sBHpgT74jVY2VvBvK6LxdRYjt2XXEkGULOXhcb6%2BhLgDw0L%2Fr%2F3jrqGeZFcp9%2FjLmaTjQ3ralGm1rdOeTEUCxK7Ih5D0YRB%2B0DDjvJjRROQ9%2F4K9JDF%2Fi7VOoEyiAJD7FXfo%2BeWYEF2UG%2FZwAPF8NCpbSKrKT1cOFk0e7CA6TcIykZayCh65UIDDyajEmq2bmPmCCd6j3PSzpzi1jB1nZfMHbWO8%2FRlaRZ5xX2pmJa8abtQJWflSW7SBZ02J4q%2Byef9mTyXVCF5emyYQIRw%3D%7Ctkp%3ABk9SR5iHroKTZA"
Price: 12.95
Details: "Brand new"
Seller: "tthaco (3,866) 99.9%"
Shipping: "N/A"
Location: "N/A"
Sold: false
==================================================
Title: "Extendable Bamboo Cutlery Tray Kitchen Drawer Organiser 7-8 Storage Compartments"
URL: "https://www.ebay.co.uk/itm/395475507170?epid=3034916341&itmmeta=01J2C2BGE6AXJKXTKD856D1H2T&hash=item5c142d57e2:g:97wAAOSwhFhmb~W0&itmprp=enc%3AAQAJAAAA0IjBtKJ8I7fFbS7FK%2FOnkfSPVkdcA6SGOiVVNIiMJ%2FG3p5SaFjysP8UAQNNQD%2Bbg2RfcjC18ZZDxFS20k%2Fvtik5i6Pt1rtrziODYv1f%2FXpigm8DMVArXT5PlsMIPMAPoO7J3i8HUo3H6PqM0ornRh3mcIg%2BIMsEJvZfLyHkeRTlhZppb8aafoqmXpl3dP%2BkvlmL6HpB1tfvbkTopoQEub9EiKu%2BMc%2FAQwgi8tuKclk2yWuhkcch9Fq6FLDem%2F86pnrBWqz2FpsfP6J2cyqrmVLY%3D%7Ctkp%3ABk9SR5iHroKTZA"
Price: 13.8
Details: "Brand new"
Seller: "youwinner-888 (1,451) 99.2%"
Shipping: "N/A"
Location: "N/A"
Sold: false```
