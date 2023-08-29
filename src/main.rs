use std::{fs::File, env};

use clap::Parser;
use ::entities::{ResultEntry, write_csv_file};
use log::{info, debug};
use modules::{base::NormalizedData, springer::SpringerEntry};

use crate::modules::ieee::IEEEEntry;

pub mod entities;
pub mod modules;

type Handler = fn(File) -> Vec<NormalizedData>;

/// Allocates names to entries and writes them to target file
fn allocate_results<T: AsRef<str> + Into<String>>(names:&[T], content: Vec<NormalizedData>) -> Vec<ResultEntry>{
    let chunksize = content.len() / names.len() +1 ;
    debug!("chunksize is {}", chunksize);

    let mut entries = vec![];
    for (contents ,person) in content.chunks(chunksize).zip(names.iter()) {
        info!("allocating {} entries to {}", contents.len(), person.as_ref().to_string());

        for content in contents {
            entries.push(ResultEntry {
                user: person.as_ref().to_string(), abstract_: content.abstract_.to_string(), title: content.title.to_string(), authors: content.authors.to_string(), url: content.url.to_string()
            });
        }
    }
    debug!("Got {} entries total", entries.len());

    entries
}

#[derive(Parser)]
#[command(author, version)]
/// A tool for quickly filtering papers
struct Arguments {
    pub output_file: String,
    pub users: Vec<String>,
    #[clap(long)]
    pub ieee_source: Option<String>,
    #[clap(long)]
    pub springer_source: Option<String>,
}
fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    }
    env_logger::init();

    let args = Arguments::parse();

    let from_ieee_source : Handler = |source: File| {
        let reader = modules::base::CSVSource::<_, IEEEEntry>::new(source);
        reader.collect::<Vec<NormalizedData>>()
    };
    let from_springer_source: Handler = |source: File | {
        let reader = modules::base::CSVSource::<_, SpringerEntry>::new(source);
        reader.collect::<Vec<NormalizedData>>()
    };

    let mut papers = vec![];
    let handlers : &[(Option<String>, Handler)]= &[(args.ieee_source, from_ieee_source), (args.springer_source, from_springer_source)];

    for (source, handler) in handlers{
        if let Some(filename) = source {
            info!("Reading content from IEEE source {}", filename);
            let file = File::open(filename).expect("Unable to open file");
            let mut content = handler(file);
            debug!("Read {} entries", content.len());
            papers.append(&mut content);
        }
    
    }
    debug!("has total of {} papers", papers.len());

    let contents = allocate_results( &args.users, papers);

    write_csv_file(&args.output_file, contents).expect("unable to write contents to output")

}
