use serde::Deserialize;

use super::base::NormalizedData;

#[derive(Deserialize, Debug)]
pub struct SpringerEntry {
    #[serde(rename = "Item Title")]
    title: String,
    #[serde(rename = "Authors")]
    authors: String,
    #[serde(rename = "URL")]
    url: String,
    #[serde(rename = "Item DOI")]
    doi: String,
}

impl From<SpringerEntry> for NormalizedData {
    fn from(val: SpringerEntry) -> Self {
        NormalizedData {
            abstract_: "None given through the dumped data, visit the link".to_string(),
            title: val.title,
            authors: val.authors,
            url: val.url,
            doi: val.doi,
        }
    }
}
