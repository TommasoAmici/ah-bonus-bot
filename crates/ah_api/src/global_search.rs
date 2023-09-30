use serde::{Deserialize, Serialize};

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
