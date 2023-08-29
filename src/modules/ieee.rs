use serde::Deserialize;

use super::base::NormalizedData;

#[derive(Debug, Deserialize)]
pub struct IEEEEntry {
    #[serde(rename = "Authors")]
    authors: String,
    #[serde(rename = "Document Title")]
    title: String,
    #[serde(rename = "Abstract")]
    abstract_: String,

    #[serde(rename = "PDF Link")]
    pdf_link: String,
    #[serde(rename = "DOI")]
    doi: String,
}

impl From<IEEEEntry> for NormalizedData {
    fn from(val: IEEEEntry) -> Self {
        NormalizedData {
            abstract_: val.abstract_,
            title: val.title,
            authors: val.authors,
            url: val.pdf_link,
            doi: val.doi,
        }
    }
}
