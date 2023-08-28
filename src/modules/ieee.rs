use serde::Deserialize;

use super::base::NormalizedData;


#[derive(Debug, Deserialize)]
pub struct IEEEEntry {
    #[serde(rename="Authors")]
    authors: String,
    #[serde(rename="Document Title")]
    title: String,
    #[serde(rename="Abstract")]
    abstract_: String,
}

impl Into <NormalizedData>for IEEEEntry {
    fn into(self) -> NormalizedData {
        NormalizedData { abstract_: self.abstract_, title: self.title, authors: self.authors }
    }
}