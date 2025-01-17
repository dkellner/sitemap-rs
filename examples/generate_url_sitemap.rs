use chrono::{DateTime, FixedOffset, NaiveDate};
use sitemap_rs::url::{ChangeFrequency, Url};
use sitemap_rs::url_set::UrlSet;
use std::path::PathBuf;

fn main() {
    let urls: Vec<Url> = vec![Url::builder(String::from("http://www.example.com/"))
        .last_modified(DateTime::from_utc(
            NaiveDate::from_ymd(2005, 1, 1).and_hms(0, 0, 0),
            FixedOffset::east(0),
        ))
        .change_frequency(ChangeFrequency::Monthly)
        .priority(0.8)
        .build()
        .expect("failed a <url> validation")];

    let url_set: UrlSet = UrlSet::new(urls).expect("failed a <urlset> validation");
    url_set
        .write_to_file(PathBuf::from("./target/url-sitemap.xml"))
        .unwrap();
}
