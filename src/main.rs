use webbrowser;

use std::env;
use std::process;
use url::Url;

const GOOGLE: &'static str = "https://www.google.com/search";

fn open_page(page: Url){
    match webbrowser::open(page.as_str()) {
        Ok(_) => println!("Opening {}", page),
        Err(err) => eprintln!("Encountered {}", err)
    }
}

fn parse_url(args: Vec<String>) -> Url {
    let sliced_args: &[String] = &args[1..];
    let query: String = sliced_args.join(" ");
    let mut new_url: Url = Url::parse(GOOGLE).unwrap();
    new_url.query_pairs_mut().append_pair("q", &query);
    return new_url
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: sk <query string> (e.g. sk weather today in Oslo)");
        println!("No arguments given, aborting..");
        process::exit(0);
    }
    else {
        let parsed_url: Url = parse_url(args);
        open_page(parsed_url);
    }

}