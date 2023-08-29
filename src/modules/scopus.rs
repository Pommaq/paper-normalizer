use serde::Deserialize;

use super::base::NormalizedData;

#[derive(Deserialize, Debug)]
pub struct ScopusEntry {
    #[serde(rename = "Title")]
    title: String,
    #[serde(rename = "Authors")]
    authors: String,
    #[serde(rename = "Link")]
    url: String,

    #[serde(rename = "DOI")]
    doi: String,
}

impl From<ScopusEntry> for NormalizedData {
    fn from(val: ScopusEntry) -> Self {
        Self {
            abstract_: "None given through the dumped data, visit the link".to_string(),
            title: val.title,
            authors: val.authors,
            url: val.url,
            doi: val.doi,
        }
    }
}
