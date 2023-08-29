use serde::Deserialize;

use super::base::NormalizedData;


#[derive(Deserialize, Debug)]
pub struct SpringerEntry {
    #[serde(rename="Item Title")]
    title: String,
    #[serde(rename="Authors")]
    authors: String,
    #[serde(rename="URL")]
    url: String,
}

impl Into <NormalizedData>for SpringerEntry {
    fn into(self) -> NormalizedData {
        NormalizedData { abstract_: "None given through the dumped data, visit the link".to_string(), title: self.title, authors: self.authors, url: self.url }
    }
}