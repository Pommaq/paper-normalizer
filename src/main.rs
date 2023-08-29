use std::{cmp::min, collections::HashMap, env, fs::File, time::Instant};

use crate::modules::{ieee::IEEEEntry, scopus::ScopusEntry};
use ::entities::{write_csv_file, ResultEntry};
use clap::Parser;
use log::{debug, info};
use modules::{base::NormalizedData, springer::SpringerEntry};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub mod modules;

type Handler = fn(File) -> Vec<NormalizedData>;

/// Allocates names to entries and writes them to target file
fn allocate_results<T: AsRef<str> + Into<String>>(
    names: &[T],
    content: &[NormalizedData],
) -> Vec<ResultEntry> {
    let entries: Vec<ResultEntry> = content
        .iter()
        .zip(names.iter().cycle())
        .map(|(content, person)| ResultEntry {
            user: person.as_ref().to_string(),
            abstract_: content.abstract_.to_string(),
            title: content.title.to_string(),
            authors: content.authors.to_string(),
            url: content.url.to_string(),
            doi: content.doi.to_string(),
        })
        .collect();
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
    // Path to a CSV file of data dumped from IEEE Xplore
    pub ieee_source: Option<String>,
    #[clap(long)]
    /// Path to a CSV file of data dumped from Springer
    pub springer_source: Option<String>,

    #[clap(long)]
    /// Path to a CSV file of data dumped from Scopus
    pub scopus_source: Option<String>,

    #[clap(short, long)]
    /// Remove duplicate articles based on DOI
    pub deduplicate: bool,

    #[clap(short, long)]
    /// Limit the number of entries collected
    pub limit: Option<usize>,
}
fn main() {
    let start = Instant::now();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();

    let args = Arguments::parse();

    let from_ieee_source: Handler = |source: File| {
        let reader = modules::base::CSVSource::<IEEEEntry>::from_file(source);
        reader.map(|p| p.into()).collect::<Vec<NormalizedData>>()
    };
    let from_springer_source: Handler = |source: File| {
        let reader = modules::base::CSVSource::<SpringerEntry>::from_file(source);
        reader.map(|p| p.into()).collect::<Vec<NormalizedData>>()
    };

    let from_scopus_source: Handler = |source: File| {
        let reader = modules::base::CSVSource::<ScopusEntry>::from_file(source);
        reader.map(|p| p.into()).collect::<Vec<NormalizedData>>()
    };

    let handlers: &[(Option<String>, Handler)] = &[
        (args.ieee_source, from_ieee_source),
        (args.springer_source, from_springer_source),
        (args.scopus_source, from_scopus_source),
    ];

    let mut papers: Vec<NormalizedData> = handlers
        .iter()
        .filter(|(filename, _)| filename.is_some())
        .flat_map(|(x, handler)| {
            let filename = x.as_ref().unwrap();
            info!("Reading content from source {}", filename);
            let file = File::open(filename).expect("Unable to open file");
            let content = handler(file);
            debug!("Read {} entries", content.len());
            content.into_iter()
        })
        .collect();

    debug!("has total of {} papers", papers.len());

    if args.deduplicate {
        // Remove duplicates
        info!("About to filter duplicates");
        let old_len = papers.len();

        let mut seen: HashMap<String, ()> = HashMap::new();
        papers.retain(|p| {
            if seen.contains_key(&p.doi) {
                false
            } else {
                seen.insert(p.doi.to_string(), ());
                true
            }
        });

        info!("Filtered {} entries", old_len - papers.len(),);
    }
    // So we dont end up with data from only one of the sources
    papers.shuffle(&mut thread_rng());

    // Respect limits
    let final_papers;
    if let Some(limit) = args.limit {
        final_papers = &papers[0..min(limit, papers.len())];
        info!(
            "Removed {} entries due to set limit",
            papers.len() - final_papers.len()
        );
    } else {
        final_papers = &papers[0..];
    }

    info!("Successfully aggregated {} entries", final_papers.len());
    let contents = allocate_results(&args.users, final_papers);

    write_csv_file(&args.output_file, contents).expect("unable to write contents to output");

    let end = Instant::now();
    info!("Finished after {} milliseconds", (end - start).as_millis());
}
