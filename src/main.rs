use reqwest::Error;
use scraper;
use std::env;

fn main() -> Result<(), Error> {
    let response = reqwest::blocking::get("https://twitter.com/danilodiez").unwrap();
    let document = &response.text().unwrap();
    let doc = scraper::Html::parse_document(&document);
    let body = scraper::Selector::parse(r#"body"#).unwrap();
    let root = scraper::Selector::parse(r#"div"#).unwrap();
    // println!("{:?}", doc);
    // let span_selector = scraper::Selector::parse("div").unwrap();
    // let tweets = doc.select(&body).next().unwrap();
    let mut titles = doc.select(&body);

    // let new_selector = scraper::Selector::parse(r#"span"#).unwrap();
    // println!("{:?}", doc);

    // let mut a_span = titles.next().unwrap().select(&new_selector);
    // for elem in titles {
    //     println!("{:?}", elem.value());
    // }
    let tweets = titles.next().unwrap().select(&root);
    let mut tweets_vector = Vec::new();

    for elem in tweets {
        tweets_vector.push(elem);
    }

    let tweets_div_container = tweets_vector[1];
    let another_div =
        scraper::Selector::parse(r#"div[class="css-1dbjc4n r-18u37iz r-13qz1uu r-417010"]"#)
            .unwrap();

    let mut pre_main_container = tweets_div_container.select(&another_div);

    println!("{:?}", pre_main_container);
    Ok(())
}
