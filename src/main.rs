
use std::env;
use exitfailure::ExitFailure;
use reqwest::Url;

fn main() {
    // that's how we get the args from the CLI
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    get_tweets("pelota");
}

async fn get_tweets (query: String) -> String {
    let url = format!("https://twitter154.p.rapidapi.com/search/search", query, "8f0c0ea637mshc27624fe5b77293p127451jsn9531dd22a97f");
    let url = Url::parse(&*url)?;
    let res = reqwest::get(url).await?.json().await?;
    return res;
}

