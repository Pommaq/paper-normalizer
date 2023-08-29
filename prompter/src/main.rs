use clap::Parser;
use colored::Colorize;
use entities::ResultEntry;
use std::{env, str::FromStr};
use strum::{EnumString, EnumVariantNames, IntoStaticStr, VariantNames};

#[derive(IntoStaticStr, EnumVariantNames, EnumString)]
enum Action {
    Accept,
    Maybe,
    Deny,
}

fn prompt_entry(entry: &ResultEntry, titles: bool, abstracts: bool) -> Action {
    if titles {
        println!("Title:\n{}", entry.title.green());
    }
    println!("Authors:\n{}", entry.authors.cyan());
    println!("URL:\n{}", entry.url.blue());
    if abstracts {
        println!("Abstract:\n{}", entry.abstract_.green());
    }
    let choice = inquire::Select::new(
        "Please choose what to do with it",
        Action::VARIANTS.to_vec(),
    )
    .prompt()
    .unwrap();
    Action::from_str(choice).unwrap()
}

#[derive(Parser)]
struct Arguments {
    #[clap(long, short)]
    /// Prompt for titles
    pub titles: bool,

    #[clap(long, short)]
    /// Prompt for abstracts
    pub abstracts: bool,

    pub input_file: String,

    /// Names to whom entries we want to be prompted by are allocated
    pub names: Vec<String>,
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    }
    env_logger::init();

    let args = Arguments::parse();

    let mut rdr = csv::Reader::from_path(args.input_file).expect("unable to open input file");

    let mut accepted = csv::Writer::from_path("accepted.csv").unwrap();
    let mut denied = csv::Writer::from_path("rejected.csv").unwrap();
    let mut questionable = csv::Writer::from_path("questionable.csv").unwrap();

    for raw_content in rdr.deserialize::<ResultEntry>() {
        let content = raw_content.unwrap();
        if args.names.contains(&content.user) {
            // Prompt if we should keep it
            match prompt_entry(&content, args.titles, args.abstracts) {
                Action::Deny => denied.serialize(content).unwrap(),
                Action::Accept => accepted.serialize(content).unwrap(),
                Action::Maybe => questionable.serialize(content).unwrap(),
            }
        } // else ignore it
    }
}
