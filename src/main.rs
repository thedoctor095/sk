mod cli;

use url::Url;
use std::process;
use cli::{CLIArgs};
use structopt::StructOpt;

use webbrowser::open as browser;

const GOOGLE: &'static str = "https://www.google.com/search"; //q=top+gun
const DDG: &'static str = "https://duckduckgo.com/"; // ?q=top+gun
const BING: &'static str = "https://www.bing.com/search"; // ?q=top+gun

fn open_page(page: Url) {
    match browser(page.as_str()) {
        Ok(_) => println!("Opening {}", page),
        Err(err) => eprintln!("Encountered {}", err)
    }
}

fn parse_url(engine: &str, search_query: Vec<String>) -> Url {
    let search_query = search_query.join(" ");
    let mut new_url = match Url::parse(engine) {
        Ok(parsed_engine) => parsed_engine,
        Err(err) => {
            eprintln!("Could not parse search engine error: {}", err);
            process::exit(0);
        }
    };
    new_url.query_pairs_mut().append_pair("q", &search_query);
    return new_url
}

fn main() {
    let CLIArgs {
        search_query,
        ddg,
        bg,
        gg
    } = CLIArgs::from_args();

    let engine: &str = if ddg {DDG} else if bg {BING} else if gg {GOOGLE} else {GOOGLE};
    let parsed_url: Url = parse_url(engine, search_query);
    open_page(parsed_url)
}