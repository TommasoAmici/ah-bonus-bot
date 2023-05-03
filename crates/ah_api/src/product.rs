use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductResponse {
    pub card: Card,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: i64,
    pub products: Vec<Product>,
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Control {
    pub theme: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyIcon {
    pub name: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub height: i64,
    pub width: i64,
    pub title: String,
    pub url: url::Url,
    pub ratio: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shield {
    pub theme: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub unit_info: Option<UnitInfo>,
    pub theme: Option<String>,
    pub now: f64,
    pub was: Option<f64>,
    pub unit_size: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitInfo {
    pub price: f64,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Discount {
    pub bonus_type: String,
    pub segment_type: String,
    pub promotion_type: String,
    pub theme: String,
    pub start_date: String,
    pub end_date: String,
    pub tiered_offer: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Taxonomy {
    pub id: i64,
    pub name: String,
    pub image_site_target: Option<String>,
    pub images: Vec<Value>,
    pub shown: bool,
    pub level: i64,
    pub sort_sequence: i64,
    pub parent_ids: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    pub lifestyle: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub gln: String,
    pub gtin: String,
    pub description: Description,
    pub nutritions: Option<Vec<Nutrition>>,
    pub contents: Contents,
    pub ingredients: Option<Ingredients>,
    pub storage: Option<Storage>,
    pub origin: Option<Origin>,
    pub contact: Contact,
    pub resources: Resources,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub descriptions: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nutrition {
    pub nutrients: Vec<Nutrient>,
    pub additional_info: Vec<Value>,
    pub daily_value_intake_reference: String,
    pub preparation_state: String,
    pub basis_quantity: String,
    pub basis_quantity_description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nutrient {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
    pub daily_value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    pub net_contents: Vec<String>,
    pub serving_size: String,
    pub servings_per_package: String,
    pub e_mark: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingredients {
    pub allergens: Option<Allergens>,
    pub statement: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Allergens {
    pub list: Vec<Value>,
    pub contains: Vec<Value>,
    pub may_contain: Vec<String>,
    pub free_from: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Storage {
    pub instructions: Vec<String>,
    pub life_span: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Origin {
    pub provenance: Vec<String>,
    pub activities: Activities,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activities {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub name: Vec<String>,
    pub address: Vec<String>,
    pub communication_channels: Vec<CommunicationChannel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunicationChannel {
    #[serde(rename = "type")]
    pub type_field: String, // TODO: enum "TELEPHONE" | "EMAIL"
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resources {
    pub attachments: Vec<Attachment>,
    pub icons: Vec<Icon>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub format: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: url::Url,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub id: i64,
    pub control: Control,
    pub title: String,
    pub link: String,
    pub available_online: bool,
    pub orderable: bool,
    pub property_icons: Vec<PropertyIcon>,
    pub images: Vec<Image>,
    pub shield: Option<Shield>,
    pub price: Price,
    pub discount: Option<Discount>,
    pub item_catalog_id: i64,
    pub brand: String,
    pub category: String,
    pub theme: String,
    pub hq_id: i64,
    pub gtins: Vec<i64>,
    pub summary: String,
    pub description_full: String,
    pub taxonomy_id: i64,
    pub taxonomies: Vec<Taxonomy>,
    pub contribution_margin: Option<i64>,
    pub properties: Properties,
}

impl Product {
    /// Returns `true` if the product is on discount
    pub fn is_on_discount(&self) -> bool {
        self.discount.is_some()
    }

    pub fn get_discount(&self) -> Option<&String> {
        match &self.shield {
            Some(shield) => Some(&shield.text),
            None => None,
        }
    }

    /// Returns the price of the product in cents so it can be stored in the database
    /// as an integer
    pub fn get_price_for_db(&self) -> u32 {
        (&self.price.now * 100.0) as u32
    }
}

pub async fn get_product(product_id: &str) -> Result<ProductResponse, reqwest::Error> {
    let url = format!(
        "https://www.ah.nl/zoeken/api/products/product?webshopId={}",
        product_id
    );
    log::info!("Fetching product: {}", url);
    reqwest::get(url).await?.json::<ProductResponse>().await
}
