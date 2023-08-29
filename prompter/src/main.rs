use std::{env, str::FromStr};

use clap::Parser;
use entities::ResultEntry;
use strum::{EnumString, EnumVariantNames, IntoStaticStr, VariantNames};

#[derive(IntoStaticStr, EnumVariantNames, EnumString)]
enum Action {
    Deny,
    Accept,
    Maybe,
}

fn prompt_entry(entry: &ResultEntry, titles: bool, abstracts: bool) -> Action {
    let choices = Action::VARIANTS;
    if titles {
        println!("Title:\n{}", entry.title);
    }
    println!("Authors:\n{}", entry.authors);
    println!("URL:\n{}", entry.url);
    if abstracts {
        println!("Abstract:\n{}", entry.abstract_);
    }
    let choice = inquire::Select::new("Please choose what to do with it", choices.to_vec())
        .prompt()
        .unwrap();
    Action::from_str(choice).unwrap()
}

#[derive(Parser)]
struct Arguments {
    #[arg(group = "mode")]
    #[clap(long, short)]
    /// Prompt for titles
    pub titles: bool,

    #[arg(group = "mode")]
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
