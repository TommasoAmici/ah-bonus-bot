use serde::{Deserialize, Serialize};
use url::Url;

use crate::fetch::fetch;

#[derive(Serialize, Deserialize)]
pub struct Icon {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Link {
    pub href: String,
}

#[derive(Serialize, Deserialize)]
pub struct Suggestion {
    pub label: String,
    pub value: String,
    #[serde(rename = "type")]
    pub result_type: String,
    pub icon: Option<Icon>,
    pub link: Link,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    pub label: String,
    pub value: String,
    #[serde(rename = "type")]
    pub result_type: String,
    pub icon: Option<Icon>,
    pub link: Link,
    // alternatives: Vec,
    pub suggestions: Vec<Suggestion>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}

/// Search for a product using the global search endpoint. It's the endpoint used by the
/// main search bar on the AH website.
pub async fn global_search(query: String, limit: u8) -> Result<SearchResponse, reqwest::Error> {
    let base_url = "https://www.ah.nl/features/api/global-search";
    let mut url = Url::parse(base_url).unwrap();
    url.query_pairs_mut()
        .append_pair("query", &query)
        .append_pair("limit", limit.to_string().as_str());
    log::info!("searching: {}", url);

    fetch(url).await?.json::<SearchResponse>().await
}
