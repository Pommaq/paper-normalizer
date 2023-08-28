pub mod ieee;
pub mod springer;

pub mod base {
    use std::io::Read;

    use serde::de::DeserializeOwned;

    #[derive(serde::Serialize, Debug, Clone)]
    pub struct NormalizedData{
        #[serde(rename="abstract")]
        pub abstract_: String,
        pub title: String,
        pub authors: String,
    }
    
    impl NormalizedData {
        pub fn new(abstract_: &str, title: &str, authors:&str) -> Self {
            Self { abstract_: abstract_.to_string(), title: title.to_string(), authors: authors.to_string() }
        }
    }
    
    pub struct CSVSource<S:Read, T: DeserializeOwned + Into<NormalizedData>> {
        inner: csv::DeserializeRecordsIntoIter<S, T>
    }
    impl <S:Read , T: DeserializeOwned + Into<NormalizedData>>Iterator for CSVSource< S, T> {
        type Item = NormalizedData;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next().map_or(None, |res| {
                match res {
                    Ok(data) => {
                        Some(data.into())
                    },
                    Err(err) => {log::error!("{}", err);
                None},
                }
            })
        }
    }

    impl <S:Read , T: DeserializeOwned + Into<NormalizedData>>CSVSource< S, T> {
        pub fn new(source: S) -> Self{
            let rdr = csv::Reader::from_reader(source);
            let inner = rdr.into_deserialize();
            Self {
                inner
            }
        }

    }

}
