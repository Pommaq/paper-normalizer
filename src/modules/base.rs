use std::fs::File;

use entities::ResultEntry;
use serde::de::DeserializeOwned;

#[derive(serde::Serialize, Debug, Clone)]
pub struct NormalizedData {
    #[serde(rename = "abstract")]
    pub abstract_: String,
    pub title: String,
    pub authors: String,
    pub url: String,
    pub doi: String,
}

impl NormalizedData {
    pub fn new(abstract_: &str, title: &str, authors: &str, url: &str, doi: &str) -> Self {
        Self {
            abstract_: abstract_.to_string(),
            title: title.to_string(),
            authors: authors.to_string(),
            url: url.to_string(),
            doi: doi.to_string(),
        }
    }
}

pub struct CSVSource<T: DeserializeOwned + Into<NormalizedData>> {
    inner: Box<dyn Iterator<Item = T>>,
}
impl<T: DeserializeOwned + Into<NormalizedData>> Iterator for CSVSource<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<T: DeserializeOwned + Into<NormalizedData> + 'static> CSVSource<T> {
    pub fn from_file(source: File) -> Self {
        let rdr = csv::Reader::from_reader(source);
        let inner = rdr.into_deserialize().filter_map(|p| p.ok());
        Self {
            inner: Box::new(inner),
        }
    }
}

impl From<ResultEntry> for NormalizedData {
    fn from(value: ResultEntry) -> Self {
        Self {
            abstract_: value.abstract_,
            title: value.title,
            authors: value.authors,
            url: value.url,
            doi: value.doi,
        }
    }
}
