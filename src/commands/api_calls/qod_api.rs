use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct QuoteJson {
    quote: String,
    length: String,
    author: String,
    tags: Vec<String>,
    category: String,
    language: String,
    date: String,
    permalink: String,
    id: String,
    background: String,
    title: String,
}

#[derive(Debug, Deserialize)]
struct OuterJson {
    succsdf: HashMap<String, u8>,
    contents: HashMap<String, Vec<QuoteJson>>,
    baseurl: String,
    copyright: HashMap<String, String>,
}

impl OuterJson {
    fn get_quote(&self) -> Option<(String, String)> {
        match self.contents.len() {
            0 => return None, //No quotes
            _ => {
                return Some((
                    self.contents.get("quotes").unwrap()[0].quote.clone(),
                    self.contents.get("quotes").unwrap()[0].author.clone(),
                ))
            }
        }
    }
}

pub async fn quote_of_the_day() -> Result<Box<(String, String)>, String> {
    let client = reqwest::Client::new();
    match client
        .get("https://quotes.rest/qod?language=en")
        .header("accept", "application/json")
        .send()
        .await
    {
        Ok(resp) => {
            // println!("{}", resp.text().await.unwrap());
            // return Err("Request Failed".to_string());
            match resp.json::<OuterJson>().await {
                Ok(resp) => match resp.get_quote() {
                    Some((quote, author)) => return Ok(Box::new((quote, author))),
                    None => return Err("Could find quote in response".to_string()),
                },
                Err(e) => {
                    println!("{}", e);
                    return Err("Sorry no quotes for today".to_string());
                }
            }
        }
        Err(_) => return Err("Request Failed".to_string()),
    }
}
