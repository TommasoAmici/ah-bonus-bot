use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

use crate::{fetch::fetch, product};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResults {
    pub cards: Vec<Card>,
    pub page: Page,
    pub aggregation: Aggregation,
    pub taxonomies: Vec<Value>,
    pub query_suggestions: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: i64,
    pub products: Vec<product::Product>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    pub lifestyle: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub size: i64,
    pub total_elements: i64,
    pub total_pages: i64,
    pub number: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggregation {
    pub properties: Vec<Property>,
    pub brands: Vec<Brand>,
    pub taxonomies: Vec<Taxonomy>,
    pub prices: Vec<Price>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub id: String,
    pub label: String,
    pub count: i64,
    pub attributes: Option<Attributes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Brand {
    pub name: String,
    pub count: i64,
    pub id: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Taxonomy {
    pub count: i64,
    pub id: i64,
    pub shown: bool,
    pub level: i64,
    pub parent_ids: Vec<i64>,
    pub rank: i64,
    pub relevant: bool,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub count: i64,
    pub min: f64,
    pub max: f64,
    pub label: String,
}

pub async fn search_products(
    query: &String,
    limit: usize,
) -> Result<SearchResults, reqwest::Error> {
    let base_url = "https://www.ah.nl/zoeken/api/products/search";
    let mut url = Url::parse(base_url).unwrap();
    url.query_pairs_mut()
        .append_pair("query", &query)
        .append_pair("size", limit.to_string().as_str());
    log::info!("searching: {}", url);

    fetch(url).await?.json::<SearchResults>().await
}
