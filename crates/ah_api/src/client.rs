use url::Url;

use crate::{global_search::SearchResponse, product::ProductResponse, search::SearchResults};

#[derive(Clone)]
pub struct AHClient {
    client: reqwest::Client,
}

impl AHClient {
    pub async fn new() -> Result<Self, ()> {
        log::info!("Initializing AH client");
        let client = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .build()
            .unwrap();
        let err = client.get("https://www.ah.nl").header("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8").header("accept-encoding", "gzip, deflate, br").header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36").send().await.err();
        if let Some(_err) = err {
            Err(())
        } else {
            Ok(Self { client })
        }
    }

    async fn fetch(&self, url: Url) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .get(url)
            .header("accept", "application/json")
            .header("accept-encoding", "gzip, deflate, br")
            .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36")
            .header("referer", "https://www.ah.nl/producten/bakkerij-en-banket")
            .send()
            .await
    }

    /// Search for a product using the global search endpoint. It's the endpoint used by the
    /// main search bar on the AH website.
    pub async fn global_search(
        &self,
        query: String,
        limit: u8,
    ) -> Result<SearchResponse, reqwest::Error> {
        let base_url = "https://www.ah.nl/features/api/global-search";
        let mut url = Url::parse(base_url).unwrap();
        url.query_pairs_mut()
            .append_pair("query", &query)
            .append_pair("limit", limit.to_string().as_str());
        log::info!("searching: {}", url);

        self.fetch(url).await?.json::<SearchResponse>().await
    }

    pub async fn search_products(
        &self,
        query: &String,
        limit: usize,
    ) -> Result<SearchResults, reqwest::Error> {
        let base_url = "https://www.ah.nl/zoeken/api/products/search";
        let mut url = Url::parse(base_url).unwrap();
        url.query_pairs_mut()
            .append_pair("query", &query)
            .append_pair("size", limit.to_string().as_str());
        log::info!("searching: {}", url);

        self.fetch(url).await?.json::<SearchResults>().await
    }

    pub async fn get_product(&self, product_id: &str) -> Result<ProductResponse, reqwest::Error> {
        let base_url = "https://www.ah.nl/zoeken/api/products/product";
        let mut url = Url::parse(base_url).unwrap();
        url.query_pairs_mut().append_pair("webshopId", product_id);
        log::info!("Fetching product: {}", url);
        self.fetch(url).await?.json::<ProductResponse>().await
    }
}
