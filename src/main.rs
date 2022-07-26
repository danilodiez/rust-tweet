
use std::env;
use exitfailure::ExitFailure;
use reqwest::Error;
use scraper::{Html, Selector};
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("get from: {}", request_url);
    let response = reqwest::get(&request_url).await?;

    println!("respuesta {:?}", response.json().await?);
    Ok(())
}
