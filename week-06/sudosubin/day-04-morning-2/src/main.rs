use reqwest::blocking::{get, Response};
use reqwest::Url;
use scraper::{Html, Selector};
use std::sync::mpsc;
use std::thread;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

fn extract_links(response: Response) -> Result<Vec<Url>, Error> {
    let base_url = response.url().to_owned();
    let document = response.text()?;
    let html = Html::parse_document(&document);
    let selector = Selector::parse("a").unwrap();

    let mut valid_urls = Vec::new();

    let (tx, rx) = mpsc::sync_channel(5);

    for element in html.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            let tx = tx.clone();
            let base_url = base_url.clone();
            let href_str = href.to_string();
            thread::spawn(move || match base_url.join(href_str.as_str()) {
                Ok(url) => {
                    tx.send(url).unwrap();
                }
                Err(err) => {
                    println!("On {base_url}: could not parse {href_str:?}: {err} (ignored)",);
                }
            });
        };
    }

    drop(tx);

    for valid_url in rx {
        valid_urls.push(valid_url);
    }

    Ok(valid_urls)
}

fn main() {
    let start_url = Url::parse("https://www.google.org").unwrap();
    let response = get(start_url).unwrap();
    match extract_links(response) {
        Ok(links) => println!("Links: {links:#?}"),
        Err(err) => println!("Could not extract links: {err:#}"),
    }
}
