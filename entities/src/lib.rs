use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ResultEntry {
    pub user: String,
    // Implementation is dumb
    // 'unable to write contents to output: Error(Serialize("serializing maps is not supported, if you have a use case, please file an issue at https://github.com/BurntSushi/rust-csv"))'
    //#[serde(flatten)]
    //pub inner: NormalizedData,
    #[serde(rename="abstract")]
    pub abstract_: String,
    pub title: String,
    pub authors: String,
    pub url: String,
}
pub fn write_csv_file(path: &str, content: Vec<ResultEntry>) -> Result<(), csv::Error> {
    let mut writer = csv::Writer::from_path(path)?;
    for entry in content {
        writer.serialize(entry)?;
    }

    Ok(())
}