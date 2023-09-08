use url::Url;

pub async fn fetch(url: Url) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .get(url)
        .header("accept", "application/json")
        .header("accept-encoding", "gzip, deflate, br")
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36")
        .header("referer", "https://www.ah.nl/producten/bakkerij-en-banket")
        .send()
        .await
}
