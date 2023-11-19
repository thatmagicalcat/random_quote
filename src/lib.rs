use reqwest::blocking::get;
use scraper::{Html, Selector};

const URL: &str = "https://zenquotes.io";

#[derive(Debug, Clone)]
pub struct Quote {
    /// Quote
    pub quote: String,

    /// Name of the author
    pub author: String,
}

impl std::fmt::Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} - {}", self.quote, self.author)
    }
}

/// Pull a random quote from https://zenquotes.io
pub fn get_quote() -> Result<Quote, Box<dyn std::error::Error>> {
    let body = get(URL)?.text()?;
    let document = Html::parse_document(&body);

    let quote_selector = Selector::parse("div#carousel-quote-1 > h1")?;
    let author_selector = Selector::parse("div#carousel-quote-1 > p")?;

    let quote_element = document.select(&quote_selector).next().unwrap();
    let author_element = document.select(&author_selector).next().unwrap();

    let quote_text = quote_element.text().next().unwrap();
    let quote_text = &quote_text[5..quote_text.len() - 5];
    let author_name = author_element.text().next().unwrap();

    Ok(Quote {
        quote: quote_text.to_owned(),
        author: author_name.to_owned()
    })
}

#[cfg(test)]
mod tests {
    use super::get_quote;

    #[test]
    fn basic_test() {
        println!("quote: {}", get_quote().unwrap());
    }
}
